from abc import ABCMeta, abstractmethod


class Expr(metaclass=ABCMeta):
    @abstractmethod
    def accept(self, visitor):
        pass


class Lambda(Expr):
    def __init__(self, head, body):
        self.head = head
        self.body = body

    def accept(self, visitor):
        return visitor.visitLambda(self)


class Apply(Expr):
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def accept(self, visitor):
        return visitor.visitApply(self)


class Let(Expr):
    def __init__(self, vars, vals, expr):
        self.vars = vars
        self.vals = vals
        self.expr = expr

    def accept(self, visitor):
        return visitor.visitLet(self)


class Group(Expr):
    def __init__(self, expr):
        self.expr = expr

    def accept(self, visitor):
        return visitor.visitGroup(self)


class Variable(Expr):
    def __init__(self, s):
        self.s = s

    def accept(self, visitor):
        return visitor.visitVariable(self)


class Integer(Expr):
    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitInteger(self)


class Float(Expr):
    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitFloat(self)


class String(Expr):
    def __init__(self, s):
        self.s = s

    def accept(self, visitor):
        return visitor.visitString(self)


class Visitor(metaclass=ABCMeta):
    def __call__(self,expr):
        return expr.accept(self)

    @abstractmethod
    def visitLambda(self, elem):
        pass

    @abstractmethod
    def visitApply(self, elem):
        pass

    @abstractmethod
    def visitLet(self, elem):
        pass

    @abstractmethod
    def visitGroup(self, elem):
        pass

    @abstractmethod
    def visitVariable(self, elem):
        pass

    @abstractmethod
    def visitInteger(self, elem):
        pass

    @abstractmethod
    def visitFloat(self, elem):
        pass

    @abstractmethod
    def visitString(self, elem):
        pass
