"""
Convert from the toaster expression to the core expression

mostly just recognizing keywords
"""

from Toaster.Expr import Visitor
import Toaster.Expr as T
import Core.Expr as C


# Y combinator
# f = T.Variable("f")
# x = T.Variable("x")
# Y = T.Lambda(f, T.Apply(T.Lambda(x, T.Apply(x, x)), T.Lambda(x, T.Apply(f, T.Apply(x, x)))))

class Convert(Visitor):
    def visitLet(self, elem):
        # return self(T.Apply(T.Lambda(elem.var, elem.expr), elem.val))
        # return self(T.Apply(T.Lambda(elem.var, elem.expr), T.Apply(Y, T.Lambda(elem.var, elem.val))))
        return C.Letrec([self(x) for x in elem.vars], [self(x) for x in elem.vals], self(elem.expr))

    def visitInteger(self, elem):
        return C.Number(elem.n)

    def visitFloat(self, elem):
        return C.Number(elem.n)

    def visitString(self, elem):
        return C.String(elem.s)

    def visitGroup(self, elem):
        return self(elem.expr)

    def visitApply(self, elem):
        return C.Apply(self(elem.left), self(elem.right))

    def visitLambda(self, elem):
        return C.Lambda(self(elem.head), self(elem.body))

    def visitVariable(self, elem):
        if elem.s == "_|_":
            return C.Bottom()
        elif elem.s == "*":
            return C.Mult()
        elif elem.s == '+':
            return C.Add()
        elif elem.s == "-":
            return C.Sub()
        elif elem.s == "if":
            return C.If()
        elif elem.s == "==":
            return C.Equal()
        elif elem.s == "!=":
            return C.NEqual()
        elif elem.s == "head":
            return C.Head()
        elif elem.s == "tail":
            return C.Tail()
        elif elem.s == "seq":
            return C.Seq()
        elif elem.s ==  "Cons":
            return C.Cons()
        elif elem.s == "Nil":
            return C.Nil()
        elif elem.s == "null":
            return C.Null()
        elif elem.s == "True":
            return C.TRUE()
        elif elem.s == "False":
            return C.FALSE()
        elif elem.s == "concat":
            return C.Concat()
        return C.Variable(elem.s)

conv = Convert()
convert = lambda x: conv(x)