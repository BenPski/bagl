from Core.Expr import *


def redex(expr, spine=None):
    if spine is None:
        spine = []
    if isinstance(expr, Apply):
        spine.append(expr.right)
        return redex(expr.left, spine)
    else:
        return expr, spine


def substitute(body, var, val):
    if isinstance(body, Apply):
        return Apply(substitute(body.left, var, val), substitute(body.right, var, val))
    elif isinstance(body, Lambda):
        if body.head.s != var.s:
            return Lambda(body.head, substitute(body.body, var, val))
        else:
            return body
    elif isinstance(body, Variable):
        if body.s == var.s:
            return val
        else:
            return body
    return body


def whnf(expr):
    expr_prev = expr
    top, spine = redex(expr_prev)
    expr, spine = whnf_step(top, spine)
    while expr != expr_prev:
        expr_prev = expr
        expr, spine = whnf_step(expr, spine)

    while len(spine) > 0:
        arg = spine.pop()
        expr = Apply(expr, arg)
    return expr


def whnf_step(expr, spine):
    if isinstance(expr, Bottom):
        raise RuntimeError("Evaluated bottom.")
    if isinstance(expr, Lambda):
        if len(spine) > 0:
            arg = spine.pop()
            expr = substitute(expr.body, expr.head, arg)
            return whnf_step(expr, spine)
    if isinstance(expr, If):
        # can either force if to have all 3 arguments present or can have it just test for boolean equality
        # currently just testing for equality
        if len(spine) >= 1:
            cond = spine.pop()
            cond, _ = whnf_step(cond, copy.deepcopy(spine))
            if isinstance(cond, TRUE):
                b1 = Lambda(Variable("x"), Lambda(Variable("y"), Variable("x")))
                return whnf_step(b1, spine)
            elif isinstance(cond, FALSE):
                b2 = Lambda(Variable("x"), Lambda(Variable("y"), Variable("y")))
                return whnf_step(b2, spine)
            else:
                raise RuntimeError("Condition in if statement is not a boolean")
    if isinstance(expr, Seq):
        if len(spine) >= 1:
            a = spine.pop()
            a, _ = whnf_step(a, copy.deepcopy(spine))
            return whnf_step(Lambda(Variable("x"), Variable("x")), spine)
    if isinstance(expr, Builtin):
        if len(spine) >= expr.args:
            args = [spine.pop() for i in range(expr.args)]
            spine_copy = copy.deepcopy(spine)
            args_eval = []
            args = [whnf_step(arg, copy.deepcopy(spine_copy))[0] for arg in args]
            expr = expr.func(args, spine_copy)
            return whnf_step(expr, spine)

    if isinstance(expr, Data):
        if not expr.initialized:

            if len(spine) >= expr.fields:
                args = [spine.pop() for i in range(expr.fields)]
                expr.construct(copy.deepcopy(args), copy.deepcopy(spine))
                return whnf_step(expr, spine)
        else:
            return expr, spine

    if isinstance(expr, Apply):  # a little weird to have it here
        expr, spine = redex(expr, spine)
        return whnf_step(expr, spine)

    return expr, spine
