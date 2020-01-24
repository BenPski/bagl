from Representation.Visitor import Visitor
import copy
import Core.Expr as C
from Representation.Expr import *

# Y combinator
f = Variable("f")
x = Variable("x")
Y = Lambda(f, Apply(Lambda(x, Apply(x, x)), Lambda(x, Apply(f, Apply(x, x)))))

class RightToLeftApply(Visitor):
    """
    The parser switches the order of application to be right to left when it should be left to right
    Here is an attempt to fix it

    only works on applies (not mutli-applies)
    """
    def visitGrouping(self, elem):
        return Grouping(self(elem.expr))
    def visitApplyM(self, elem):
        return elem
    def visitHead(self, elem):
        return elem
    def visitTail(self, elem):
        return elem
    def visitNil(self, elem):
        return elem
    def visitCons(self, elem):
        return elem
    def visitSub(self, elem):
        return elem
    def visitEqual(self, elem):
        return elem
    def visitMult(self, elem):
        return elem
    def visitAdd(self, elem):
        return elem
    def visitLambdaM(self, elem):
        return elem
    def visitNumber(self, elem):
        return elem
    def visitIf(self, elem):
        return elem
    def visitVariable(self, elem):
        return elem
    def visitBottom(self, elem):
        return elem
    def visitFALSE(self, elem):
        return elem
    def visitTRUE(self, elem):
        return elem
    def visitSeq(self, elem):
        return elem
    def visitLetRec(self, elem):
        return LetRec(elem.var, self(elem.val), self(elem.expr))
    def visitApply(self, elem):
        if isinstance(elem.right, Apply):
            return self(Apply(self(Apply(self(elem.left), self(elem.right.left))), self(elem.right.right)))
        else:
            return Apply(self(elem.left), self(elem.right))
    def visitLambda(self, elem):
        return Lambda(elem.head, self(self.body))
    def visitLet(self, elem):
        return Let(elem.var, self(elem.val), self(elem.expr))


class SingleApply(Visitor):
    """
    Convert all multi-value applies to single value applies
    """

    def visitApplyM(self, elem):
        expr = elem.func
        for val in elem.values:
            expr = Apply(expr, val)
        return self(expr)

    def visitLambdaM(self, elem):
        return LambdaM(elem.args, self(elem.body))

    def visitSeq(self, elem):
        return elem

    def visitLet(self, elem):
        return Let(elem.var, self(elem.val), self(elem.expr))

    def visitLetRec(self, elem):
        return LetRec(elem.var, self(elem.val), self(elem.expr))

    def visitTRUE(self, elem):
        return elem

    def visitFALSE(self, elem):
        return elem

    def visitBottom(self, elem):
        return elem

    def visitVariable(self, elem):
        return elem

    def visitLambda(self, elem):
        return Lambda(elem.head, self(elem.body))

    def visitApply(self, elem):
        return Apply(self(elem.left), self(elem.right))

    def visitIf(self, elem):
        return elem

    def visitNumber(self, elem):
        return elem

    def visitAdd(self, elem):
        return elem

    def visitMult(self, elem):
        return elem

    def visitEqual(self, elem):
        return elem

    def visitSub(self, elem):
        return elem

    def visitCons(self, elem):
        return elem

    def visitNil(self, elem):
        return elem

    def visitTail(self, elem):
        return elem

    def visitHead(self, elem):
        return elem

    def visitGrouping(self, elem):
        return elem


class SingleArgument(Visitor):
    """
    Convert all multi-argument lambdas to single argument lambdas
    """

    def visitLambdaM(self, elem):
        expr = self(elem.body)
        for arg in elem.args[::-1]:
            expr = Lambda(arg, expr)
        return expr

    def visitApplyM(self, elem):
        return ApplyM(self(elem.func), [self(i) for i in elem.values])

    def visitSeq(self, elem):
        return elem

    def visitLet(self, elem):
        return Let(elem.var, self(elem.val), self(elem.expr))

    def visitLetRec(self, elem):
        return LetRec(elem.var, self(elem.val), self(elem.expr))

    def visitTRUE(self, elem):
        return elem

    def visitFALSE(self, elem):
        return elem

    def visitBottom(self, elem):
        return elem

    def visitVariable(self, elem):
        return elem

    def visitLambda(self, elem):
        return Lambda(elem.head, self(elem.body))

    def visitApply(self, elem):
        return Apply(self(elem.left), self(elem.right))

    def visitIf(self, elem):
        return elem

    def visitNumber(self, elem):
        return elem

    def visitAdd(self, elem):
        return elem

    def visitMult(self, elem):
        return elem

    def visitEqual(self, elem):
        return elem

    def visitSub(self, elem):
        return elem

    def visitCons(self, elem):
        return elem

    def visitNil(self, elem):
        return elem

    def visitTail(self, elem):
        return elem

    def visitHead(self, elem):
        return elem

    def visitGrouping(self, elem):
        return elem


