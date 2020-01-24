"""
Want to parse multiple lines in a let block 
    have to have at least one definition

currently ignoring newlines, need to consider them significant
    may cause some issues in some of the patterns with recursive ascent

"""

from Toaster.Expr import *
from Toaster.Tokens import *

keywords = {
    '.': TokenKind.Dot,
    '=': TokenKind.Equal,
    '\\': TokenKind.Slash,
    'in': TokenKind.In,
    'let': TokenKind.Let,
}


def lex(src, line=0, column=0, tokens=None):
    if tokens is None:
        tokens = []
    if len(src) == 0 or src[0] == '\0':
        return tokens
    c, rest = grab(src)
    if c == '(':
        tokens.append(Token(TokenKind.L_Paren, '(', line, column))
        return lex(rest, line, column + 1, tokens)
    if c == ')':
        tokens.append(Token(TokenKind.R_Paren, ')', line, column))
        return lex(rest, line, column + 1, tokens)
    if c == ';':
        tokens.append(Token(TokenKind.Semicolon, ';', line, column))
        return lex(rest, line, column + 1, tokens)
    if c == ' ':
        return lex(rest, line, column + 1, tokens)
    if c == '\n':
        return lex(rest, line + 1, 0, tokens)
    if isLeading(c):
        sym = c
        next_col = column
        c, rest = grab(rest)
        while isCharacter(c):
            sym += c
            next_col += 1
            c, rest = grab(rest)
        if sym in keywords:
            tokens.append(Token(keywords[sym], sym, line, column))
        else:
            tokens.append(Token(TokenKind.Symbol, sym, line, column, sym))
        return lex(c + rest, line, next_col + 1, tokens)
    if c == '"':
        sym = ''
        next_col = column
        next_line = line
        c, rest = grab(rest)
        while c != '"':
            sym += c
            if c == '\n':
                next_line += 1
                next_col = 0
            else:
                next_col += 1
            c, rest = grab(rest)
        tokens.append(Token(TokenKind.String, sym, line, column, sym))
        return lex(rest, next_line, next_col + 1, tokens)
    if isNum(c):
        num = c
        next_col = column
        c, rest = grab(rest)
        while isNum(c):
            num += c
            next_col += 1
            c, rest = grab(rest)
        if c == '.':
            num += c
            c, rest = grab(rest)
            while isNum(c):
                num += c
                next_col += 1
                c, rest = grab(rest)
            tokens.append(Token(TokenKind.Float, num, line, column, float(num)))
            return lex(c + rest, line, next_col + 1, tokens)
        else:
            tokens.append(Token(TokenKind.Integer, num, line, column, int(num)))
            return lex(c + rest, line, next_col + 1, tokens)


def multilambda(stack):
    # check if it looks like a multi-argument lambda
    if len(stack) >= 4 and isinstance(stack[-1], Expr) and isinstance(stack[-2], Token) and stack[
        -2].kind is TokenKind.Dot:
        # check for multiple arguments
        i = 0
        while len(stack) >= 4 + i and isinstance(stack[-3 - i], Variable):
            i += 1
        return (isinstance(stack[-3 - i], Token) and stack[-3 - i].kind is TokenKind.Slash)
    return False


def parse_lambda(stack):
    expr = stack.pop()
    stack.pop()
    while isinstance(stack[-1], Variable):
        v = stack.pop()
        expr = Lambda(v, expr)
    stack.pop()
    stack.append(expr)
    return stack


def application(stack):
    # since application is a bit ambiguous as far as how parsing goes need to be a bit more thorough with checking
    # e.g., arguments in a multi-argument lambda look like application
    # for now just check that there is no proceeding slash
    if len(stack) >= 2 and isinstance(stack[-1], Expr) and isinstance(stack[-2], Expr):
        i = 0
        while len(stack) > 2 + i and isinstance(stack[-3 - i], Expr):
            i += 1
        return not (isinstance(stack[-3 - i], Token) and stack[-3 - i].kind is TokenKind.Slash)
    else:
        return False


