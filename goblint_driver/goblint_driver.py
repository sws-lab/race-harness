#!/usr/bin/env python3
import sys
import os
import logging
import argparse
import pathlib
import shlex
import tempfile
import subprocess
import json
from typing import Optional, Iterable, List, Tuple
from compile_db.compilation_database import CompilationDatabase, KernelBuild, BuildTarget, BuildTargetType

class GoblintDriverException(BaseException): pass

def resolve_build(db: CompilationDatabase, build_id: Optional[str]) -> KernelBuild:
    if build_id is not None:
        build = db.kernel_build(build_id)
        if build is None:
            raise GoblintDriverException(f'Unable to find requested kernel build {build_id} in database {db.db_filepath}')
    else:
        build = db.latest_kernel_build()
        if build is None:
            raise GoblintDriverException(f'Kernel compilation database {db.db_filepath} is empty')
    return build

def resolve_inputs(db: CompilationDatabase, build: KernelBuild, inputs: Iterable[str]) -> Iterable[pathlib.Path]:
    for input in inputs:
        target = db.build_target(build_id=build.identifier, target=input)
        if target is not None:
            for kernel_filepath in db.target_base_dependencies(build.identifier, target.target):
                yield build.path / kernel_filepath
        else:
            local_path = pathlib.Path(input)
            kernel_path = build.path / input
            if local_path.exists():
                yield local_path
            elif kernel_path.exists():
                yield kernel_path
            else:
                raise GoblintDriverException(f'Unable to find input file input {input}')
            
class GoblintDriver:
    def __init__(self, db: CompilationDatabase, goblint_filepath: str, goblint_extra_args: Optional[List[str]], logger: logging.Logger):
        self._db = db
        self._goblint_filepath = goblint_filepath
        self._goblint_extra_args = goblint_extra_args.copy() if goblint_extra_args else list()
        self._logger = logger

    def __call__(self, *args, **kwargs):
        return self.run(*args, **kwargs)

    def run(self, build: KernelBuild, inputs: Iterable[str]):
        resolved_inputs = list(self._resolve_inputs(build, inputs))
        inputs = [
            str(input.absolute())
            for _, input in resolved_inputs
        ]
        compile_cmdline = self._determine_command_line(
            target
            for target, _ in resolved_inputs
            if target is not None
        )
        inlcude_paths = self._find_include_paths(build, compile_cmdline)

        with tempfile.NamedTemporaryFile(mode='w', suffix='.json') as goblint_conf:
            goblint_conf_content = json.dumps({
                'kernel': True,
                'kernel_use_main': True,
                'pre': {
                    'kernel-root': str(build.path.absolute())
                }
            }, indent=2)
            goblint_conf.write(goblint_conf_content)
            goblint_conf.flush()

            self._logger.info('Starting Goblint with configuration %s on %s', goblint_conf_content, inputs)
            goblint = subprocess.Popen(
                executable=self._goblint_filepath,
                args=[self._goblint_filepath, '--conf', goblint_conf.name, '--conf', '/home/jprotopopov/goblint/analyzer/conf/examples/very-precise.json', *self._goblint_extra_args, *inputs],
                stdin=subprocess.DEVNULL,
                shell=False
            )
            goblint.wait()

    def _resolve_inputs(self, build: KernelBuild, inputs: Iterable[str]) -> Iterable[Tuple[Optional[BuildTarget], pathlib.Path]]:
        for input in inputs:
            target = self._db.build_target(build_id=build.identifier, target=input)
            if target is not None:
                for kernel_filepath in self._db.target_base_dependencies(build.identifier, target.target):
                    yield self._db.find_target_for(build.identifier, kernel_filepath), build.path / kernel_filepath
            else:
                local_path = pathlib.Path(input)
                kernel_path = build.path / input
                if local_path.exists():
                    yield None, local_path
                elif kernel_path.exists():
                    yield None, kernel_path
                else:
                    raise GoblintDriverException(f'Unable to find input file input {input}')
                
    def _determine_command_line(self, targets: Iterable[BuildTarget]) -> List[str]:
        for target in targets:
            if target.type == BuildTargetType.CompiledObject:
                return target.tool_args
        logger.warning(f'Unable to determine kernel target compilation command line; make sure to --goblint-args arguments to correctly configure Goblint')
        return []
    
    def _find_include_paths(self, build: KernelBuild, cmdline: Iterable[str]) -> Iterable[pathlib.Path]:
        yield build.path

        yield_next_arg = False
        for argument in cmdline:
            if yield_next_arg:
                yield build.path / argument
                yield_next_arg = False
            if argument == '-I':
                yield_next_arg = True
            elif argument.startswith('-I'):
                yield build.path / argument[2:]

if __name__ == '__main__':
    prog_basename = os.path.basename(__file__)
    parser = argparse.ArgumentParser(prog=prog_basename, description='Goblint driver for Linux kernel module verification')
    parser.add_argument('--db', type=str, required=True, help='Kernel compilation database path')
    parser.add_argument('--build-id', type=str, required=False, help='Kernel build identifier')
    parser.add_argument('--goblint', type=str, required=True, help='Goblint executable path')
    parser.add_argument('--goblint-args', type=str, default='', required=False, help='Extra arguments passed to Goblint')
    parser.add_argument('--quiet', action='store_true', help='Suppress all logging')
    parser.add_argument('input', nargs='+', help='List of kernel objects and external stub files')
    args = parser.parse_args(sys.argv[1:])

    logger = logging.Logger(prog_basename)
    if not args.quiet:
        logger.addHandler(logging.StreamHandler(sys.stderr))
    
    with CompilationDatabase(args.db) as db:
        try:
            build = resolve_build(db, args.build_id)
            goblint = GoblintDriver(db, args.goblint, shlex.split(args.goblint_args), logger)
            goblint(build, args.input)
        except GoblintDriverException as ex:
            logger.error(str(ex))
            sys.exit(-1)
    
