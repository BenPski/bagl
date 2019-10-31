from Expr import *
from Core import CorePrint, whnf

if __name__ == "__main__":
    convert = ExprToCore()
    printer = CorePrint()

    x = Variable("x")
    y = Variable("y")
    z = Variable("z")

    id = Lambda(x, x)
    NOT = Lambda(x, Apply(Apply(Apply(If(), x), FALSE()), TRUE()))

    # expr = convert(Apply(Apply(Apply(If(), TRUE()), Number(1)), Number(2)))
    expr = convert(Apply(Apply(Mult(), Apply(Apply(Add(), Number(2)), Number(2))), Number(11)))

    print(printer(expr))
    print(printer(whnf(expr)))
