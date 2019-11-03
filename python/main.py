from Representation.Expr import *
from Representation.Convert import ExprToCore
from Core.Eval import whnf
from Core.Print import Print

if __name__ == "__main__":
    convert = ExprToCore()
    printer = Print()

    x = Variable("x")
    y = Variable("y")
    z = Variable("z")

    id = Lambda(x, x)
    NOT = Lambda(x, Apply(Apply(Apply(If(), x), FALSE()), TRUE()))

    # expr = convert(Apply(Apply(Apply(If(), TRUE()), Number(1)), Number(2)))
    expr = convert(Apply(Apply(Mult(), Apply(Apply(Add(), Number(2)), Number(2))), Number(11)))

    print(printer(expr))
    print(printer(whnf(expr)))
