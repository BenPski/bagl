"""

Used for generating the expression trees and the visitors to eliminate the tedium of it

Easier to automatically generate the representation expression and visitor, but the builtins confound the core expressoin a bit

"""

"""
For the lambda calculus representation generate expr and visitor in separate files
"""


class Node():
    def __init__(self, name, args=None):
        self.name = name
        if args is None:
            self.args = []
        else:
            self.args = args

    def definition(self):
        """
        return string representation for putting in file
        """
        s = "class " + self.name + "(Expr):\n"
        if len(self.args) != 0:
            s += "    def __init__(self, " + ", ".join(self.args) + "):\n"
            for arg in self.args:
                s += "        self." + arg + " = " + arg + "\n"
            s += "\n"
        s += "    def accept(self, visitor):\n"
        s += "        return visitor.visit" + self.name + "(self)\n"
        return s

    def visit(self):
        """
        visitor defintion function
        """
        return "    @abstractmethod\n    def visit" + self.name + "(self, elem):\n        pass\n"


def expr(nodes):
    s = "from abc import ABCMeta, abstractmethod\n\n\n"
    s += "class Expr(metaclass=ABCMeta):\n"
    s += "    @abstractmethod\n"
    s += "    def accept(self, visitor):\n"
    s += "        pass\n"
    for node in nodes:
        s += "\n\n"
        s += node.definition()
    return s


def visitor(nodes):
    s = "from abc import ABCMeta, abstractmethod\n\n\n"
    s += "class Visitor(metaclass=ABCMeta):\n"
    s += "    def __call__(self, expr):\n"
    s += "        return expr.accept(self)\n"
    for node in nodes:
        s += "\n"
        s += node.visit()
    return s


if __name__ == "__main__":
    nodes = [Node("Variable", ["s"]),
             Node("Lambda", ["head", "body"]),
             Node("LambdaM", ["args", "body"]),
             Node("Apply", ["left", "right"]),
             Node("ApplyM", ["func", "values"]),
             Node("Bottom"),
             Node("TRUE"),
             Node("FALSE"),
             Node("If"),
             Node("Seq"),
             Node("Number", ["n"]),
             Node("Add"),
             Node("Sub"),
             Node("Mult"),
             Node("Equal"),
             Node("Let", ["var", "val", "expr"]),
             Node("LetRec", ["var", "val", "expr"]),
             Node("Nil"),
             Node("Cons"),
             Node("Head"),
             Node("Tail")]
    with open("Representation/Expr.py", 'w') as f:
        f.write(expr(nodes))
    with open("Representation/Visitor.py", 'w') as f:
        f.write(visitor(nodes))
