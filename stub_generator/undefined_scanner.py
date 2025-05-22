import dataclasses
from typing import Iterable, Container, Collection, Optional, Callable
import clang.cindex as cindex

class UndefinedReferenceScannerError(BaseException): pass

@dataclasses.dataclass
class UndefinedReferenceScannerProfile:
    external_declaration_markings: Container[str]

    @staticmethod
    def linux_kernel() -> 'UndefinedReferenceScannerProfile':
        return UndefinedReferenceScannerProfile(
            # TODO To be extended with markings from https://github.com/torvalds/linux/blob/16f73eb02d7e1765ccab3d2018e0bd98eb93d973/include/linux/init.h
            external_declaration_markings=[
                '__init', '__initdata', '__initconst', '__exit', '__exitdata', '__exit_call'
            ]
        )

class UndefinedReferenceScanner:
    def __init__(self, profile: UndefinedReferenceScannerProfile, include_symbols: Optional[Callable[[cindex.Cursor], bool]]):
        self._index = cindex.Index.create()
        self._profile = profile
        self._used_function_usrs = set()
        self._used_variable_usrs = set()
        self._defined_function_usrs = set()
        self._defined_variable_usrs = set()
        self._entity_index = dict()
        self._includes = list()
        self._traversed = set()
        self._include_symbols = include_symbols

    def load(self, *args, **kwargs):
        unit = self._index.parse(*args, **kwargs)
        if not unit:
            raise UndefinedReferenceScannerError('Failed to parse input')
        self._traverse_node(unit.cursor, False)

        for include in unit.get_includes():
            include_filepath = str(include.include)
            if include.depth == 1 and include_filepath not in self._includes:
                self._includes.append(str(include.include))
    
    def _undefined_functions(self) -> Collection[str]:
        return self._used_function_usrs.difference(self._defined_function_usrs)
    
    def _undefined_variables(self) -> Collection[str]:
        return self._used_variable_usrs.difference(self._defined_variable_usrs)
    
    def _resolve_usrs(self, usrs: Iterable[str]) -> Iterable[cindex.Cursor]:
        return sorted((
            self._entity_index[usr]
            for usr in usrs
        ), key=lambda node: node.spelling)

    def undefined_functions(self) -> Iterable[cindex.Cursor]:
        return self._resolve_usrs(self._undefined_functions())
    
    def undefined_variables(self) -> Iterable[cindex.Cursor]:
        return self._resolve_usrs(self._undefined_variables())
    
    def includes(self) -> Iterable[str]:
        return self._includes
    
    def _is_external_decl(self, node: cindex.Cursor):
        if node.linkage == cindex.LinkageKind.EXTERNAL or (self._include_symbols is not None and self._include_symbols(node)):
            return True
    
        for token in node.get_tokens():
            if token.spelling in self._profile.external_declaration_markings:
                return True
            
        return False
    
    def _traverse_node_children(self, node: cindex.Cursor, recursive: bool):
        node_usr = node.get_usr()
        if node.is_definition() and node_usr:
            if node_usr in self._traversed:
                return
            self._traversed.add(node_usr)

        for child in node.get_children():
            self._traverse_node(child, recursive)
    
    def _traverse_node(self, node: cindex.Cursor, recursive: bool):
        self._match_definitions_and_uses(node)

        if node.kind == cindex.CursorKind.TRANSLATION_UNIT:
            self._traverse_node_children(node, recursive)
        elif (node.kind == cindex.CursorKind.FUNCTION_DECL or node.kind == cindex.CursorKind.VAR_DECL) and \
            node.is_definition() and \
            self._is_external_decl(node):
            self._traverse_node_children(node, True)        
        elif node.kind == cindex.CursorKind.CALL_EXPR and \
            node.referenced is not None:
            self._traverse_node(node.referenced, True)
            self._traverse_node_children(node, True)
        elif node.kind == cindex.CursorKind.DECL_REF_EXPR and \
            node.referenced is not None:
            self._traverse_node(node.referenced, True)
            self._traverse_node_children(node, True)
        elif recursive:
            self._traverse_node_children(node, True)

    def _update_entity_index(self, node: cindex.Cursor):
        usr = node.get_usr()
        if usr not in self._entity_index:
            self._entity_index[usr] = node

    def _found_function_declaration(self, declaration: cindex.Cursor):
        declaration = declaration.canonical
        decl_usr = declaration.get_usr()
        self._used_function_usrs.add(decl_usr)
        self._update_entity_index(declaration)

    def _found_variable_declaration(self, declaration: cindex.Cursor):
        declaration = declaration.canonical
        decl_usr = declaration.get_usr()
        self._used_variable_usrs.add(decl_usr)
        self._update_entity_index(declaration)
        
    def _match_definitions_and_uses(self, node: cindex.Cursor):
        if node.kind == cindex.CursorKind.CALL_EXPR and \
            node.referenced is not None and \
            node.referenced.kind == cindex.CursorKind.FUNCTION_DECL:
            self._found_function_declaration(node.referenced)
        elif node.kind == cindex.CursorKind.DECL_REF_EXPR and \
            node.referenced is not None:
            if node.referenced.kind == cindex.CursorKind.FUNCTION_DECL:
                self._found_function_declaration(node.referenced)
            elif node.referenced.kind == cindex.CursorKind.VAR_DECL:
                self._found_variable_declaration(node.referenced)
        elif node.kind == cindex.CursorKind.FUNCTION_DECL:
            canonical_node = node.canonical
            if node.is_definition():
                self._defined_function_usrs.add(canonical_node.get_usr())
            self._update_entity_index(canonical_node)
        elif node.kind == cindex.CursorKind.VAR_DECL:
            canonical_node = node.canonical
            if node.is_definition() or node.linkage != cindex.LinkageKind.EXTERNAL:
                self._defined_variable_usrs.add(canonical_node.get_usr())
            self._update_entity_index(canonical_node)
