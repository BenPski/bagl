"""
Use parser combinators to do the parsing

parse :: string -> [(a, string)]


What should the syntax be?

let variable = expr in expr
(\\ x . x)

use infix notation for operators



"""

from abc import ABCMeta, abstractmethod
import copy


def runParser(parser, string):
    res = parser(string)
    print(res)
    if len(res) == 1:
        if len(res[0][1]) == 0:
            return res
    raise RuntimeError("Some parse error.")


class Parser(metaclass=ABCMeta):

    def __call__(self, string):
        return self.parse(string)

    @abstractmethod
    def parse(self, string):
        pass


class Basic(Parser):
    def __init__(self, f):
        self.f = f

    def parse(self, string):
        return self.f(string)


class Item(Parser):
    def parse(self, string):
        if len(string) == 0:
            return []
        else:
            return [(string[0], string[1:])]


class Unit(Parser):
    def __init__(self, a):
        self.a = a

    def parse(self, string):
        return [(self.a, string)]


class Bind(Parser):
    def __init__(self, p, f):
        self.p = p
        self.f = f

    def parse(self, string):
        res = self.p(string)
        accum = []
        for r in res:
            a = r[0]
            s = copy.deepcopy(r[1])
            p = self.f(a)
            accum += p(s)
        return accum


class Map(Parser):
    def __init__(self, f, p):
        self.f = f
        self.p = p

    def parse(self, string):
        res = self.p(string)
        return [(self.f(a), b) for (a, b) in res]


class Apply(Parser):
    def __init__(self, p1, p2):
        self.p1 = p1
        self.p2 = p2

    def parse(self, string):
        res = self.p1(string)
        accum = []
        for (f, s1) in res:
            a, s2 = self.p2(s1)
            accum.append((f(a), s2))
        return accum


class Failure(Parser):
    def parse(self, string):
        return []


class Combine(Parser):
    def __init__(self, first, second):
        self.first = first
        self.second = second

    def parse(self, string):
        # not sure if this is really necessary
        s1 = copy.deepcopy(string)
        return self.first(s1) + self.second(string)


class Option(Parser):
    def __init__(self, first, second):
        self.first = first
        self.second = second

    def parse(self, string):
        s1 = copy.deepcopy(string)
        res = self.first(s1)
        if len(res) == 0:
            return self.second(string)
        else:
            return res


class Satisfy(Parser):
    def __init__(self, pred):
        def cond(c):
            if pred(c):
                return Unit(c)
            else:
                return Basic(lambda xs: [])
        self.parser = Bind(Item(), lambda c: cond(c))

    def parse(self, string):
        return self.parser(string)