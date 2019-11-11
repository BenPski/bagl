"""
Tokens for parsing
"""

from enum import Enum, auto


class TokenType(Enum):
    # single character
    LEFT_PAREN = auto()
    RIGHT_PAREN = auto()
    DOT = auto()
    SLASH = auto()
    STAR = auto()
    PLUS = auto()
    MINUS = auto()
    BACKSLASH = auto()
    EQUAL = auto()

    # operator
    EQUAL_EQUAL = auto()

    # literals
    SYMBOL = auto()
    NUMBER = auto()

    # keywords
    IF = auto()
    TRUE = auto()
    FALSE = auto()
    LET = auto()
    IN = auto()

    EOF = auto()
