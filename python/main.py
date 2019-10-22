from Expr import *
from ExprDB import *

if __name__ == "__main__":
    # # variables for defintions
    # x = [Variable("x" + str(i)) for i in range(10)]
    # # x = Variable("x")
    # y = Variable("y")
    # z = Variable("z")
    # a = Variable("a")
    # b = Variable("b")
    # c = Variable("c")
    # t = Lambda(x[0], Lambda(x[1], x[0]))
    # f = Lambda(x[2], Lambda(x[3], x[3]))
    # NOT = Lambda(x[4], Apply(Apply(x[4], f), t))
    # AND = Lambda(x[5], Lambda(x[6], Apply(Apply(x[5], x[6]), x[5])))
    # OR = Lambda(x[5], Lambda(x[6], Apply(Apply(x[5], x[5]), x[6])))
    # # AND = Lambda(x,Lambda(y,Lambda(z,)))
    # # id = Lambda(x, x)
    # # expr = Apply(id,Variable("y"))
    # expr = Apply(Apply(OR, f), Apply(NOT, f))
    # printer = ExprPrint()
    # interp = ExprWHNF()
    # print(expr.accept(interp).accept(interp).accept(printer))

    # x = VariableDB("x")
    # id = LambdaDB(IndexDB(1))
    # expr = ApplyDB(id, x)
    # interp = ExprDBWHNF()
    # printer = ExprDBPrint()
    # print(expr.accept(interp).accept(printer))

    convert = ExprToExprDB()
    printer = ExprDBPrint()
    interp = ExprDBWHNF()

    x = Variable("x")
    y = Variable("y")
    z = Variable("z")

    t = Lambda(x,Lambda(y,x))
    f = Lambda(x,Lambda(y,y))
    id = Lambda(x,x)
    NOT = Lambda(x, Apply(Apply(x, f), t))
    AND = Lambda(x, Lambda(y, Apply(Apply(x, y), x)))
    OR = Lambda(x, Lambda(y, Apply(Apply(x, x), y)))

    expr1 = convert(Apply(Apply(AND, f), Apply(NOT, t)))
    expr2 = convert(Apply(Apply(OR, t), Apply(NOT, t)))

    print(printer(interp(expr1)))
    print(printer(interp(expr2)))
