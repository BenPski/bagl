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


class Core():
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


class Symbol(Core):
    """
    An abstract named symbol
    It is just an atom with a name
    """

    def __init__(self, s):
        self.s = s

    def accept(self, visitor):
        return visitor.visitSymbol(self)


class Index(Core):
    """
    Index in de bruijn notation
    """

    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitIndex(self)


class Lambda(Core):
    """
    Lambda abstraction
    """

    def __init__(self, body):
        self.body = body

    def accept(self, visitor):
        return visitor.visitLambda(self)


class Apply(Core):
    """
    Function application
    """

    def __init__(self, left, right):
        self.left = left
        self.right = right

    def accept(self, visitor):
        return visitor.visitApply(self)


class Bottom(Core):
    """
    A bottom to represent non-terminating code
    """

    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitBottom(self)


class TRUE(Core):
    """
    Truthy value
    """

    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitTrue(self)


class FALSE(Core):
    """
    Falsy value
    """

    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitFalse(self)


class If(Core):
    """
    An if expression
    """

    # def __init__(self, condition, branch1, branch2):
    #     self.condition = condition
    #     self.branch1 = branch1
    #     self.branch2 = branch2
    def __init__(self):
        pass

    def accept(self, visitor):
        return visitor.visitIf(self)


"""
Visitor pattern
"""


class CoreVisitor():
    """
    Visitor definition for the expression tree
    """
    __metaclass__ = ABCMeta

    def __call__(self, expr):
        return expr.accept(self)

    @abstractmethod
    def visitSymbol(self, elem):
        pass

    @abstractmethod
    def visitIndex(self, elem):
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


class CorePrint(CoreVisitor):
    def visitSymbol(self, elem):
        return elem.s

    def visitIndex(self, elem):
        return "#" + str(elem.n)

    def visitLambda(self, elem):
        return "(\\ " + self(elem.body) + ")"

    def visitApply(self, elem):
        return self(elem.left) + " " + self(elem.right)

    def visitBottom(self, elem):
        return "_|_"

    def visitTrue(self, elem):
        return "True"

    def visitFalse(self, elem):
        return "False"

    def visitIf(self, elem):
        # return "if " + self(elem.condition) + " then " + self(elem.branch1) + " else " + self(elem.branch2)
        return "if"


"""
Functions for evaluating the expressions
"""


def redex(expr, spine=None):
    if spine is None:
        spine = []
    if isinstance(expr, Apply):
        spine.append(expr.right)
        return redex(expr.left, spine)
    else:
        return expr, spine


def substitute(body, index, val):
    if isinstance(body, Apply):
        return Apply(substitute(body.left, index, val), substitute(body.right, index, val))
    elif isinstance(body, Lambda):
        return Lambda(substitute(body.body, Index(index.n + 1), val))
    elif isinstance(body, Index):
        if body.n == index.n:
            return val
        else:
            return body
    return body


def whnf(expr):
    expr_prev = expr
    top, spine = redex(expr_prev)
    expr, spine = whnf_step(top, spine)
    while expr != expr_prev:
        expr_prev = expr
        expr, spine = whnf_step(expr, spine)

    while len(spine) > 0:
        arg = spine.pop()
        expr = Apply(expr, arg)
    return expr


def whnf_step(expr, spine):
    if isinstance(expr, Bottom):
        raise RuntimeError("Evaluated bottom.")
    if isinstance(expr, Lambda):
        if len(spine) > 0:
            arg = spine.pop()
            expr = substitute(expr.body, Index(1), arg)
            return whnf_step(expr, spine)
    if isinstance(expr, If):
        # can either force if to have all 3 arguments present or can have it just test for boolean equality
        # currently just testing for equality
        if len(spine) >= 1:
            cond = spine.pop()
            cond, _ = whnf_step(cond, copy.deepcopy(spine))
            if isinstance(cond, TRUE):
                b1 = Lambda(Lambda(Index(2)))
                return whnf_step(b1, spine)
            elif isinstance(cond, FALSE):
                b2 = Lambda(Lambda(Index(1)))
                return whnf_step(b2, spine)
            else:
                raise RuntimeError("Condition in if statement is not a boolean")
    if isinstance(expr, Apply):  # a little weird to have it here
        expr, spine = redex(expr, spine)
        return whnf_step(expr, spine)

    return (expr, spine)
