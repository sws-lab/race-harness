#!/usr/bin/env python3
import sys
import argparse
import pathlib
import os
import io
import re
import shlex
import dataclasses
import functools
from typing import Collection, List
import clang.cindex as cindex
from compile_db.compilation_database import CompilationDatabase, BuildTargetType
from undefined_scanner import UndefinedReferenceScanner, UndefinedReferenceScannerProfile

class StubGeneratorError(Exception): pass

@dataclasses.dataclass
class StubGeneratorProfile:
    scanner_profile: UndefinedReferenceScannerProfile
    remove_include_path_prefixes: Collection[str]
    remove_includes: Collection[str]

    @staticmethod
    def linux_kernel() -> 'StubGeneratorProfile':
        return StubGeneratorProfile(
            scanner_profile=UndefinedReferenceScannerProfile.linux_kernel(),
            remove_include_path_prefixes=[
                'include',
                'arch/*/include'
            ],
            remove_includes=[
                'linux/compiler-version.h',
                'linux/vermagic.h'
            ]
        )

class StubGenerator:
    LOCAL_FILE_CMD_LINE_REGEXP = re.compile('^\/\/\s*CLANG\s+COMMAND\s+LINE\s*:(.*)$')

    def __init__(self, db: CompilationDatabase, build_id: str, profile: StubGeneratorProfile):
        self._db = db
        self._build = db.kernel_build(build_id)
        self._profile = profile
        if self._build is None:
            raise StubGeneratorError(f'Unable to find kernel build {build_id}')
        self._scanner = UndefinedReferenceScanner(profile=profile.scanner_profile)
        self._compiler_command_line_sample = None

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
        if self._compiler_command_line_sample is None:
            self._compiler_command_line_sample = target.tool_args
        workdir = os.getcwd()
        try:
            os.chdir(self._build.path)
            self._scanner.load(path=None, args=target.tool_args)
        finally:
            os.chdir(workdir)

    def _scan_local_source_file(self, local_path: pathlib.Path):
        local_path = local_path.absolute()
        cmdline = None
        with open(local_path, 'r') as local_file:
            for line in local_file:
                match = StubGenerator.LOCAL_FILE_CMD_LINE_REGEXP.match(line)
                if match:
                    cmdline = shlex.split(match[1])
        if cmdline is None:
            raise StubGeneratorError(f'Unable to find compiler command line in {local_path}')

        if self._compiler_command_line_sample is None:
            self._compiler_command_line_sample = cmdline

        workdir = os.getcwd()
        try:
            os.chdir(self._build.path)
            self._scanner.load(path=str(local_path), args=cmdline)
        finally:
            os.chdir(workdir)

    @functools.cached_property
    def _removed_include_paths(self) -> Collection[pathlib.Path]:
        return set(map(lambda x: pathlib.Path(x), self._profile.remove_includes))

    def _process_include_path(self, include_filepath: str):
        def resolve_prefixes():
            include_path = (self._build.path / include_filepath)
            for prefix in self._profile.remove_include_path_prefixes:
                for resolved_prefix in pathlib.Path(self._build.path).glob(prefix):
                    if resolved_prefix in include_path.parents:
                        return include_path.relative_to(resolved_prefix)
            return include_path.relative_to(self._build.path)
        
        include_path = resolve_prefixes()
        if include_path in self._removed_include_paths:
            return None
        else:
            return include_path
        
    def _filter_command_line(self, cmdline: List[str]) -> List[str]:
        semicolon_index = cmdline.index(';')
        if semicolon_index is not None:
            cmdline = cmdline[:semicolon_index]
        
        output_index = cmdline.index('-o')
        if output_index is not None:
            del cmdline[output_index + 1]
            del cmdline[output_index]
        
        return [
            arg
            for arg in cmdline
            if not arg.endswith('.c')
        ]
        

    def generate_stubs(self, blacklist) -> str:
        stubs = io.StringIO()

        stubs.write('''// This is a stub skeleton for Linux kernel module verification.
// The skeleton contains variable and function declarations external to the module.
// Please fill-in appropriate function code and variable values to obtain complete definitions
// for verifier to work on.
//
// Make sure that your stub file compiles correctly in the kernel build directory. Compiler command line and
// include list are preliminary, so feel free to edit the stub as needed.
//
// When added to stub_generator script as an input, complete stub along with respective kernel modules shall
// produce an empty skeleton.
//
''')

        if self._compiler_command_line_sample:
            stubs.write(f'// CLANG COMMAND LINE: {shlex.join(self._filter_command_line(self._compiler_command_line_sample))}\n')
        else:
            stubs.write('// !!! UNABLE TO DETERMINE COMPILER COMMAND LINE !!!\n')
            stubs.write('// !!! PLEASE FILL IN APPROPRIATE COMMAND LINE BELOW !!!\n')
            stubs.write('// CLANG COMMAND LINE: \n')
        
        has_undefined = False
        for node in self._scanner.undefined_variables():
            if blacklist is None or not blacklist(node):
                has_undefined = True
                break
        if not has_undefined:
            for node in self._scanner.undefined_functions():
                if blacklist is None or not blacklist(node):
                    has_undefined = True
                    break

        if has_undefined:
            has_includes = False
            for include in self._scanner.includes():
                include_path = self._process_include_path(include)
                if include_path is not None:
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
        stub_generator = StubGenerator(db, args.build_id, StubGeneratorProfile.linux_kernel())
        for input in args.input:
            stub_generator.load(input)
        print(stub_generator.generate_stubs(blacklist_callback))
