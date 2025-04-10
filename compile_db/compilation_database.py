import dataclasses
import enum
import pathlib
import shlex
import functools
import sqlite3
import datetime
from typing import List, Iterable, Optional

class BuildTargetType(enum.Enum):
    CompiledObject = 0
    LinkedObject = 1
    Module = 2

@dataclasses.dataclass
class BuildTarget:
    build_id: str
    cmd_filepath: pathlib.Path
    type: BuildTargetType
    target: str
    command: str
    sources: List[str]

    @functools.cached_property
    def tool(self) -> str:
        return shlex.split(self.command)[0]
    
    @functools.cached_property
    def tool_args(self) -> List[str]:
        return shlex.split(self.command)[1:]
    
@dataclasses.dataclass
class KernelBuild:
    identifier: str
    path: pathlib.Path
    timestamp: datetime.datetime

class CompilationDatabase:
    def __init__(self, db_filepath: str):
        self._db_filepath = db_filepath
        self._db = None

    def __enter__(self, *args, **kwargs):
        self._db = sqlite3.connect(self._db_filepath)
        self._initialize_db_schema()
        return self
    
    def __exit__(self, *args, **kwargs):
        self._db.commit()
        self._db.close()
        
    def _initialize_db_schema(self):
        self._db.execute('''
            CREATE TABLE IF NOT EXISTS KernelBuilds (
                ID VARCHAR PRIMARY KEY,
                Path VARCHAR NOT NULL,
                Timestamp INTEGER NOT NULL
            );
        ''')
        self._db.execute('''
            CREATE TABLE IF NOT EXISTS Targets (
                BuildID VARCHAR NOT NULL,
                Target VARCHAR NOT NULL,
                CmdFile VARCHAR NOT NULL,
                Type INTEGER NOT NULL,
                Command VARCHAR NOT NULL,
                PRIMARY KEY (BuildID, Target),
                FOREIGN KEY (BuildID) REFERENCES KernelBuilds(ID)
            )
        ''')
        self._db.execute('''
            CREATE TABLE IF NOT EXISTS TargetDependencies (
                BuildID VARCHAR NOT NULL,
                Target VARCHAR NOT NULL,
                Source VARCHAR NOT NULL,
                PRIMARY KEY (BuildID, Target, Source)
                FOREIGN KEY (BuildID, Target) REFERENCES Targets (BuildID, Target)
            )
        ''')
    
    def all_kernel_builds(self) -> Iterable[KernelBuild]:
        cursor = self._db.execute('''
            SELECT ID, Path, Timestamp FROM KernelBuilds
        ''')
        for row in cursor:
            yield KernelBuild(
                identifier=row[0],
                path=pathlib.Path(row[1]),
                timestamp=datetime.datetime.fromtimestamp(row[2])
            )

    def all_build_targets(self, build_id: str) -> Iterable[BuildTarget]:
        cursor = self._db.execute('''
            SELECT Target, CmdFile, Type, Command FROM Targets WHERE BuildID = ?
        ''', [build_id])
        for row in cursor:
            target = BuildTarget(
                build_id=build_id,
                cmd_filepath=row[1],
                type=BuildTargetType(row[2]),
                target=row[0],
                command=row[3],
                sources=list(self.target_dependencies(build_id, row[0]))
            )
            yield target

    def all_build_targets_of_type(self, build_id: str, type: BuildTargetType) -> Iterable[BuildTarget]:
        cursor = self._db.execute('''
            SELECT Target, CmdFile, Command FROM Targets WHERE BuildID = ? AND Type = ?
        ''', [build_id, type.value])
        for row in cursor:
            target = BuildTarget(
                build_id=build_id,
                cmd_filepath=row[1],
                type=type,
                target=row[0],
                command=row[2],
                sources=list(self.target_dependencies(build_id, row[0]))
            )
            yield target

    def build_target(self, build_id: str, target: str) -> Optional[BuildTarget]:
        cursor = self._db.execute('''
            SELECT Type, CmdFile, Command FROM Targets WHERE BuildID = ? AND Target = ?
        ''', [build_id, target])
        if cursor.rowcount == 0:
            return None
        else:
            row = cursor.fetchone()
            return BuildTarget(
                build_id=build_id,
                cmd_filepath=row[1],
                type=row[0],
                target=target,
                command=row[2],
                sources=list(self.target_dependencies(build_id, target))
            )

    def target_dependencies(self, build_id: str, target: str) -> Iterable[str]:
        cursor = self._db.execute('''
            SELECT Source FROM TargetDependencies WHERE BuildID = ? AND Target = ?
        ''', [build_id, target])
        for row in cursor:
            yield row[0]
            
    def all_transitive_target_dependencies(self, build_id: str, target: str) -> Iterable[str]:
        cursor = self._db.execute('''
            WITH RECURSIVE DepRec AS (
                SELECT BuildID, Source FROM TargetDependencies WHERE BuildID = ? AND Target = ?
                UNION
                SELECT TargetDependencies.BuildID, TargetDependencies.Source
                    FROM TargetDependencies
                    INNER JOIN DepRec ON DepRec.BuildID = TargetDependencies.BuildID AND DepRec.Source = TargetDependencies.Target
            )
            SELECT Source FROM DepRec
        ''', [build_id, target])
        for row in cursor:
            yield row[0]

    def target_base_dependencies(self, build_id: str, target: str) -> Iterable[str]:
        cursor = self._db.execute('''
            WITH RECURSIVE DepRec AS (
                SELECT BuildID, Source FROM TargetDependencies WHERE BuildID = ? AND Target = ?
                UNION
                SELECT TargetDependencies.BuildID, TargetDependencies.Source
                    FROM TargetDependencies
                    INNER JOIN DepRec ON DepRec.BuildID = TargetDependencies.BuildID AND DepRec.Source = TargetDependencies.Target
            )
            SELECT Source FROM DepRec
            WHERE NOT EXISTS (SELECT 1 FROM Targets WHERE Targets.BuildID = DepRec.BuildID AND Source = Targets.Target)
        ''', [build_id, target])
        for row in cursor:
            yield row[0]

    def insert_build(self, build: KernelBuild, *, commit: bool = False):
        self._db.execute('''
            INSERT INTO KernelBuilds VALUES (?, ?, ?);
        ''', [build.identifier, str(build.path.absolute()), int(build.timestamp.timestamp())])
        if commit:
            self._db.commit()

    def insert_target(self, target: BuildTarget, *, commit: bool = False):
        self._db.execute('''
            INSERT INTO Targets VALUES (?, ?, ?, ?, ?)
        ''', [target.build_id, target.target, str(target.cmd_filepath), target.type.value, target.command])
        self._db.executemany('''
            INSERT INTO TargetDependencies VALUES (?, ?, ?)
        ''', ((target.build_id, target.target, source) for source in set(target.sources)))
        if commit:
            self._db.commit()

    def commit(self):
        self._db.commit()
