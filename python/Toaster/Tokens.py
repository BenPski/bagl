from enum import Enum, auto


class Token():
    def __init__(self, kind, lexeme, line, column, literal=None):
        self.kind = kind
        self.lexeme = lexeme
        self.line = line
        self.column = column
        self.literal = literal

    def __str__(self):
        return 'Token: ' + self.lexeme + ' at ' + str(self.line) + ' ' + str(self.column)

    def __repr__(self):
        return self.__str__()


def grab(str):
    if len(str) >= 1:
        return str[0], str[1:]
    else:
        return '\0', ''


def peek(str, n=0):
    if len(str) >= n:
        return str[n]
    else:
        return '\0'


def isLeading(c):
    return not isNum(c) and isCharacter(c)


def isCharacter(c):
    return not (c in [' ', "\n", '\0', '\t', '(', ')', ',', '"', '{', '}', '[', ']', ';'])


def isAlpha(c):
    return 'a' <= c <= 'z' or 'A' <= c <= 'Z'


def isNum(c):
    return '0' <= c <= '9'


def isAlphaNum(c):
    return isAlpha(c) or isNum(c) or c == '_'




class TokenKind(Enum):
    L_Paren = auto()
    R_Paren = auto()
    Slash = auto()
    Dot = auto()
    Equal = auto()
    Let = auto()
    In = auto()
    Symbol = auto()
    Integer = auto()
    Float = auto()
    String = auto()
    Semicolon = auto()


