"""
Want to add builtin let and letrec rather than relying on conversion to lambdas and the y combinator

for letrec it needs to store a list of defintions 
for let it can only be one defintion since multiple defintions is equaivalent to 2 composed with each other

for letrec the names need to be available before the lookup happens

Now need to introduce an environment into the evaluation rather than just the spine

Substitution still occurs as before, but now when evaluating a variable to whnf look it up in the environment and then do substitution for the variable

"""


from Core.Expr import *
import copy


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
    elif isinstance(body, Letrec):
        used = False
        for v in body.vars:
            if v.s == var.s:
                used = True
        if not used:
            expr_new = substitute(body.expr, var, val)
            vals_new = []
            for v in body.vals:
                vals_new.append(substitute(v, var, val))
            return Letrec(body.vars, vals_new, expr_new)
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
    if isinstance(expr, Letrec):
        env = {}
        for i in range(len(expr.vars)):
            env[expr.vars[i].s] = expr.vals[i]
        push_scope(expr, env)
        return whnf_step(expr.expr, spine)
    if isinstance(expr, Variable):
        val = expr.env.lookup(expr.s)
        if val is None:
            raise RuntimeError("Variable not defined.")
        else:
            return whnf_step(val, spine)
        # if expr.s in env:
        #     val = env[expr.s]
        #     return whnf_step(val, spine)
        # else:
        #     raise RuntimeError("Variable not defined.")
    if isinstance(expr, Lambda):
        if len(spine) > 0:
            arg = spine.pop()
            expr_new = substitute(expr.body, expr.head, arg)
            # env_new = copy.deepcopy(env)
            # for v in env_new:
            #     env_new[v] = substitute(env[v], expr.head, arg)
            return whnf_step(expr_new, spine)
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




def push_scope(expr, scope):
    if isinstance(expr, Lambda):
        # expr.env.add(scope)
        push_scope(expr.body, scope)
    elif isinstance(expr, Apply):
        # expr.env.add(scope)
        push_scope(expr.left, scope)
        push_scope(expr.right, scope)
    elif isinstance(expr, Letrec):
        for val in expr.vals:
            push_scope(val, scope)
        push_scope(expr.expr, scope)
    elif isinstance(expr, Variable): # only place environments really matter
        expr.env.add(scope)
    else:
        pass