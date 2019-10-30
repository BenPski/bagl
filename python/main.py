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

    expr = convert(Apply(Lambda(x, Apply(If(), Apply(NOT, x))), TRUE()))
    # expr = convert(Apply(x,z))

    print(printer(expr))
    print(printer(whnf(expr)))