class RewriteLet(Visitor):
    """
    Convert let statements to lambda abstractions

    let x = y in z -> (\\x . z) y
    let(x,y,z) = Apply(Lambda(x,z),y)
    """

    def visitMult(self, elem):
        return elem

    def visitAdd(self, elem):
        return elem

    def visitNumber(self, elem):
        return elem

    def visitIf(self, elem):
        return elem

    def visitSeq(self, elem):
        return elem

    def visitApply(self, elem):
        return Apply(self(elem.left), self(elem.right))

    def visitLambda(self, elem):
        return Lambda(elem.head, self(elem.body))

    def visitVariable(self, elem):
        return elem

    def visitBottom(self, elem):
        return elem

    def visitFALSE(self, elem):
        return elem

    def visitTRUE(self, elem):
        return elem

    def visitLet(self, elem):
        return self(Apply(Lambda(elem.var, elem.expr), elem.val))

    def visitLetRec(self, elem):
        return self(Apply(Lambda(elem.var, elem.expr), Apply(Y, Lambda(elem.var, elem.val))))

    def visitLambdaM(self, elem):
        return LambdaM(elem.args, self(elem.body))

    def visitApplyM(self, elem):
        return ApplyM(self(elem.func), [self(i) for i in elem.values])

    def visitEqual(self, elem):
        return elem

    def visitSub(self, elem):
        return elem

    def visitHead(self, elem):
        return elem

    def visitTail(self, elem):
        return elem

    def visitCons(self, elem):
        return elem

    def visitNil(self, elem):
        return elem

    def visitGrouping(self, elem):
        return elem


class ExprToCore(Visitor):
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
        return C.Variable(elem.s)

    def visitLambda(self, elem):
        s = elem.head.s
        return C.Lambda(elem.head, self(elem.body))

    def visitApply(self, elem):
        self_copy = copy.deepcopy(self)
        return C.Apply(self_copy(elem.left), self_copy(elem.right))

    def visitBottom(self, elem):
        return C.Bottom()

    def visitFALSE(self, elem):
        return C.FALSE()

    def visitTRUE(self, elem):
        return C.TRUE()

    def visitIf(self, elem):
        return C.If()

    def visitSeq(self, elem):
        return C.Seq()

    def visitNumber(self, elem):
        return C.Number(elem.n)

    def visitAdd(self, elem):
        return C.Add()

    def visitMult(self, elem):
        return C.Mult()

    def visitLet(self, elem):
        raise RuntimeError("Encountered let in conversion to core.")

    def visitLetRec(self, elem):
        raise RuntimeError("Encountered let-rec in conversion to core.")

    def visitLambdaM(self, elem):
        raise RuntimeError("Encountered multi-argument lambda in conversion to core.")

    def visitApplyM(self, elem):
        raise RuntimeError("Encountered multi-valued apply in conversion to core.")

    def visitEqual(self, elem):
        return C.Equal()

    def visitSub(self, elem):
        return C.Sub()

    def visitNil(self, elem):
        return C.Nil()

    def visitCons(self, elem):
        return C.Cons()

    def visitHead(self, elem):
        return C.Head()

    def visitTail(self, elem):
        return C.Tail()

    def visitGrouping(self, elem):
        return self(elem.expr)


# convenience compoisition
singApp = SingleApply()
singArg = SingleArgument()
let = RewriteLet()
core = ExprToCore()

convert = lambda x: core(let(singArg(singApp(x))))
