import dataclasses
from typing import Union, Optional, Iterable

@dataclasses.dataclass
class IndentedLine:
    relative_indent: int
    line: Optional[str]

IndentedLineGenerator = Iterable[Union[IndentedLine, str]]
