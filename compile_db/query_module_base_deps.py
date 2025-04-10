#!/usr/bin/env python3
import sys
import argparse
import json
from compilation_database import CompilationDatabase, BuildTargetType

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--db', type=str, required=True, help='Path to SQLite3 database')
    parser.add_argument('--build-id', type=str, required=True, help='Build identifier')
    args = parser.parse_args(sys.argv[1:])

    result = dict()
    with CompilationDatabase(args.db) as db:
        for target in db.all_build_targets_of_type(build_id=args.build_id, type=BuildTargetType.Module):
            result[target.target] = list(db.target_base_dependencies(build_id=args.build_id, target=target.target))
    json.dump(result, sys.stdout, indent=2)
