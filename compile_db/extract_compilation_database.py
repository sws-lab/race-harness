#!/usr/bin/env python3
import sys
import pathlib
import argparse
import re
import shlex
import uuid
import logging
import datetime
from typing import Iterable, Optional
from compilation_database import KernelBuild, BuildTarget, BuildTargetType, CompilationDatabase

def discover_cmd_files(root_dir: pathlib.Path) -> Iterable[pathlib.Path]:
    for path in root_dir.iterdir():
        if path.is_file() and path.suffix == '.cmd':
            yield path
        elif path.is_dir():
            yield from discover_cmd_files(path)

class CMDFileParser:
    def __init__(self):
        self._patterns = [
            (re.compile('^savedcmd_?([^\s]+\.o)\s*:=\s*(clang.*)$'), self._parse_compiled_object_file),
            (re.compile('^savedcmd_?([^\s]+\.o)\s*:=\s*(l?ld.*)$'), self._parse_linked_object_file),
            (re.compile('^savedcmd_?([^\s]+\.ko)\s*:=\s*(l?ld.*)$'), self._parse_kernel_module)
        ]
        self._c_source_file_pattern = re.compile('^([^-\s][^\s]*\.c)$')
        self._object_file_pattern = re.compile('^([^-\s][^\s]*\.o)$')
        self._mod_file_pattern = re.compile('^(@[^-\s][^\s]*\.mod)$')

    def parse(self, build_id: str, build_dir: pathlib.Path, filepath: pathlib.Path) -> Optional[BuildTarget]:
        with open(filepath, 'r') as cmd_file:
            for line in cmd_file:
                for pattern, handler in self._patterns:
                    match = pattern.match(line)
                    if match:
                        return handler(build_id, build_dir, filepath.relative_to(build_dir), match)
                    
    def discover_and_parse(self, build_id: str, build_dir: pathlib.Path) -> Iterable[BuildTarget]:
        for filepath in discover_cmd_files(build_dir):
            res = self.parse(build_id, build_dir, filepath)
            if res is not None:
                yield res

    def _parse_compiled_object_file(self, build_id: str, build_dir: pathlib.Path, cmd_filepath: pathlib.Path, match: re.Match) -> BuildTarget:
        target = match[1]
        command = match[2]
        sources = [arg for arg in shlex.split(command) if self._c_source_file_pattern.match(arg) is not None]
        return BuildTarget(build_id=build_id, cmd_filepath=cmd_filepath, type=BuildTargetType.CompiledObject, target=target, command=command, sources=sources)

    def _parse_linked_object_file(self, build_id: str, build_dir: pathlib.Path, cmd_filepath: pathlib.Path, match: re.Match) -> BuildTarget:
        target = match[1]
        command = match[2]
        sources = list()
        for arg in shlex.split(command):
            if self._mod_file_pattern.match(arg):
                with open(build_dir / arg[1:]) as mod_file:
                    for line in mod_file:
                        line = line.strip()
                        if line:
                            sources.append(line)
        return BuildTarget(build_id=build_id, cmd_filepath=cmd_filepath, type=BuildTargetType.LinkedObject, target=target, command=command, sources=sources)

    def _parse_kernel_module(self, build_id: str, build_dir: pathlib.Path, cmd_filepath: pathlib.Path, match: re.Match) -> BuildTarget:
        target = match[1]
        command = match[2]
        sources = [arg for arg in shlex.split(command) if self._object_file_pattern.match(arg) is not None]
        return BuildTarget(build_id=build_id, cmd_filepath=cmd_filepath, type=BuildTargetType.Module, target=target, command=command, sources=sources)

def main(build_dir: pathlib.Path, db_filepath: str, logger: logging.Logger):
    cmd_parser = CMDFileParser()
    build_id = str(uuid.uuid4())
    with CompilationDatabase(db_filepath=db_filepath) as db:
        db.insert_build(KernelBuild(identifier=build_id, path=build_dir, timestamp=datetime.datetime.now()))
        for target in cmd_parser.discover_and_parse(build_id, build_dir):
            db.insert_target(target)
            logger.info('Processed %s', target.cmd_filepath)
        db.commit()
    logger.info('Finished processing %s', build_id)

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Compilation database builder for Linux kernel')
    parser.add_argument('--build-dir', type=str, required=True, help='Linux kernel build directory')
    parser.add_argument('--db', type=str, required=True, help='Path to SQLite3 database')
    args = parser.parse_args(sys.argv[1:])

    logger = logging.Logger(name='default')
    logger.addHandler(logging.StreamHandler(sys.stderr))

    main(build_dir=pathlib.Path(args.build_dir), db_filepath=args.db, logger=logger)
