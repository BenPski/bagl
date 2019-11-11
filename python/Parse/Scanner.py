"""
Scanner to get all the tokens from a file
"""

from Parse.TokenType import TokenType as TT
from Parse.Token import Token

keywords = {'if': TT.IF, 'let': TT.LET, 'in': TT.IN, 'True': TT.TRUE, 'False': TT.FALSE}


class Scanner():
    def __init__(self, source):
        self.source = source
        self.tokens = []
        self.start = 0
        self.current = 0
        self.line = 1
        self.column = 1

    def scanTokens(self):
        while not self.atEnd():
            self.start = self.current
            self.scanToken()

        self.tokens.append(Token(TT.EOF, "", None, self.line, self.column))
        return self.tokens

    def atEnd(self):
        return self.current >= len(self.source)

    def scanToken(self):
        c = self.advance()
        if c == "(":
            self.addToken(TT.LEFT_PAREN)
        elif c == ")":
            self.addToken(TT.RIGHT_PAREN)
        elif c == ".":
            self.addToken(TT.DOT)
        elif c == "*":
            self.addToken(TT.STAR)
        elif c == "+":
            self.addToken(TT.PLUS)
        elif c == "-":
            self.addToken(TT.MINUS)
        elif c == "/":
            self.addToken(TT.SLASH)
        elif c == "\\":
            self.addToken(TT.BACKSLASH)
        elif c == "=":
            self.addToken(TT.EQUAL)
        elif c in [' ', '\r', '\t']:
            pass
        elif c == '\n':
            self.line += 1
            self.column = 1
        else:
            if self.isDigit(c):
                self.number()
            elif self.isAlpha(c):
                self.identifier()
            else:
                raise RuntimeError("Unrecognized character.")

    def advance(self):
        self.current += 1
        self.column += 1
        return self.source[self.current - 1]

    def match(self, c):
        if self.atEnd():
            return False
        if self.source[self.current] != c:
            return False
        self.current += 1
        self.column += 1
        return True

    def peek(self, n=1):
        if self.current + (n - 1) >= len(self.source):
            return "\0"
        return self.source[self.current + (n - 1)]

    def addToken(self, kind, literal=None):
        text = self.source[self.start:self.current]
        self.tokens.append(Token(kind, text, literal, self.line, self.column))

    def isDigit(self, c):
        return '9' >= c >= '0'

    def number(self):
        while self.isDigit(self.peek()):
            self.advance()
        if self.peek() == "." and self.isDigit(self.peek(2)):
            self.advance()
            while self.isDigit(self.peek()):
                self.advance()
        self.addToken(TT.NUMBER, float(self.source[self.start:self.current]))

    def identifier(self):
        while self.isAlphaNum(self.peek()):
            self.advance()
        text = self.source[self.start:self.current]
        if text in keywords:
            self.addToken(keywords[text])
        else:
            self.addToken(TT.SYMBOL)

    def isAlpha(self, c):
        return ('a' <= c <= 'z') or ('A' <= c <= 'Z') or c == '_'

    def isAlphaNum(self, c):
        return self.isAlpha(c) or self.isDigit(c)
