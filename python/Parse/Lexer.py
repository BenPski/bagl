"""
Scans the source and collects the tokens to get the lexical representation of the source code

The primary functions are advance and peek

Generally language looks like

let
x = y
in z

only define things with let's for now
whitespace makes no difference
multi-argument lambdas are the only functions

the lexer iterates over the source looking for tokens
when finding a token it stores its data and holds onto its position in the source file as well for error reporting

lex [] = EOF
lex (c:cs) = let t = case c of
                        blah
             in t:(lex cs)

"""

from Parse.TokenType import TokenType as TT
from Parse.Token import Token

keywords = {'if': TT.IF, 'let': TT.LET, 'in': TT.IN, 'True': TT.TRUE, 'False': TT.FALSE}


def lex(string, tokens=None, line=1, column=1):
    # initialize tokens
    if tokens is None:
        tokens = []

    # got to end of source
    if len(string) == 0:
        return tokens

    # determine next token
    c = string[0]
    rest = string[1:]
    token = None
    if c == "(":
        token = Token(TT.LEFT_PAREN, c, None, line, column)
    elif c == ")":
        token = Token(TT.RIGHT_PAREN, c, None, line, column)
    elif c == ".":
        token = Token(TT.DOT, c, None, line, column)
    elif c == "*":
        token = Token(TT.STAR, c, None, line, column)
    elif c == "+":
        token = Token(TT.PLUS, c, None, line, column)
    elif c == "-":
        token = Token(TT.MINUS, c, None, line, column)
    elif c == "/":
        token = Token(TT.SLASH, c, None, line, column)
    elif c == "\\":
        token = Token(TT.BACKSLASH, c, None, line, column)
    elif c == "=":
        if peek(rest) == "=":
            c += "="
            rest = rest[1:]
            token = Token(TT.EQUAL_EQUAL, c, None, line, column)
        else:
            token = Token(TT.EQUAL, c, None, line, column)
    elif c in [' ', '\r']:
        column += 1
    elif c == '\n':
        line += 1
        column = 1
    else:
        if isDigit(c):
            str_num = c
            while isDigit(peek(rest)):
                str_num += rest[0]
                rest = rest[1:]
            if peek(rest) == "." and isDigit(peek(rest, 1)):
                str_num += rest[0]
                rest = rest[1:]
                while isDigit(peek(rest)):
                    str_num += rest[0]
                    rest = rest[1:]
            token = Token(TT.NUMBER, str_num, float(str_num), line, column)
        elif isAlpha(c):
            str_ident = c
            while isAlphaNum(peek(rest)):
                str_ident += rest[0]
                rest = rest[1:]
            if str_ident in keywords:
                token = Token(keywords[str_ident], str_ident, None, line, column)
            else:
                token = Token(TT.SYMBOL, str_ident, None, line, column)
        else:
            raise RuntimeError("Unrecognized character.")

    # found a token? add it
    if token is not None:
        tokens.append(token)

    return lex(rest, tokens, line, column)


def peek(s, n=0):
    if n + 1 > len(s):
        return '\0'
    else:
        return s[n]


def isDigit(c):
    return '9' >= c >= '0'


def isAlpha(c):
    return ('a' <= c <= 'z') or ('A' <= c <= 'Z') or c == '_'


def isAlphaNum(c):
    return isDigit(c) or isAlpha(c)
