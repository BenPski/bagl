from abc import ABCMeta, abstractmethod


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


class ExprDBWHNF(ExprDBVisitor):
    """
    Evaluate to weak head normal form

    essentially if an apply can be done it should be done
    Apply(Lambda(blah), value) -> substitute(blah.body, blah.head, value)
    """

    def visitVariable(self, elem):
        return elem

    def visitIndex(self, elem):
        return elem

    def visitLambda(self, elem):
        return elem

    def visitApply(self, elem):
        if isinstance(elem.left, LambdaDB):
            return self(SubsDB(elem.left.body, IndexDB(1), elem.right))
        else:
            return self(ApplyDB(self(elem.left), elem.right))

    def visitSubs(self, elem):
        if isinstance(elem.expr, IndexDB):
            if elem.expr.n == elem.index.n:
                return self(elem.val)
            else:
                return self(elem.expr)
        elif isinstance(elem.expr, LambdaDB):
            return self(LambdaDB(self(SubsDB(elem.expr.body, IndexDB(elem.index.n + 1), elem.val))))
        elif isinstance(elem.expr, ApplyDB):
            return self(ApplyDB(self(SubsDB(elem.expr.left, elem.index, elem.val)),
                                self(SubsDB(elem.expr.right, elem.index, elem.val))))
        return elem.expr
