"""
Use parser combinators to do the parsing

parse :: string -> [(a, string)]


What should the syntax be?

let variable = expr in expr
(\\ x . x)

for now all application is prefix

have to recognize
    let
    letrec
    lambdas
    application
    grouping (parenthesis)

highest precendence is grouping

let and letrec only fully defined with
let _ = _ in _
so it is let, symbol, equal, expression, in, expression

lambdas can have multiple arguments
(\\ x y z. body)
(, \\, [symbols], expression, )

application is adjacency
essentially is nothing else it is just application


expression -> lambda
let -> let/letrec symbol = expression in expression
lambdas -> \\ [symbols] . expression
application ->
primary -> Number | True | False | "(" expression ")"


application -> (Symbol | builtin) expression


expression -> let
let -> ("let" | "Letrec") Symbol "=" expression "in" expression | lambda
lambda -> "\\" [Symbol] "." expression | application
application -> primary [expression] | primary
primary -> Num | data | "(" expression ")"


screwed up a bit in the formalisms, no way to do "failure"

every result needs to return a list of parses
then every subsequent one maps over the previous result?

"""

from abc import ABCMeta, abstractmethod
import copy
from Parse.TokenType import TokenType as TT
from Representation.Expr import *


def empty(tokens):
    return []


def option(p, q, tokens):
    res = p(copy.deepcopy(tokens))
    if len(res) == 0:
        return q(copy.deepcopy(tokens))
    else:
        return res


def bind(p, f, tokens):
    res = p(copy.deepcopy(tokens))
    final = []
    for (a, s) in res:
        p = f(a)
        final += p(copy.deepcopy(s))
    return final


def take(tokens):
    return tokens[0], tokens[1:]


def match(tokens, val):
    if len(tokens) == 0:
        return False
    else:
        return tokens[0].kind is val


def matches(tokens, vals):
    if len(vals) == 0:
        return False
    elif match(tokens, vals[0]):
        return True
    else:
        return matches(tokens, vals[1:])


def parse(tokens):
    res = EXPRESSION(tokens)
    print(res)
    if len(res) == 1:
        expr, tokens = res[0]
        if len(tokens) == 0:
            return expr
        else:
            raise RuntimeError("Input not fully consumed.")
    else:
        raise RuntimeError("Some sort of parse error.")


def EXPRESSION(tokens):
    return LET(tokens)


def LET(tokens):
    return option(lambda x: parse_let(x), lambda x: LAMBDA(x), tokens)


def LAMBDA(tokens):
    return option(lambda x: parse_lambda(x), lambda x: APPLICATION(x), tokens)


def APPLICATION(tokens):
    return option(lambda x: parse_application(x), lambda x: PRIMARY(x), tokens)


def PRIMARY(tokens):
    return option(lambda x: parse_primary(x), lambda x: empty(x), tokens)


def parse_let(tokens):
    res = []
    if match(tokens, TT.LET) or match(tokens, TT.LETREC):
        op, tokens = take(tokens)
        if match(tokens, TT.SYMBOL):
            var, tokens = take(tokens)
            if match(tokens, TT.EQUAL):
                _, tokens = take(tokens)
                res_expr = EXPRESSION(tokens)
                # now want to bind it
                for (a, tokens) in res_expr:
                    if match(tokens, TT.IN):
                        _, tokens = take(tokens)
                        res_expr2 = EXPRESSION(tokens)
                        for (b, tokens) in res_expr2:
                            if op.kind is TT.LET:
                                res += [(Let(Variable(var.lexeme), a, b), copy.deepcopy(tokens))]
                            else:
                                res += [(LetRec(Variable(var.lexeme), a, b), copy.deepcopy(tokens))]
    return res


def parse_lambda(tokens):
    res = []
    if match(tokens, TT.BACKSLASH):
        _, tokens = take(tokens)
        args = []
        while match(tokens, TT.SYMBOL):
            arg, token = take(tokens)
            args.append(Variable(arg.lexeme))
        if len(args) == 0:
            raise RuntimeError("Lambda needs at least one argument in head that is a symbol.")
        if match(tokens, TT.DOT):
            _, tokens = take(tokens)
            res_body = EXPRESSION(tokens)
            for (body, tokens) in res_body:
                res += [(LambdaM(args, body), copy.deepcopy(tokens))]

    return res


def parse_application(tokens):
    """
    Application is
    (a b c) -> ((a b) c)
    Apply(Apply(a,b),c)

    if an expression is parsed and more can be parsed then it is an application
    if nothing more can be parsed it is a primary

    or just have to parse all arguments at once and use ApplyM

    while there are still tokens to parse use them in application

    can't properly do because any subsequent values gets interpreted as application
    """
    res = []
    if matches(tokens, [TT.SYMBOL, TT.STAR, TT.SLASH, TT.EQUAL_EQUAL, TT.IF, TT.MINUS]):
        res_func = parse_primary(tokens)
        for (func, tokens) in res_func:
            res += parse_application_help(func, copy.deepcopy(tokens))
    return res


def parse_application_help(expr, tokens):
    res_arg = EXPRESSION(copy.deepcopy(tokens))
    if len(res_arg) == 0:
        return [(expr, tokens)]
    else:
        res = []
        for (arg, tokens) in res_arg:
            res += parse_application_help(Apply(expr, arg), copy.deepcopy(tokens))
        return res


def parse_primary(tokens):
    if match(tokens, TT.SYMBOL):
        var, tokens = take(tokens)
        return [(Variable(var.lexeme), tokens)]
    elif match(tokens, TT.NUMBER):
        num, tokens = take(tokens)
        return [(Number(num.literal), tokens)]
    elif match(tokens, TT.STAR):
        _, tokens = take(tokens)
        return [(Mult(), tokens)]
    elif match(tokens, TT.MINUS):
        _, tokens = take(tokens)
        return [(Sub(), tokens)]
    elif match(tokens, TT.EQUAL_EQUAL):
        _, tokens = take(tokens)
        return [(Equal(), tokens)]
    elif match(tokens, TT.IF):
        _, tokens = take(tokens)
        return [(If(), tokens)]
    elif match(tokens, TT.LEFT_PAREN):
        res = []
        _, tokens = take(tokens)
        res_expr = EXPRESSION(copy.deepcopy(tokens))
        print(res_expr)
        for (expr, tokens) in res_expr:
            if match(tokens, TT.RIGHT_PAREN):
                _, tokens = take(tokens)
                res += [(Grouping(expr), copy.deepcopy(tokens))]
        return res
    else:
        return []
