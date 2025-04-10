#!/usr/bin/env python3

import sys
import argparse
import json
from compilation_database import CompilationDatabase

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--db', type=str, required=True, help='Path to SQLite3 database')
    args = parser.parse_args(sys.argv[1:])

    result = dict()
    with CompilationDatabase(args.db) as db:
        for build in db.all_kernel_builds():
            result[build.identifier] = {
                'path': str(build.path),
                'timestamp': int(build.timestamp.timestamp())
            }
    json.dump(result, sys.stdout, indent=2)
