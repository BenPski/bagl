"""
The tokens
"""


class Token():
    def __init__(self, kind, lexeme, literal, line, column):
        self.kind = kind  # token type
        self.lexeme = lexeme
        self.literal = literal
        self.line = line
        self.column = column

    def __str__(self):
        return str(self.kind) + " " + self.lexeme + " " + str(self.literal)
