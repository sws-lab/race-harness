#!/usr/bin/env python3

import sys
import os
import logging
import json
import tempfile
import dataclasses
import argparse
import subprocess
import pathlib
from typing import List, Any
from compile_db.compilation_database import CompilationDatabase, KernelBuild, BuildTarget, BuildTargetType
from goblint_driver.goblint_driver import resolve_build, GoblintDriverException, GoblintDriver

@dataclasses.dataclass
class VerificationTask:
    root_dir: pathlib.Path
    kernel_inputs: List[str]
    stubs: List[str]
    harness: str
    goblint_conf: Any
    goblint_extra_args: List[str]

    @staticmethod
    def load(config, root_dir: pathlib.Path):
        return VerificationTask(
            root_dir=root_dir,
            kernel_inputs=config['kernel_inputs'],
            stubs=config['stubs'],
            harness=config['harness'],
            goblint_conf=config.get('goblint_conf', dict()),
            goblint_extra_args=config.get('goblint_extra_args', list())
        )

class VerificationTaskDriver:
    def __init__(self, db: CompilationDatabase, goblint_path: str, harness_compiler_path: str, logger: logging.Logger):
        self._harness_compiler_path = harness_compiler_path
        self._logger = logger
        self._goblint_driver = GoblintDriver(db, goblint_path, logger)

    def __call__(self, build: KernelBuild, task: VerificationTask):
        harness_filepath = pathlib.Path(task.harness)
        if not harness_filepath.is_absolute():
            harness_filepath = task.root_dir / harness_filepath
        with tempfile.NamedTemporaryFile(mode='wb', suffix='.c') as harness_file:
            self._logger.info(f'Start generating harness code for {task.harness}')
            subprocess.check_call(
                executable=self._harness_compiler_path,
                args=[self._harness_compiler_path, str(harness_filepath)],
                stdin=subprocess.DEVNULL,
                stdout=harness_file.file,
                shell=False
            )
            self._logger.info(f'Finish generating harness code for {task.harness}')

            stubs = list()
            for stub_filepath in task.stubs:
                stub_path = pathlib.Path(stub_filepath)
                if not stub_path.is_absolute():
                    stub_path = task.root_dir / stub_path
                stubs.append(str(stub_path))

            with tempfile.NamedTemporaryFile(mode='w', suffix='.json') as goblint_conf:
                json.dump(task.goblint_conf, goblint_conf)
                goblint_conf.flush()
                self._goblint_driver.run(build, goblint_conf.name, task.goblint_extra_args, [*task.kernel_inputs, *stubs, harness_file.name])


if __name__ == '__main__':
    prog_name = os.path.basename(__file__)
    parser = argparse.ArgumentParser(prog=prog_name, description='Goblint driver for Linux kernel module verification')
    parser.add_argument('--db', type=str, required=True, help='Kernel compilation database path')
    parser.add_argument('--build-id', type=str, required=False, help='Kernel build identifier')
    parser.add_argument('--goblint', type=str, required=True, help='Goblint executable path')
    parser.add_argument('--harness-compiler', type=str, required=True, help='Harness compiler executable path')
    parser.add_argument('--quiet', action='store_true', help='Suppress all logging')
    parser.add_argument('task', help='Verification task definition is JSON format')

    args = parser.parse_args(sys.argv[1:])
    logger = logging.Logger(prog_name)
    if not args.quiet:
        logger.addHandler(logging.StreamHandler(sys.stderr))

    with open(args.task) as task_file:
        task = VerificationTask.load(json.load(task_file), pathlib.Path(args.task).parent)

    with CompilationDatabase(args.db) as db:
        try:
            build = resolve_build(db, args.build_id)
            driver = VerificationTaskDriver(db, args.goblint, args.harness_compiler, logger)
            driver(build, task)
        except GoblintDriverException as ex:
            logger.error(str(ex))
            sys.exit(-1)