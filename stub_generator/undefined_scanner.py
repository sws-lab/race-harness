import dataclasses
from typing import Iterable, Container, Collection
import clang.cindex as cindex

class UndefinedReferenceScannerError(Exception): pass

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
    def __init__(self, profile: UndefinedReferenceScannerProfile):
        self._index = cindex.Index.create()
        self._profile = profile
        self._used_function_usrs = set()
        self._used_variable_usrs = set()
        self._defined_function_usrs = set()
        self._defined_variable_usrs = set()
        self._entity_index = dict()
        self._includes = set()

    def load(self, *args, **kwargs):
        unit = self._index.parse(*args, **kwargs)
        if not unit:
            raise UndefinedReferenceScannerError('Failed to parse input')
        self._traverse_node(unit.cursor)

        for include in unit.get_includes():
            if include.depth == 1:
                self._includes.add(str(include.include))
    
    def _undefined_functions(self) -> Collection[str]:
        return self._used_function_usrs.difference(self._defined_function_usrs)
    
    def _undefined_variables(self) -> Collection[str]:
        return self._used_variable_usrs.difference(self._defined_variable_usrs)

    def undefined_functions(self) -> Iterable[cindex.Cursor]:
        return sorted((
            self._entity_index[usr]
            for usr in self._undefined_functions()
        ), key=lambda node: node.spelling)
    
    def undefined_variables(self) -> Iterable[cindex.Cursor]:
        return sorted((
            self._entity_index[usr]
            for usr in self._undefined_variables()
        ), key=lambda node: node.spelling)
    
    def includes(self) -> Iterable[str]:
        return sorted(self._includes)
    
    def _is_external_decl(self, node: cindex.Cursor):
        if node.linkage == cindex.LinkageKind.EXTERNAL:
            return True
    
        for token in node.get_tokens():
            if token.spelling in self._profile.external_declaration_markings:
                return True
            
        return False
    
    def _traverse_node(self, node: cindex.Cursor, recursive: bool = False):
        def traverse(node: cindex.Cursor, recursive: bool):
            for child in node.get_children():
                self._traverse_node(child, recursive)

        self._match_definitions_and_uses(node)

        if node.kind == cindex.CursorKind.TRANSLATION_UNIT:
            traverse(node, recursive)
        elif (node.kind == cindex.CursorKind.FUNCTION_DECL or node.kind == cindex.CursorKind.VAR_DECL) and \
            node.is_definition() and \
            self._is_external_decl(node):
            traverse(node, True)        
        elif node.kind == cindex.CursorKind.CALL_EXPR and \
            node.referenced is not None:
            traverse(node.referenced, True)
            traverse(node, True)
        elif node.kind == cindex.CursorKind.DECL_REF_EXPR and \
            node.referenced is not None:
            traverse(node.referenced, True)
            traverse(node, True)
        elif recursive:
            traverse(node, True)
        
    def _match_definitions_and_uses(self, node: cindex.Cursor):
        def found_func_decl(declaration: cindex.Cursor):
            declaration = declaration.canonical
            decl_usr = declaration.get_usr()
            self._used_function_usrs.add(decl_usr)
            if decl_usr not in self._entity_index:
                self._entity_index[decl_usr] = declaration

        def found_variable_decl(declaration: cindex.Cursor):
            declaration = declaration.canonical
            decl_usr = declaration.get_usr()
            self._used_variable_usrs.add(decl_usr)
            if decl_usr not in self._entity_index:
                self._entity_index[decl_usr] = declaration

        if node.kind == cindex.CursorKind.CALL_EXPR and \
            node.referenced is not None and \
            node.referenced.kind == cindex.CursorKind.FUNCTION_DECL:
            found_func_decl(node.referenced)
        elif node.kind == cindex.CursorKind.DECL_REF_EXPR and \
            node.referenced is not None:
            if node.referenced.kind == cindex.CursorKind.FUNCTION_DECL:
                found_func_decl(node.referenced)
            elif node.referenced.kind == cindex.CursorKind.VAR_DECL:
                found_variable_decl(node.referenced)
        elif node.kind == cindex.CursorKind.FUNCTION_DECL:
            if node.is_definition():
                self._defined_function_usrs.add(node.canonical.get_usr())
            if node.canonical.get_usr() not in self._entity_index:
                self._entity_index[node.canonical.get_usr()] = node.canonical
        elif node.kind == cindex.CursorKind.VAR_DECL:
            if node.is_definition() or node.linkage != cindex.LinkageKind.EXTERNAL:
                self._defined_variable_usrs.add(node.canonical.get_usr())
            if node.canonical.get_usr() not in self._entity_index:
                self._entity_index[node.canonical.get_usr()] = node.canonical
