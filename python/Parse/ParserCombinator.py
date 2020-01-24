"""
Parser combinators

its annoying to use function objects due to having to initialize them, so just functions


parser :: tokens -> [(expr,tokens)]

expression -> let
let -> ("let" | "letrec") Symbol "=" expression "in" expression | lambda
lambda -> "\\" [Symbol] "." expression | application
application -> (Symbol | builtin) [expression] | primary
primary -> Symbol | builtin

"""

from abc import ABCMeta, abstractmethod
from Parse.TokenType import TokenType as TT
from Representation.Expr import *
import copy


def parse(parser, tokens):
    res = parser(tokens)
    if len(res) != 1:
        raise RuntimeError("Some sort of parse error")
    else:
        expr, tokens = res[0]
        if len(tokens) != 0:
            raise RuntimeError("input not fully consumed")
        else:
            return expr


class Parser(metaclass=ABCMeta):
    """
    Must implement run which takes a string and returns [(a, string)] where a is the parsed expression
    """

    def __call__(self, tokens):
        return self.run(tokens)

    @abstractmethod
    def run(self, tokens):
        pass


class Empty(Parser):
    def run(self, tokens):
        return []


class Unit(Parser):
    def __init__(self, a):
        self.a = a

    def run(self, tokens):
        return [(self.a, tokens)]


class Item(Parser):
    def run(self, tokens):
        if len(tokens) == 0:
            return []
        else:
            return [(tokens[0], tokens[1:])]


class Bind(Parser):
    def __init__(self, p, f):
        self.p = p
        self.f = f

    def run(self, tokens):
        res = self.p(tokens)
        final = []
        for (a, s) in res:
            p = self.f(a)
            final += p(copy.deepcopy(s))
        return final


class Then(Parser):
    def __init__(self, p, q):
        self.p = p
        self.q = q

    def run(self, tokens):
        return Bind(self.p, lambda x: self.q)(tokens)


class Map(Parser):
    def __init__(self, f, p):
        self.f = f
        self.p = p

    def run(self, tokens):
        res = self.p(tokens)
        final = []
        for (a, s) in res:
            final.append((self.f(a), copy.deepcopy(s)))
        return final


class Apply(Parser):
    def __init__(self, p, q):
        self.p = p
        self.q = q

    def run(self, tokens):
        final = []
        for (f, s1) in self.p(tokens):
            for (a, s2) in self.q(s1):
                final += (f(a), s2)
        return final


class Combine(Parser):
    def __init__(self, p1, p2):
        self.p1 = p1
        self.p2 = p2

    def run(self, tokens):
        return self.p1(copy.deepcopy(tokens)) + self.p2(copy.deepcopy(tokens))


class Option(Parser):
    def __init__(self, p1, p2):
        self.p1 = p1
        self.p2 = p2

    def run(self, tokens):
        res = self.p1(copy.deepcopy(tokens))
        if len(res) == 0:
            return self.p2(copy.deepcopy(tokens))
        else:
            return res


class Satisfy(Parser):
    def __init__(self, pred):
        self.pred = pred

    def run(self, tokens):
        item = Item()
        b = Bind(Item(), lambda x: self.help(x))
        return b(tokens)

    def help(self, val):
        if self.pred(val):
            return Unit(val)
        else:
            return Empty()


class OneOf(Parser):
    def __init__(self, elems):
        self.elems = elems

    def run(self, tokens):
        return Satisfy(lambda x: x in self.elems)(tokens)


class Many(Parser):
    def __init__(self, p):
        self.p = p

    def run(self, tokens):
        return Option(Some(self.p), Empty())(tokens)


class Some(Parser):
    def __init__(self, p):
        self.p = p

    def run(self, tokens):
        res = []
        self.p(tokens)
        return Apply(Map(lambda x: lambda y: [x] + y, self.p), Many(self.p))(tokens)


class Let(Parser):
    """
    satisfy let
    var <- symbol
    satisfy equal
    val <- expression
    satisyf in
    expr <- expression
    return Let(var, val, expr)

    satisfy let >> symbol >>= \var -> equal >> expression >>= \val -> satisfy in >> expression >>= \expr -> unit (let var val expr)
    """
    def run(self, tokens):
        let = Satisfy(lambda x: x.kind is TT.LET)
        var = Satisfy(lambda x: x.kind is TT.SYMBOL)
        eq = Satisfy(lambda x: x.kind is TT.EQUAL)
        val = Expression()
        IN = Satisfy(lambda x: x.kind is TT.IN)
        expr = Expression()
        p = Bind(Expression(), lambda z: Unit(Let(Variable(x.lexeme), y, z)))
        p = Then(IN, p)
        p = Bind(Expression(), lambda y: p)
        p = Then(eq, p)
        p = Bind(var, lambda x: p)
        p = Then(let, p)
        return p(tokens)

class Expression(Parser):
    def run(self, tokens):
        return Unit(None)(tokens)

# class Expression(Parser):
#     def run(self, tokens):
#         let = Let()
#         return let(tokens)
#
#
# class Let(Parser):
#     def run(self, tokens):
#         if tokens[0].kind is TT.LET or tokens[0].kind is TT.LETREC:
#             op = tokens[0]
#             tokens = tokens[1:]
#             if tokens[0].kind is TT.SYMBOL:
#                 var = tokens[0]
#                 tokens = tokens[1:]
#                 if tokens[0].kind is TT.EQUAL:
#                     tokens = tokens[1:]
#                     expr, tokens = parse_expression(copy.deepcopy(tokens))
#                     if tokens[0].kind is TT.IN:
#                         tokens = tokens[1:]
#                         expr2, tokens = parse_expression(copy.deepcopy(tokens))
#                         if op.kind is TT.LET:
#                             return Let(Variable(var.lexeme), expr, expr2), tokens
#                         else:
#                             return LetRec(Variable(var.lexeme), expr, expr2), tokens
#                     else:
#                         raise RuntimeError("Inner definition of let needs to end with 'in'.")
#                 else:
#                     raise RuntimeError("Symbol in let definition should be followed by an '='.")
#             else:
#                 raise RuntimeError("Let/letrec should be followed by a symbol.")
#         else:
#             lamb = Lambda()
#             return lamb(tokens)
#
#
# class Lambda(Parser):
#     def run(self, tokens):
#         pass
#
#
# class Application(Parser):
#     def run(self, tokens):
#         pass
#
#
# class Primary(Parser):
#     def run(self, tokens):
#         pass
