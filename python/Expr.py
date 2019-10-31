"""
Lambda calculus representation for easier expression writing and then conversion to core
"""

from abc import ABCMeta, abstractmethod
import Core as C
import copy


class Expr():
    """
    The expression tree object for lambda calculus
    """
    __metaclass__ = ABCMeta

    @abstractmethod
    def accept(self, visitor):
        pass


class Variable(Expr):
    """
    Holds a name for a symbol
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
    Non-terminating node
    """

    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitBottom(self)


class TRUE(Expr):
    """
    truthy value
    """

    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitTrue(self)


class FALSE(Expr):
    """
    falsy value
    """

    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitFalse(self)


class If(Expr):
    """
    If condition
    """

    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitIf(self)


class Number(Expr):
    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitNumber(self)

class Add(Expr):
    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitAdd(self)

class Mult(Expr):
    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitMult(self)


"""
Visitor Pattern
"""


class ExprVisitor():
    """
    Visitor definition for the expression tree
    """
    __metaclass__ = ABCMeta

    def __call__(self, expr):
        return expr.accept(self)

    @abstractmethod
    def visitVariable(self, elem):
        pass

    @abstractmethod
    def visitLambda(self, elem):
        pass

    @abstractmethod
    def visitApply(self, elem):
        pass

    @abstractmethod
    def visitBottom(self, elem):
        pass

    @abstractmethod
    def visitTrue(self, elem):
        pass

    @abstractmethod
    def visitFalse(self, elem):
        pass

    @abstractmethod
    def visitIf(self, elem):
        pass

    @abstractmethod
    def visitNumber(self, elem):
        pass

    @abstractmethod
    def visitAdd(self, elem):
        pass

    @abstractmethod
    def visitMult(self, elem):
        pass


class ExprPrint(ExprVisitor):
    def visitVariable(self, elem):
        return elem.s

    def visitLambda(self, elem):
        return "(\\" + self(elem.head) + "." + self(elem.body) + ")"

    def visitApply(self, elem):
        return self(elem.left) + " " + self(elem.right)

    def visitBottom(self, elem):
        return "_|_"

    def visitFalse(self, elem):
        return "False"

    def visitTrue(self, elem):
        return "True"

    def visitIf(self, elem):
        return "if "


class ExprToCore(ExprVisitor):
    """
    Convert regular lambda calculus expressions to de bruijn notation

    have to keep track of associated indices for variables so modify some internal state

    just using a dictionary :: variable -> index
    update indices when going down one level in a lambda
    assume no shadowing of names
    if variable not found just leave it as an atom (means it is a free variable)

    when encountering a lambda have to initialize a new index in the environment
    """

    def __init__(self):
        self.env = {}

    def visitVariable(self, elem):
        if elem.s in self.env:
            return C.Index(self.env[elem.s])
        else:
            return C.Symbol(elem.s)

    def visitLambda(self, elem):
        s = elem.head.s
        self.env[s] = 0
        for key in self.env.keys():
            self.env[key] += 1
        return C.Lambda(self(elem.body))

    def visitApply(self, elem):
        self_copy = copy.deepcopy(self)
        return C.Apply(self_copy(elem.left), self_copy(elem.right))

    def visitBottom(self, elem):
        return C.Bottom()

    def visitFalse(self, elem):
        return C.FALSE()

    def visitTrue(self, elem):
        return C.TRUE()

    def visitIf(self, elem):
        return C.If()

    def visitNumber(self, elem):
        return C.Number(elem.n)

    def visitAdd(self, elem):
        return C.Add()

    def visitMult(self, elem):
        return C.Mult()
