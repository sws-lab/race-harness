#!/usr/bin/env python3
import sys
import argparse
import pathlib
import os
import io
import re
import clang.cindex as cindex
from compile_db.compilation_database import CompilationDatabase, BuildTargetType
from undefined_scanner import UndefinedReferenceScanner

class StubGeneratorError(Exception): pass

class StubGenerator:
    def __init__(self, db: CompilationDatabase, build_id: str):
        self._db = db
        self._build = db.kernel_build(build_id)
        if self._build is None:
            raise StubGeneratorError(f'Unable to find kernel build {build_id}')
        self._scanner = UndefinedReferenceScanner()

    def load(self, input: str):
        kernel_target = self._db.build_target(self._build.identifier, input)
        if kernel_target is not None:
            for base_dep in self._db.target_base_dependencies(kernel_target.build_id, kernel_target.target):
                self._scan_kernel_source_file(base_dep)
        else:
            local_path = pathlib.Path(input)
            kernel_path = self._build.path / input
            if local_path.exists():
                self._scan_local_source_file(local_path)
            elif kernel_path.exists():
                self._scan_kernel_source_file(input)
            else:
                raise StubGeneratorError(f'Unable to find stub generator input {input}')
            
    def _scan_kernel_source_file(self, source_file: str):
        target = self._db.find_target_for(self._build.identifier, source_file)
        if target is None or target.type != BuildTargetType.CompiledObject:
            raise StubGeneratorError(f'Unable to find appropriate kernel build target for {source_file}')
        workdir = os.getcwd()
        try:
            os.chdir(self._build.path)
            self._scanner.load(path=None, args=target.tool_args)
        finally:
            os.chdir(workdir)

    def _scan_local_source_file(self, local_path: pathlib.Path):
        local_path = local_path.absolute()
        workdir = os.getcwd()
        try:
            os.chdir(self._build.path)
            self._scanner.load(path=str(local_path), args=[])
        finally:
            os.chdir(workdir)

    def generate_stubs(self, blacklist) -> str:
        stubs = io.StringIO()

        has_includes = False
        for include in self._scanner.includes():
            include_path = (self._build.path / include).relative_to(self._build.path)
            stubs.write(f'#include "{include_path}"\n')
            has_includes = True
        if has_includes:
            stubs.write('\n')

        def write_node(node: cindex.Cursor):
            location_file = pathlib.Path(str(node.location.file))
            display_location_file = location_file
            if not location_file.is_absolute():
                location_file = (self._build.path / location_file)
                if location_file.exists():
                    display_location_file = location_file.relative_to(self._build.path)
                else:
                    location_file = str(node.location.file)
                    display_location_file = location_file
            stubs.write(f'// {node.spelling} [{display_location_file} line {node.location.line} column {node.location.column}]\n')

            begin_offset = node.extent.start.offset
            end_offset = node.extent.end.offset
            with open(location_file, 'r') as file:
                file.seek(begin_offset)
                stubs.write(file.read(end_offset - begin_offset))

            stubs.write(';\n\n')

        for node in self._scanner.undefined_variables():
            if blacklist is None or not blacklist(node):
                write_node(node)
        for node in self._scanner.undefined_functions():
            if blacklist is None or not blacklist(node):
                write_node(node)
        return stubs.getvalue()

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--db', type=str, required=True, help='Linux kernel compilation database path')
    parser.add_argument('--build-id', type=str, required=True, help='Linux kernel build identifier')
    parser.add_argument('--blacklist', action='append', help='Regular expressions for blacklisted symbols')
    parser.add_argument('input', nargs='+', help='List of kernel objects and external stub files')
    args = parser.parse_args(sys.argv[1:])

    blacklist_regexps = list()
    if args.blacklist:
        for entry in args.blacklist:
            blacklist_regexps.append(re.compile(entry))

    def blacklist_callback(node):
        for entry in blacklist_regexps:
            if entry.match(node.spelling):
                return True
        return False

    with CompilationDatabase(db_filepath=args.db) as db:
        stub_generator = StubGenerator(db, args.build_id)
        for input in args.input:
            stub_generator.load(input)
        print(stub_generator.generate_stubs(blacklist_callback))
