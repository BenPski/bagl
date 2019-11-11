"""
The core language that gets evaluated

Comprised of the lambda calculus

removing de bruijn notation stuff
For some reason there is an issue with recursion that I'm not aware of
also in a program with no free variables de bruijn notation is not important
"""

from abc import ABCMeta, abstractmethod
import copy
from Core.Print import Print

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

    def __str__(self):
        printer = Print()
        return printer(self)


class Variable(Expr):
    """
    An abstract named symbol
    It is just an atom with a name
    """

    def __init__(self, s):
        self.s = s

    def accept(self, visitor):
        return visitor.visitVariable(self)


class Lambda(Expr):
    """
    Lambda abstraction
    """

    def __init__(self, head, body):
        self.head = head
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


class Seq(Expr):
    """
    Force first argument to whnf then return second argument
    """

    def accept(self, visitor):
        return visitor.visitSeq(self)


class Number(Expr):
    """
    number: double for now
    """

    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitNumber(self)


class Data(Expr):
    """
    Generalized data constructor

    Very similar to builtins except it stores information and has an associated type
    """

    __metaclass__ = ABCMeta

    def __init__(self):
        self.initialized = False
        self.values = None

    def accept(self, visitor):
        return visitor.visitData(self)

    @abstractmethod
    def show(self):
        pass

    @property
    @abstractmethod
    def fields(self):
        pass

    @property
    @abstractmethod
    def type(self):
        pass

    def construct(self, fields, spine):
        self.values = fields
        self.initialized = True


class Nil(Data):
    def show(self):
        return "Nil"

    @property
    def fields(self):
        return 0

    @property
    def type(self):
        return "List"


class Cons(Data):
    def show(self):
        if self.initialized:
            return "(Cons " + str(self.values[0]) + " " + str(self.values[1]) + ")"
        else:
            return "Cons"

    @property
    def fields(self):
        return 2

    @property
    def type(self):
        return "List"


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
            raise RuntimeError("Can only add numbers")

    def show(self):
        return "+"


class Sub(Builtin):
    @property
    def args(self):
        return 2

    def func(self, args, spine):
        if isinstance(args[0], Number) and isinstance(args[1], Number):
            return Number(args[0].n - args[1].n)
        else:
            raise RuntimeError("Can only subtract numbers")

    def show(self):
        return "-"


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


class Equal(Builtin):
    @property
    def args(self):
        return 2

    def func(self, args, spine):
        if isinstance(args[0], Number) and isinstance(args[1], Number):
            if args[0].n == args[1].n:
                return TRUE()
            else:
                return FALSE()
        else:
            raise RuntimeError("Equality between numbers only.")

    def show(self):
        return "=="


class Head(Builtin):
    @property
    def args(self):
        return 1

    def show(self):
        return "head"

    def func(self, args, spine):
        if isinstance(args[0], Data) and args[0].type == "List":
            if isinstance(args[0], Cons):
                return args[0].values[0]
            else:
                raise RuntimeError("Empty list has no head")
        else:
            raise RuntimeError("Head only works on lists")


class Tail(Builtin):
    @property
    def args(self):
        return 1

    def show(self):
        return "tail"

    def func(self, args, spine):
        if isinstance(args[0], Data) and args[0].type == "List":
            if isinstance(args[0], Cons):
                return args[0].values[1]
            else:
                raise RuntimeError("Empty list has no tail")
        else:
            raise RuntimeError("Tail only works on lists")


class Value(Expr):
    """
    Interface for builtin values
    """
    __metaclass__ = ABCMeta

    @property
    @abstractmethod
    def value(self):
        pass
