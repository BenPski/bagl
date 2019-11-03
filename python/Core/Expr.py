"""
The core language that gets evaluated

Comprised of lambda calculus in de bruijn notation
everything else is just convenience
"""

from abc import ABCMeta, abstractmethod
import copy

"""
Representation
"""


class Expr():
    """
    The expression tree object for lambda calculus in de bruijn form
    """
    __metaclass__ = ABCMeta

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.__dict__ == other.__dict__
        else:
            return False

    @abstractmethod
    def accept(self, visitor):
        pass


class Symbol(Expr):
    """
    An abstract named symbol
    It is just an atom with a name
    """

    def __init__(self, s):
        self.s = s

    def accept(self, visitor):
        return visitor.visitSymbol(self)


class Index(Expr):
    """
    Index in de bruijn notation
    """

    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitIndex(self)


class Lambda(Expr):
    """
    Lambda abstraction
    """

    def __init__(self, body):
        self.body = body

    def accept(self, visitor):
        return visitor.visitLambda(self)


class Apply(Expr):
    """
    Function application
    """

    def __init__(self, left, right):
        self.left = left
        self.right = right

    def accept(self, visitor):
        return visitor.visitApply(self)


class Bottom(Expr):
    """
    A bottom to represent non-terminating code
    """

    def accept(self, visitor):
        return visitor.visitBottom(self)


class TRUE(Expr):
    """
    Truthy value
    """

    def accept(self, visitor):
        return visitor.visitTrue(self)


class FALSE(Expr):
    """
    Falsy value
    """

    def accept(self, visitor):
        return visitor.visitFalse(self)


class If(Expr):
    """
    An if expression
    """

    def accept(self, visitor):
        return visitor.visitIf(self)


class Number(Expr):
    """
    number: double for now
    """

    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitNumber(self)


class Builtin(Expr):
    """
    Definitions for builtin functions

    Assumed strict in every argument
    """
    __metaclass__ = ABCMeta

    def accept(self, visitor):
        return visitor.visitBuiltin(self)

    @abstractmethod
    def func(self, args, spine):
        pass

    @abstractmethod
    def show(self):
        pass

    @property
    @abstractmethod
    def args(self):
        pass


class Add(Builtin):
    @property
    def args(self):
        return 2

    def func(self, args, spine):
        if isinstance(args[0], Number) and isinstance(args[1], Number):
            return Number(args[0].n + args[1].n)
        else:
            raise RuntimeError("Can only multiply numbers")

    def show(self):
        return "+"


class Mult(Builtin):
    @property
    def args(self):
        return 2

    def func(self, args, spine):
        if isinstance(args[0], Number) and isinstance(args[1], Number):
            return Number(args[0].n * args[1].n)
        else:
            raise RuntimeError("Can only multiply numbers")

    def show(self):
        return "*"


class Value(Expr):
    """
    Interface for builtin values
    """
    __metaclass__ = ABCMeta

    @property
    @abstractmethod
    def value(self):
        pass


# class String(Value):
#     def __init__(self, str):
#         self.value_internal = str
#
#     @property
#     def value(self):
#         return self.value_internal
#
# class Number(Value):
#     def __init__(self, n):
#         self.value_internal = n
#
#     def value(self):
#         return self.value_internal
#
# class Boolean(Value):
#     def __init__(self, val):
#         self.value_internal = val
#
#     def value(self):
#         return self.value_internal
#
# FALSE = Boolean("false")
# TRUE = Boolean("true")


"""
Visitor pattern
"""





"""
Functions for evaluating the expressions
"""

