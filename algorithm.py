#!/usr/bin/env python3

from abc import ABC
from dataclasses import dataclass, field, replace
from typing import TypeVar, Generic, Self, Optional, Sequence

V = TypeVar('V')


@dataclass(frozen=True)
class T(Generic[V]):
    label: V
    children: Sequence[Self] = field(default_factory=tuple)

    def __call__(self, *children: Self) -> Self:
        return replace(self, children=children)

    def cursor(self) -> 'Cursor[V]':
        return Cursor(self)


@dataclass(frozen=True)
class Cursor(Generic[V]):
    curr: T[V]
    left: Sequence[T[V]] = field(default_factory=tuple)

    def next_subtree(self) -> Optional[Self]:
        if self.left:
            next, *left = self.left
            return replace(self, curr=next, left=left)

    def first_child(self) -> Optional[Self]:
        if self.curr.children:
            child, *left = self.curr.children
            return replace(self, curr=child, left=[*left, *self.left])

    def first_leaf(self) -> Self:
        curr = replace(self)
        while (child := curr.first_child()) is not None:
            curr = child
        return curr

    def token(self) -> V:
        assert not self.curr.children, 'not a token'
        return self.curr.label


class Wildcard(ABC):
    pass


def match(pattern: Sequence[V | Wildcard], cursor: Optional[Cursor[V]]) -> bool:
    if not pattern or not cursor:
        return not pattern and not cursor

    tok, *rest = pattern  # unpack first & rest

    if isinstance(tok, Wildcard):
        while not match(rest, cursor.next_subtree()):
            cursor = cursor.first_child()
            if not cursor:
                return False
        return True

    cursor = cursor.first_leaf()
    return tok == cursor.token() \
        and match(rest, cursor.next_subtree())


_ = Wildcard()
a, b, c = map(T, 'abc')
r, s, t = map(T, 'rst')

assert match([], None)
assert match('a', a.cursor())
assert match('a', t(a).cursor())
assert match('aa', t(a, a).cursor())
assert match('ab', t(a, b).cursor())
assert match('ab', r(a, t(b)).cursor())
assert match('ab', r(s(a), t(b)).cursor())
assert match('abc', t(a, b, c).cursor())
assert match('abc', r(t(a, b), c).cursor())
assert match('abc', r(s(a, t(b)), c).cursor())
assert match('abc', r(s(a), s(t(b, c))).cursor())

assert match([_], a.cursor())
assert match([_], t(a).cursor())
assert match([_], t(a, b).cursor())
assert match(['a', _], t(a, b).cursor())
assert match([_, 'b'], t(a, b).cursor())
assert match(['a', _, 'c'], t(a, b, c).cursor())
assert match([_, 'b', 'c'], r(t(a, b), c).cursor())
assert match(['a', _, 'c'], r(a, t(b, c)).cursor())

assert not match([], a.cursor())
assert not match(['a'], b.cursor())
assert not match(['a', 'b'], a.cursor())
assert not match(['a', 'b'], b.cursor())
assert not match(['a', 'a'], t(a, b).cursor())

assert not match(['a', _], t(a).cursor())
assert not match([_, 'b'], t(b).cursor())
assert not match(['a', _, 'c'], t(a, c).cursor())
assert not match([_, 'b', 'c'], r(t(b), c).cursor())
assert not match(['a', _, 'c'], r(a, t(b)).cursor())

print('all tests pass!')
