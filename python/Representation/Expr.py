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


class LambdaM(Expr):
    def __init__(self, args, body):
        self.args = args
        self.body = body

    def accept(self, visitor):
        return visitor.visitLambdaM(self)


class Apply(Expr):
    def __init__(self, left, right):
        self.left = left
        self.right = right

    def accept(self, visitor):
        return visitor.visitApply(self)


class ApplyM(Expr):
    def __init__(self, func, values):
        self.func = func
        self.values = values

    def accept(self, visitor):
        return visitor.visitApplyM(self)


class Grouping(Expr):
    def __init__(self, expr):
        self.expr = expr

    def accept(self, visitor):
        return visitor.visitGrouping(self)


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


class Seq(Expr):
    def accept(self, visitor):
        return visitor.visitSeq(self)


class Number(Expr):
    def __init__(self, n):
        self.n = n

    def accept(self, visitor):
        return visitor.visitNumber(self)


class Add(Expr):
    def accept(self, visitor):
        return visitor.visitAdd(self)


class Sub(Expr):
    def accept(self, visitor):
        return visitor.visitSub(self)


class Mult(Expr):
    def accept(self, visitor):
        return visitor.visitMult(self)


class Equal(Expr):
    def accept(self, visitor):
        return visitor.visitEqual(self)


class Let(Expr):
    def __init__(self, var, val, expr):
        self.var = var
        self.val = val
        self.expr = expr

    def accept(self, visitor):
        return visitor.visitLet(self)


class LetRec(Expr):
    def __init__(self, var, val, expr):
        self.var = var
        self.val = val
        self.expr = expr

    def accept(self, visitor):
        return visitor.visitLetRec(self)


class Nil(Expr):
    def accept(self, visitor):
        return visitor.visitNil(self)


class Cons(Expr):
    def accept(self, visitor):
        return visitor.visitCons(self)


class Head(Expr):
    def accept(self, visitor):
        return visitor.visitHead(self)


class Tail(Expr):
    def accept(self, visitor):
        return visitor.visitTail(self)
