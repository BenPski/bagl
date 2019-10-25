from abc import ABCMeta, abstractmethod
import copy


class ExprDB():
    """
    The expression tree object for lambda calculus in de bruijn form
    """
    __metaclass__ = ABCMeta

    @abstractmethod
    def accept(self, visitor):
        pass


class VariableDB(ExprDB):
    """
    Holds a name for a symbol
    """

    def __init__(self, s):
        self.s = s

    def accept(self, visitor):
        return visitor.visitVariable(self)


class IndexDB(ExprDB):
    """
    Index in de bruijn notation
    """

    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitIndex(self)


class LambdaDB(ExprDB):
    """
    Lambda abstraction
    """

    def __init__(self, body):
        self.body = body

    def accept(self, visitor):
        return visitor.visitLambda(self)


class ApplyDB(ExprDB):
    """
    Function application
    """

    def __init__(self, left, right):
        self.left = left
        self.right = right

    def accept(self, visitor):
        return visitor.visitApply(self)


class SubsDB(ExprDB):
    """
    A substitution node
    [E:x\y]
    """

    def __init__(self, expr, index, val):
        self.expr = expr
        self.index = index
        self.val = val

    def accept(self, visitor):
        return visitor.visitSubs(self)


class ExprDBVisitor():
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
    def visitIndex(self, elem):
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


class ExprDBPrint(ExprDBVisitor):
    def visitVariable(self, elem):
        return elem.s

    def visitIndex(self, elem):
        return "#" + str(elem.n)

    def visitLambda(self, elem):
        return "(\\ " + self(elem.body) + ")"

    def visitApply(self, elem):
        return self(elem.left) + " " + self(elem.right)

    def visitSubs(self, elem):
        return "[ " + self(elem.expr) + " | " + self(elem.index) + " -> " + self(elem.val) + " ]"


class OutermostRedex(ExprDBVisitor):
    """
    Find the expression that constitutes the outermost redex
    essentially traverse all left sides of applies
    """

    def visitApply(self, elem):
        return self(elem.left)

    def visitIndex(self, elem):
        return elem  # really shouldn't be happening

    def visitLambda(self, elem):
        return elem

    def visitSubs(self, elem):
        return elem  # shouldn't ever encounter

    def visitVariable(self, elem):
        return elem


class AccumArguments(ExprDBVisitor):
    """
    Accumulates the arguments that are applied to the outermost redex
    """

    def __init__(self):
        self.args = []

    def visitVariable(self, elem):
        return self.args

    def visitSubs(self, elem):
        return self.args

    def visitLambda(self, elem):
        return self.args

    def visitIndex(self, elem):
        return self.args

    def visitApply(self, elem):
        self.args.append(elem.right)
        return self(elem.left)


class Eval(ExprDBVisitor):
    """
    Track to outer redex, maintain stack of arguments, evaluate

    If arriving at something that cannot be evaluated unwind the applies
    """

    def __init__(self):
        self.args = []

    def unwrap(self, elem):
        while len(self.args) > 0:
            elem = ApplyDB(elem, self.args.pop())
        return elem

    def visitVariable(self, elem):
        return self.unwrap(elem)

    def visitSubs(self, elem):
        if isinstance(elem.expr, IndexDB):
            if elem.expr.n == elem.index.n:
                return self(elem.val)
            else:
                return self(elem.expr)
        elif isinstance(elem.expr, LambdaDB):
            return self(LambdaDB(self(SubsDB(elem.expr.body, IndexDB(elem.index.n + 1), elem.val))))
        elif isinstance(elem.expr, ApplyDB):
            return ApplyDB(self(SubsDB(elem.expr.left, elem.index, elem.val)),
                           self(SubsDB(elem.expr.right, elem.index, elem.val)))
        elif isinstance(elem.expr, SubsDB):
            return SubsDB(self(elem.expr), elem.index, elem.val)
        else:
            return self(elem.expr)

    def visitLambda(self, elem):
        if len(self.args) >= 1:
            arg = self.args.pop()
            return self(SubsDB(elem.body, IndexDB(1), arg))
        else:
            return elem

    def visitIndex(self, elem):
        return elem

    def visitApply(self, elem):
        if isinstance(elem.left, VariableDB):
            return self.unwrap(elem)
        else:
            next = Eval()
            next.args.append(elem.right)
            return self(next(elem.left))
