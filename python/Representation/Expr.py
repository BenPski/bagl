from abc import ABCMeta, abstractmethod


class Expr(metaclass=ABCMeta):
    @abstractmethod
    def accept(self, visitor):
        pass


class Variable(Expr):
    def __init__(self, s):
        self.s = s

    def accept(self, visitor):
        return visitor.visitVariable(self)


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


class Bottom(Expr):
    def accept(self, visitor):
        return visitor.visitBottom(self)


class TRUE(Expr):
    def accept(self, visitor):
        return visitor.visitTRUE(self)


class FALSE(Expr):
    def accept(self, visitor):
        return visitor.visitFALSE(self)


class If(Expr):
    def accept(self, visitor):
        return visitor.visitIf(self)


class Number(Expr):
    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitNumber(self)


class Add(Expr):
    def accept(self, visitor):
        return visitor.visitAdd(self)


class Mult(Expr):
    def accept(self, visitor):
        return visitor.visitMult(self)
