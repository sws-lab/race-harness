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
import uuid
from typing import List, Any, Optional
from compile_db.compilation_database import CompilationDatabase, KernelBuild, BuildTarget, BuildTargetType
from goblint_driver.goblint_driver import resolve_build, GoblintDriverException, GoblintDriver

@dataclasses.dataclass
class VerificationTask:
    task_filepath: pathlib.Path
    harness_compiler_extraargs: List[str]
    kernel_inputs: List[str]
    stubs: List[str]
    harness: str
    goblint_conf: List[Any]
    goblint_extra_args: List[str]

    @property
    def root_dir(self) -> pathlib.Path:
        return self.task_filepath.parent

    @staticmethod
    def load(config, task_filepath: pathlib.Path):
        return VerificationTask(
            task_filepath=task_filepath,
            harness_compiler_extraargs=config.get('harness_extra', list()),
            kernel_inputs=config['kernel_inputs'],
            stubs=config['stubs'],
            harness=config['harness'],
            goblint_conf=config.get('goblint_conf', dict()),
            goblint_extra_args=config.get('goblint_extra_args', list())
        )

class VerificationTaskDriver:
    def __init__(self, db: CompilationDatabase, goblint_path: str, harness_compiler_path: str, amalgamate: Optional[str], logger: logging.Logger):
        self._harness_compiler_path = harness_compiler_path
        self._amalgamate = amalgamate
        self._logger = logger
        self._goblint_driver = GoblintDriver(db, goblint_path, logger)

    def __call__(self, build: KernelBuild, task: VerificationTask):
        harness_filepath = pathlib.Path(task.harness)
        if not harness_filepath.is_absolute():
            harness_filepath = task.root_dir / harness_filepath
        with tempfile.NamedTemporaryFile(mode='wb', suffix='.c') as harness_file:
            self._logger.info(f'Start generating harness code for {task.harness}')
            harness_compiler_extraargs = list()
            for arg in task.harness_compiler_extraargs:
                if arg.startswith('-'):
                    harness_compiler_extraargs.append(arg)
                elif path := pathlib.Path(arg):
                    if path.is_absolute():
                        harness_compiler_extraargs.append(str(path.resolve()))
                    elif (rel_path := task.root_dir / path).exists():
                        harness_compiler_extraargs.append(str(rel_path.resolve()))
                    else:
                        harness_compiler_extraargs.append(arg)
                else:
                    harness_compiler_extraargs.append(arg)
            subprocess.check_call(
                executable=self._harness_compiler_path,
                args=[self._harness_compiler_path, str(harness_filepath), *harness_compiler_extraargs],
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
                stubs.append(str(stub_path.absolute()))

            with tempfile.TemporaryDirectory() as goblint_conf_dir:
                goblint_confs = list()
                for index, conf_part in enumerate(task.goblint_conf):
                    if isinstance(conf_part, str):
                        conf_filepath = pathlib.Path(conf_part)
                        if not conf_filepath.is_absolute():
                            conf_filepath = task.root_dir / conf_filepath
                        goblint_confs.append(str(conf_filepath.absolute()))
                    else:
                        conf_filename = os.path.join(goblint_conf_dir, f'conf{index}.json')
                        with open(conf_filename, 'w') as conf_out:
                            json.dump(conf_part, conf_out)
                        goblint_confs.append(conf_filename)
                if self._amalgamate:
                    conf_amalgam, task_amalgam = self._goblint_driver.run(build, goblint_confs, task.goblint_extra_args, [*task.kernel_inputs, *stubs, harness_file.name], amalgamate=True)
                    if not os.path.isdir(self._amalgamate):
                        os.mkdir(self._amalgamate)
                    random_id = str(uuid.uuid4())
                    conf_amalgam_filepath = os.path.join(self._amalgamate, f'{task.task_filepath.with_suffix("").name}-{random_id}.json')
                    amalgam_filepath = os.path.join(self._amalgamate, f'{task.task_filepath.with_suffix("").name}-{random_id}.c')
                    with open(conf_amalgam_filepath, 'w') as conf_amalgam_file:
                        json.dump(conf_amalgam, conf_amalgam_file, indent=2)
                    with open(amalgam_filepath, 'w') as amalgam_file:
                        amalgam_file.write(task_amalgam)
                else:
                    self._goblint_driver.run(build, goblint_confs, task.goblint_extra_args, [*task.kernel_inputs, *stubs, harness_file.name])


if __name__ == '__main__':
    prog_name = os.path.basename(__file__)
    parser = argparse.ArgumentParser(prog=prog_name, description='Goblint driver for Linux kernel module verification')
    parser.add_argument('--db', type=str, required=True, help='Kernel compilation database path')
    parser.add_argument('--build-id', type=str, required=False, help='Kernel build identifier')
    parser.add_argument('--goblint', type=str, required=True, help='Goblint executable path')
    parser.add_argument('--harness-compiler', type=str, required=True, help='Harness compiler executable path')
    parser.add_argument('--quiet', action='store_true', help='Suppress all logging')
    parser.add_argument('--amalgamate', type=str, required=False, help='Generate and save amalgamation of verification task and configuration instead of running verification')
    parser.add_argument('task', help='Verification task definition is JSON format')

    args = parser.parse_args(sys.argv[1:])
    logger = logging.Logger(prog_name)
    if not args.quiet:
        logger.addHandler(logging.StreamHandler(sys.stderr))

    with open(args.task) as task_file:
        task = VerificationTask.load(json.load(task_file), pathlib.Path(args.task))

    with CompilationDatabase(args.db) as db:
        try:
            build = resolve_build(db, args.build_id)
            driver = VerificationTaskDriver(db, args.goblint, args.harness_compiler, args.amalgamate, logger)
            driver(build, task)
        except GoblintDriverException as ex:
            logger.error(str(ex))
            sys.exit(-1)