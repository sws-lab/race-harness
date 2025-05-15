import io
from typing import Union, Iterable
from harness.control_flow.node import ControlFlowNode, ControlFlowStatement, ControlFlowSequence, ControlFlowBranchNode, ControlFlowLabelledNode, ControlFlowGotoNode, ControlFlowLock, ControlFlowUnlock

class ControlFlowFormatter:
    def __init__(self):
        pass

    def format(self, node: ControlFlowNode) -> str:
        indent_level = 0
        out = io.StringIO()
        for entry in self._format(node):
            if isinstance(entry, int):
                indent_level += entry
            else:
                out.write('  ' * indent_level)
                out.write(entry)
                out.write('\n')
        return out.getvalue()

    def _format(self, node: ControlFlowNode, *args, **kwargs) -> Iterable[Union[int, str]]:
        if node.as_statement():
            yield from self._format_statement(node.as_statement(), *args, **kwargs)
        elif node.as_sequence():
            yield from self._format_sequence(node.as_sequence(), *args, **kwargs)
        elif node.as_labelled_statement():
            yield from self._format_labelled_statement(node.as_labelled_statement(), *args, **kwargs)
        elif node.as_branch():
            yield from self._format_branch(node.as_branch(), *args, **kwargs)
        elif node.as_goto():
            yield from self._format_goto(node.as_goto(), *args, **kwargs)
        elif node.as_lock():
            yield from self._format_lock(node.as_lock(), *args, **kwargs)
        elif node.as_unlock():
            yield from self._format_unlock(node.as_unlock(), *args, **kwargs)
        
    def _format_statement(self, node: ControlFlowStatement) -> Iterable[Union[int, str]]:
        yield f'{node.state_graph_edge.action.mnemonic}; // {node.state_graph_edge}'
        
    def _format_sequence(self, node: ControlFlowSequence, omit_block: bool = False) -> Iterable[Union[int, str]]:
        if not omit_block:
            yield '{'
            yield 1
        for subnode in node.sequence:
            yield from self._format(subnode)
        if not omit_block:
            yield -1
            yield '}'

    def _format_labelled_statement(self, node: ControlFlowLabelledNode) -> Iterable[Union[int, str]]:
        node_format = self._format(node.body)
        first = next(node_format, None)
        if first is None:
            yield f'{node.label.label}:;'
            return
        elif isinstance(first, str):
            yield f'{node.label.label}: {first}'
        else:
            yield f'{node.label.label}:'
            yield first
        yield from node_format

    def _format_branch(self, node: ControlFlowBranchNode) -> Iterable[Union[int, str]]:
        for index, subnode in enumerate(node.branches):
            if index == 0:
                yield 'if {'
            elif index + 1 < len(node):
                yield '} elif {'
            else:
                yield '} else {'
            yield 1
            yield from self._format(subnode, omit_block=True)
            yield -1
            if index + 1 == len(node):
                yield '}'

    def _format_goto(self, node: ControlFlowGotoNode) -> Iterable[Union[int, str]]:
        yield f'goto {node.label.label};'

    def _format_lock(self, node: ControlFlowLock) -> Iterable[Union[int, str]]:
        yield 'lock {};'.format(
            ', '.join(
                f'mutex{mtx.identifier}'
                for mtx in node.mutexes
            )
        )

    def _format_unlock(self, node: ControlFlowUnlock) -> Iterable[Union[int, str]]:
        yield 'unlock {};'.format(
            ', '.join(
                f'mutex{mtx.identifier}'
                for mtx in node.mutexes
            )
        )
