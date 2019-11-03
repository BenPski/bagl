from Core.Expr import *


def redex(expr, spine=None):
    if spine is None:
        spine = []
    if isinstance(expr, Apply):
        spine.append(expr.right)
        return redex(expr.left, spine)
    else:
        return expr, spine


def substitute(body, index, val):
    if isinstance(body, Apply):
        return Apply(substitute(body.left, index, val), substitute(body.right, index, val))
    elif isinstance(body, Lambda):
        return Lambda(substitute(body.body, Index(index.n + 1), val))
    elif isinstance(body, Index):
        if body.n == index.n:
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
            expr = substitute(expr.body, Index(1), arg)
            return whnf_step(expr, spine)
    if isinstance(expr, If):
        # can either force if to have all 3 arguments present or can have it just test for boolean equality
        # currently just testing for equality
        if len(spine) >= 1:
            cond = spine.pop()
            cond, _ = whnf_step(cond, copy.deepcopy(spine))
            if isinstance(cond, TRUE):
                b1 = Lambda(Lambda(Index(2)))
                return whnf_step(b1, spine)
            elif isinstance(cond, FALSE):
                b2 = Lambda(Lambda(Index(1)))
                return whnf_step(b2, spine)
            else:
                raise RuntimeError("Condition in if statement is not a boolean")
    if isinstance(expr, Builtin):
        if len(spine) >= expr.args:
            args = [spine.pop() for i in range(expr.args)]
            spine_copy = copy.deepcopy(spine)
            args = [whnf_step(arg, spine_copy)[0] for arg in args]
            expr = expr.func(args, spine_copy)
            return whnf_step(expr, spine)
    if isinstance(expr, Apply):  # a little weird to have it here
        expr, spine = redex(expr, spine)
        return whnf_step(expr, spine)

    return expr, spine