def letblock(stack):
    # check if the stack looks like a let block
    # Let (defintion)* In Expr
    if len(stack) >= 7 and isinstance(stack[-1], Expr) and isinstance(stack[-2], Token) and stack[
        -2].kind is TokenKind.In:
        # ends correctly, look for defintions
        i = 0
        while len(stack) >= 4 * (i + 1) + 3 and isinstance(stack[-4 - 4 * i], Expr) and isinstance(stack[-5 - 4 * i],
                                                                                                   Token) and stack[
            -5 - 4 * i].kind is TokenKind.Equal and isinstance(stack[-6 - 4 * i], Expr) and isinstance(
            stack[-3 - 4 * i], Token) and stack[-3 - 4 * i].kind is TokenKind.Semicolon:
            i += 1
        # i defintions
        return isinstance(stack[-3 - 4 * i], Token) and stack[-3 - 4 * i].kind is TokenKind.Let

    return False


def parse_let(stack):
    vars = []
    vals = []
    expr = stack.pop()
    stack.pop()
    while not (isinstance(stack[-1], Token) and stack[-1].kind is TokenKind.Let):
        stack.pop()
        val = stack.pop()
        stack.pop()
        # kind of a hacky way of turning f a b = ... => f = \ a . (\ b . ...)
        var = stack.pop()
        while isinstance(var, Apply):
            arg = var.right
            var = var.left
            val = Lambda(arg, val)
        vars.append(var)
        vals.append(val)
    stack.pop()
    stack.append(Let(vars, vals, expr))
    return stack


def parse(tokens):
    stack = []
    while len(tokens) > 0 or len(stack) > 1 or not isinstance(stack[0], Expr):
        shift = True
        if len(stack) >= 4 and isinstance(stack[-4], Token) and stack[-4].kind is TokenKind.Slash and isinstance(
                stack[-3], Variable) and isinstance(stack[-2], Token) and stack[
            -2].kind is TokenKind.Dot and isinstance(stack[-1], Expr):
            shift = False
            v3 = stack.pop()
            v2 = stack.pop()
            v1 = stack.pop()
            v0 = stack.pop()
            stack.append(Lambda(v1, v3))
        if multilambda(stack):
            shift = False
            stack = parse_lambda(stack)
        # if len(stack) >= 2 and isinstance(stack[-2], Expr) and isinstance(stack[-1], Expr):
        if application(stack):
            shift = False
            v1 = stack.pop()
            v0 = stack.pop()
            stack.append(Apply(v0, v1))
        if letblock(stack):
            shift = False
            stack = parse_let(stack)
        if len(stack) >= 6 and isinstance(stack[-6], Token) and stack[-6].kind is TokenKind.Let and isinstance(
                stack[-5], Variable) and isinstance(stack[-4], Token) and stack[
            -4].kind is TokenKind.Equal and isinstance(stack[-3], Expr) and isinstance(stack[-2], Token) and stack[
            -2].kind is TokenKind.In and isinstance(stack[-1], Expr):
            shift = False
            v5 = stack.pop()
            v4 = stack.pop()
            v3 = stack.pop()
            v2 = stack.pop()
            v1 = stack.pop()
            v0 = stack.pop()
            stack.append(Let([v1], [v3], v5))
        if len(stack) >= 3 and isinstance(stack[-3], Token) and stack[-3].kind is TokenKind.L_Paren and isinstance(
                stack[-2], Expr) and isinstance(stack[-1], Token) and stack[-1].kind is TokenKind.R_Paren:
            shift = False
            v2 = stack.pop()
            v1 = stack.pop()
            v0 = stack.pop()
            stack.append(Group(v1))
        if len(stack) >= 1 and isinstance(stack[-1], Token) and stack[-1].kind is TokenKind.Symbol:
            shift = False
            v0 = stack.pop()
            stack.append(Variable(v0.lexeme))
        if len(stack) >= 1 and isinstance(stack[-1], Token) and stack[-1].kind is TokenKind.Integer:
            shift = False
            v0 = stack.pop()
            stack.append(Integer(v0.literal))
        if len(stack) >= 1 and isinstance(stack[-1], Token) and stack[-1].kind is TokenKind.Float:
            shift = False
            v0 = stack.pop()
            stack.append(Float(v0.literal))
        if len(stack) >= 1 and isinstance(stack[-1], Token) and stack[-1].kind is TokenKind.String:
            shift = False
            v0 = stack.pop()
            stack.append(String(v0.literal))
        if shift and len(tokens) == 0:
            raise RuntimeError('Some kind of parse error. The stack:', stack)
        if shift:
            stack.append(tokens[0])
            tokens = tokens[1:]
    return stack[0]


def read(s):
    return parse(lex(s))
