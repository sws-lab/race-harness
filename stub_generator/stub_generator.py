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
import logging
from typing import Collection, List, Callable, Optional
import clang.cindex as cindex
from compile_db.compilation_database import CompilationDatabase, BuildTargetType
from undefined_scanner import UndefinedReferenceScanner, UndefinedReferenceScannerProfile

class StubGeneratorError(BaseException): pass

@dataclasses.dataclass
class StubGeneratorProfile:
    scanner_profile: UndefinedReferenceScannerProfile
    compiler_command_line_regex: str
    remove_include_path_prefixes: Collection[str]
    remove_includes: Collection[str]

    @staticmethod
    def linux_kernel() -> 'StubGeneratorProfile':
        return StubGeneratorProfile(
            scanner_profile=UndefinedReferenceScannerProfile.linux_kernel(),
            compiler_command_line_regex='^\/\/\s*CLANG\s+COMMAND\s+LINE\s*:(.*)$',
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
    def __init__(self, db: CompilationDatabase, build_id: str, profile: StubGeneratorProfile, include_symbols: Optional[Callable[[cindex.Cursor], bool]]):
        self._db = db
        self._build = db.kernel_build(build_id)
        self._profile = profile
        if self._build is None:
            raise StubGeneratorError(f'Unable to find kernel build {build_id}')
        self._scanner = UndefinedReferenceScanner(profile=profile.scanner_profile, include_symbols=include_symbols)
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
            self._compiler_command_line_sample = target.tool_args.copy()
        workdir = os.getcwd()
        try:
            os.chdir(self._build.path)
            self._scanner.load(path=None, args=target.tool_args)
        finally:
            os.chdir(workdir)

    @functools.cached_property
    def _compiler_command_line_regex(self) -> re.Pattern:
        return re.compile(self._profile.compiler_command_line_regex)

    def _scan_local_source_file(self, local_path: pathlib.Path):
        local_path = local_path.absolute()
        cmdline = None
        with open(local_path, 'r') as local_file:
            for line in local_file:
                match = self._compiler_command_line_regex.match(line)
                if match:
                    cmdline = shlex.split(match[1])
        if cmdline is None:
            raise StubGeneratorError(f'Unable to find compiler command line in {local_path}')

        if self._compiler_command_line_sample is None:
            self._compiler_command_line_sample = cmdline.copy()

        workdir = os.getcwd()
        try:
            os.chdir(self._build.path)
            self._scanner.load(path=str(local_path), args=cmdline)
        finally:
            os.chdir(workdir)

    @functools.cached_property
    def _removed_include_paths(self) -> Collection[pathlib.Path]:
        return set(map(lambda x: pathlib.Path(x), self._profile.remove_includes))

    def _process_include_path(self, include_filepath: str) -> pathlib.Path:
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
        if ';' in cmdline:
            semicolon_index = cmdline.index(';')
            cmdline = cmdline[:semicolon_index]
        
        if '-o' in cmdline:
            output_index = cmdline.index('-o')
            del cmdline[output_index + 1]
            del cmdline[output_index]
        
        return [
            '-I.',
            *(arg
            for arg in cmdline
            if not arg.endswith('.c') and arg != '-c')
        ]
    
    def _undefined_variables(self, blacklist: Optional[Callable[[cindex.Cursor], bool]]):
        for node in self._scanner.undefined_variables():
            if blacklist is None or not blacklist(node):
                yield node
    
    def _undefined_functions(self, blacklist: Optional[Callable[[cindex.Cursor], bool]]):
        for node in self._scanner.undefined_functions():
            if blacklist is None or not blacklist(node):
                yield node

    def _has_undefined(self, blacklist: Optional[Callable[[cindex.Cursor], bool]]) -> bool:
        for _ in self._undefined_variables(blacklist):
            return True
        for _ in self._undefined_functions(blacklist):
            return True
        return False
    
    def _generate_node_stub(self, stubs: io.StringIO, node: cindex.Cursor):
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

    def _generate_stubs_header(self, stubs: io.StringIO):
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

    def _generate_stubs_includes(self, stubs: io.StringIO):
        has_includes = False
        for include in self._scanner.includes():
            include_path = self._process_include_path(include)
            if include_path is not None:
                stubs.write(f'#include "{include_path}"\n')
                has_includes = True
        if has_includes:
            stubs.write('\n')

    def _generate_stubs_footer(self, stubs: io.StringIO):
        stubs.write('''
int main(void) {
    init_module();
    // Fill in the code to meaningfully drive module execution
    cleanup_module();
    return 0;
}
''')

    def generate_stubs(self, blacklist: Optional[Callable[[cindex.Cursor], bool]]) -> str:
        stubs = io.StringIO()

        self._generate_stubs_header(stubs)
        if self._has_undefined(blacklist):
            self._generate_stubs_includes(stubs)
            for node in self._undefined_variables(blacklist):
                self._generate_node_stub(stubs, node)
            for node in self._undefined_functions(blacklist):
                self._generate_node_stub(stubs, node)
        self._generate_stubs_footer(stubs)

        return stubs.getvalue()

if __name__ == '__main__':
    prog_basename = os.path.basename(__file__)
    parser = argparse.ArgumentParser(prog=prog_basename, description='Stub generator for Linux kernel module external dependencies')
    parser.add_argument('--db', type=str, required=True, help='Kernel compilation database path')
    parser.add_argument('--build-id', type=str, required=False, help='Kernel build identifier')
    parser.add_argument('--include', action='append', help='Regular expressions for symbols included into scan list')
    parser.add_argument('--blacklist', action='append', help='Regular expressions for blacklisted symbols')
    parser.add_argument('--quiet', action='store_true', default=False, help='Suppress all logging')
    parser.add_argument('input', nargs='+', help='List of kernel objects and external stub files')
    args = parser.parse_args(sys.argv[1:])

    if args.blacklist:
        blacklist_regexps = [
            re.compile(entry)
            for entry in args.blacklist
        ]
    else:
        blacklist_regexps = list()

    if args.include:
        include_regexps = [
            re.compile(entry)
            for entry in args.include
        ]
    else:
        include_regexps = list()

    def blacklist_callback(node: cindex.Cursor):
        return any(
            entry.match(node.spelling) for entry in blacklist_regexps
        )
    
    def include_callback(node: cindex.Cursor):
        return any(
            entry.match(node.spelling) for entry in include_regexps
        )
        
    
    logger = logging.Logger(prog_basename)
    if not args.quiet:
        logger.addHandler(logging.StreamHandler(sys.stderr))

    with CompilationDatabase(db_filepath=args.db) as db:
        logger.info('Using kernel build database %s', db.db_filepath)
        if args.build_id is not None:
            build = db.kernel_build(args.build_id)
            if build is None:
                logger.error('Unable to find requested build %s in kernel build database %s', args.build_id, db.db_filepath)
                sys.exit(-1)
        else:
            build = db.latest_kernel_build()
            if build is None:
                logger.error('Kernel build database %s does not contain any builds', db.db_filepath)
                sys.exit(-1)
        logger.info('Using kernel build %s', build.identifier)
            
        stub_generator = StubGenerator(db, build.identifier, StubGeneratorProfile.linux_kernel(), include_callback)
        for input in args.input:
            stub_generator.load(input)
        print(stub_generator.generate_stubs(blacklist_callback))
