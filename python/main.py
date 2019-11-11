from Representation.Expr import *
from Representation.Convert import ExprToCore, RewriteLet, SingleArgument, SingleApply
from Core.Eval import whnf
# from Core.Print import Print

if __name__ == "__main__":
    single = SingleArgument()
    app = SingleApply()
    rewrite = RewriteLet()
    convert = ExprToCore()
    conv = lambda x: convert(rewrite(single(app(x))))

    x = Variable("x")
    y = Variable("y")
    z = Variable("z")
    f = Variable("f")

    id = Lambda(x, x)
    NOT = Lambda(x, Apply(Apply(Apply(If(), x), FALSE()), TRUE()))
    fst = single(LambdaM([x,y], x))
    snd = single(LambdaM([x,y], y))
    Y = Lambda(f, Apply(Lambda(x, Apply(f, Apply(x,x))), Lambda(x, Apply(f,Apply(x,x)))))




    # expr = convert(Apply(Apply(Apply(If(), FALSE()), Bottom()), Number(2)))
    # expr = convert(rewrite(Apply(Apply(Mult(), Apply(Apply(Add(), Number(2)), Number(2))), Number(11))))
    # expr = convert(rewrite(Let(x, Number(2), Apply(Apply(Mult(), x), Number(3)))))
    # expr = convert(rewrite(Let(x, TRUE(), FALSE())))
    # expr = convert(rewrite(Apply(Apply(snd, Bottom()), FALSE())))
    # expr = conv(Let(Variable("fix"), Lambda(f, LetRec(x, Apply(f, x), x)), Apply(Variable("fix"), Lambda(x, Number(2)))))
    # expr = conv(LetRec(f, Lambda(x, Apply(Apply(Apply(If(), Apply(Apply(Equal(), x), Number(1))), Number(1)), Let(y, Apply(f, Apply(Apply(Sub(), x), Number(1))), Apply(Apply(Mult(), x), y)))), Apply(f, Number(10))))
    expr = conv(ApplyM(Cons(), [Number(1), Nil()]))
    # expr = conv(Apply(Head(), Apply(Tail(), Apply(Apply(Cons(), Number(2)), Apply(Apply(Cons(), Number(1)), Nil())))))


    # list = ApplyM(Cons(), [Number(1), ApplyM(Cons(), [Number(2), Nil()])])
    # expr = conv(Apply(Tail(), inf_list))
    inf_list = LetRec(f, Lambda(x, ApplyM(Cons(), [x, Apply(f, ApplyM(Add(), [x, Number(1)]))])), Apply(f, Number(1)))
    expr = conv(Apply(Head(), Apply(Tail(), Apply(Tail(), inf_list))))
    # OR = LambdaM([x,y], ApplyM(If(), [x, x, y]))
    # expr = conv(LetRec(f, Lambda(x, ApplyM(If(), [ApplyM(OR, [ApplyM(Equal(), [x, Number(0)]), ApplyM(Equal(), [x, Number(1)])]), Number(1), ApplyM(Add(), [Apply(f, ApplyM(Sub(), [x,Number(1)])), Apply(f, ApplyM(Sub(), [x, Number(2)]))])])), Apply(f, Number(10))))


    print(expr)
    print(whnf(expr))
