from Expr import *
from ExprDB import *

if __name__ == "__main__":
    convert = ExprToExprDB()
    printer = ExprDBPrint()
    interp = Eval()

    x = Variable("x")
    y = Variable("y")
    z = Variable("z")

    t = Lambda(x, Lambda(y, x))
    f = Lambda(x, Lambda(y, y))
    id = Lambda(x, x)
    NOT = Lambda(x, Apply(Apply(x, f), t))
    AND = Lambda(x, Lambda(y, Apply(Apply(x,y), x)))
    OR = Lambda(x, Lambda(y, Apply(Apply(x, x), y)))

    expr1 = convert(Apply(Apply(AND, Apply(NOT, f)), Apply(NOT,t)))
    expr2 = convert(Apply(Apply(OR, t), t))

    # expr1 = convert(Apply(x,z))

    print(printer(expr1))
    print(printer(interp(expr1)))
