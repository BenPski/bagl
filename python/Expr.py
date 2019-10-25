from abc import ABCMeta, abstractmethod
from ExprDB import *
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


class Subs(Expr):
    """
    A substitution node
    [E:x\y]
    """

    def __init__(self, expr, var, val):
        self.expr = expr
        self.var = var
        self.val = val

    def accept(self, visitor):
        return visitor.visitSubs(self)


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
    def visitSubs(self, elem):
        pass


class ExprPrint(ExprVisitor):
    def visitVariable(self, elem):
        return elem.s

    def visitLambda(self, elem):
        return "(\\" + self(elem.head) + "." + self(elem.body) + ")"

    def visitApply(self, elem):
        return self(elem.left) + " " + self(elem.right)

    def visitSubs(self, elem):
        return "[ " + self(elem.expr) + " | " + self(elem.var) + " -> " + self(elem.val) + " ]"


class ExprToExprDB(ExprVisitor):
    """
    Convert regular lambda calculus expressions to de bruijn notation

    have to keep track of associated indices for variables so modify some internal state

    just using a dictionary :: variable -> index
    update indices when going down one level in a lambda
    assume no shadowing of names
    if variable not found just leave it as an atom

    when encountering a lambda have to initialize a new index in the environment
    """

    def __init__(self):
        self.env = {}

    def visitVariable(self, elem):
        if elem.s in self.env:
            return IndexDB(self.env[elem.s])
        else:
            return VariableDB(elem.s)

    def visitLambda(self, elem):
        s = elem.head.s
        self.env[s] = 0
        for key in self.env.keys():
            self.env[key] += 1
        return LambdaDB(self(elem.body))

    def visitApply(self, elem):
        self_copy = copy.deepcopy(self)
        return ApplyDB(self_copy(elem.left), self_copy(elem.right))

    def visitSubs(self, elem):
        self_copy = copy.deepcopy(self)
        return SubsDB(self_copy(elem.expr), self_copy(elem.var), self_copy(elem.val))
