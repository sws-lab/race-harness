#!/usr/bin/env python3
import sys
import os
import argparse
import json
import logging
from typing import Optional, Iterable
from compilation_database import CompilationDatabase, BuildTargetType, KernelBuild, BuildTarget

class CompilationDatabaseQueryException(BaseException): pass

def get_build(db: CompilationDatabase, build_id: str) -> KernelBuild:
    build = db.kernel_build(build_id)
    if build is None:
        raise CompilationDatabaseQueryException(f'Unable to find requested kernel build {build_id} in {db.db_filepath}')
    return build


def resolve_build(db: CompilationDatabase, build_id: Optional[str]) -> KernelBuild:
    if build_id is not None:
        return get_build(db, build_id)
    else:
        build = db.latest_kernel_build()
        if build is None:
            raise CompilationDatabaseQueryException(f'Kernel compilation database {db.db_filepath} has no builds')
        return build
    
def resolve_targets(db: CompilationDatabase, build_id: str, target: Optional[str], include_objects: bool) -> Iterable[BuildTarget]:
    if target is not None:
        build_target = db.build_target(build_id, target)
        if build_target is None:
            raise CompilationDatabaseQueryException(f'Unable to find requested target for kernel build {build_id} in database {db.db_filepath}')
        return [build_target]
    elif include_objects:
        return db.all_build_targets(build_id=build_id)
    else:
        return db.all_build_targets_of_type(build_id=build_id, type=BuildTargetType.Module)
    
def query_builds(db: CompilationDatabase, build_id: Optional[str], target: Optional[str], include_objects: bool, logger: logging.Logger):
    if build_id is not None:
        builds = [db.kernel_build(build_id)]
    else:
        builds = db.all_kernel_builds()
    json.dump({
        build.identifier: {
            'path': str(build.path),
            'timestamp': int(build.timestamp.timestamp())
        }
        for build in builds
    }, sys.stdout, indent=2)
        
def query_module_base_deps(db: CompilationDatabase, build_id: Optional[str], target: Optional[str], include_objects: bool, logger: logging.Logger):
    build = resolve_build(db, build_id)
    logger.info('Quering kernel build %s', build.identifier)

    targets = resolve_targets(db, build.identifier, target, include_objects)
    result = dict()
    for target in targets:
        result[target.target] = list(db.target_base_dependencies(build_id=build.identifier, target=target.target))
    json.dump(result, sys.stdout, indent=2)

def query_module_all_deps(db: CompilationDatabase, build_id: Optional[str], target: Optional[str], include_objects: bool, logger: logging.Logger):
    build = resolve_build(db, build_id)
    logger.info('Quering kernel build %s', build.identifier)

    targets = resolve_targets(db, build.identifier, target, include_objects)
    result = dict()
    for target in targets:
        result[target.target] = list(db.all_transitive_target_dependencies(build_id=build.identifier, target=target.target))
    json.dump(result, sys.stdout, indent=2)

def query_module_direct_deps(db: CompilationDatabase, build_id: Optional[str], target: Optional[str], include_objects: bool, logger: logging.Logger):
    build = resolve_build(db, build_id)
    logger.info('Quering kernel build %s', build.identifier)

    targets = resolve_targets(db, build.identifier, target, include_objects)
    result = dict()
    for target in targets:
        result[target.target] = list(db.target_dependencies(build_id=build.identifier, target=target.target))
    json.dump(result, sys.stdout, indent=2)

if __name__ == '__main__':
    prog_basename = os.path.basename(__file__)
    parser = argparse.ArgumentParser(prog=prog_basename, description='Linux kernel compilation database queries')
    parser.add_argument('--db', type=str, required=True, help='Kernel compilation database path')
    parser.add_argument('--build-id', type=str, required=False, help='Kernel build identifier')
    parser.add_argument('--quiet', action='store_true', required=False, help='Suppress any logs')
    query_group = parser.add_mutually_exclusive_group(required=True)
    query_group.add_argument('--query-builds', action='store_true', help='Query kernel builds')
    query_group.add_argument('--query-base-deps', action='store_true', help='Query source dependencies of kernel build artifact')
    query_group.add_argument('--query-all-deps', action='store_true', help='Query all dependencies of kernel build artifact')
    query_group.add_argument('--query-direct-deps', action='store_true', help='Query direct dependencies of kernel build artifact')
    target_group = parser.add_mutually_exclusive_group(required=False)
    target_group.add_argument('--include-objects', action='store_true', default=False, help='Include object files into the queried targets')
    target_group.add_argument('--target', type=str, help='Query specific target')
    args = parser.parse_args(sys.argv[1:])

    logger = logging.Logger(prog_basename)
    if not args.quiet:
        logger.addHandler(logging.StreamHandler(sys.stderr))

    logger.info('Opening kernel build database %s', args.db)
    with CompilationDatabase(args.db) as db:
        if args.query_builds:
            handler = query_builds
        elif args.query_base_deps:
            handler = query_module_base_deps
        elif args.query_all_deps:
            handler = query_module_all_deps
        elif args.query_direct_deps:
            handler = query_module_direct_deps
        
        try:
            handler(db, args.build_id, args.target, args.include_objects, logger)
        except CompilationDatabaseQueryException as ex:
            logger.error(str(ex))
            sys.exit(-1)
