use std::rc::Rc;
use std::ops::Deref;
use std::collections::HashMap;
use std::cell::RefCell;

use crate::environment::*;
use crate::environment::Env::*;
use crate::expr::Expr;
use crate::expr::Expr::*;

fn redex(expr: Rc<Expr>, spine: &mut Vec<Rc<Expr>>) -> Rc<Expr> {
    match & *expr {
        Apply(left, right) => {
            spine.push(Rc::clone(right));
            redex(Rc::clone(left), spine)
        }
        _ => expr
    }
}

fn substitute(body: Rc<Expr>, var: Rc<Expr>, val: Rc<Expr>) -> Rc<Expr> {
    match &*body {
        Apply(left, right) => Rc::new(Apply(substitute(Rc::clone(left), Rc::clone(&var), Rc::clone(&val)), substitute(Rc::clone(right), Rc::clone(&var), Rc::clone(&val)))),
        Lambda(head, lam_body) => if head == &var { body } else { Rc::new(Lambda(Rc::clone(head), substitute(Rc::clone(lam_body), var, val))) },
        LetRec(vars, vals, expr, env) => {
            for i in 0..vars.len() {
                if vars[i] == var {
                    return body
                }
            }
            let mut vars_new = Vec::clone(vars);
            let mut vals_new = Vec::clone(vals);
            vars_new.push(var);
            vals_new.push(val);
            Rc::new(LetRec(vars_new, vals_new, Rc::clone(expr), env.clone()))
        },
        Variable(_, _) => if body == var {val} else {body},
        Case(expr, pats, branches, env) => {
            let mut branches_new = Vec::new();
            for branch in branches {
                branches_new.push(substitute(Rc::clone(branch), Rc::clone(&var), Rc::clone(&val)));
            }
            Rc::new(Case(substitute(Rc::clone(&expr), var, val), Vec::clone(pats), branches_new, env.clone()))
        },
        _ => body
    }
}


pub fn whnf(expr: Rc<Expr>) -> Rc<Expr> {
    let spine = &mut Vec::new();
    let top = redex(expr, spine);
    whnf_rep(top, spine)
}

fn whnf_rep(expr: Rc<Expr>, spine: &mut Vec<Rc<Expr>>) -> Rc<Expr> {
    let mut modified = false;
    let expr_next = whnf_step(Rc::clone(&expr), spine, &mut modified);

    if !modified {
        if spine.len() == 0 {
            match &*expr_next {
                Apply(_, _) => whnf(expr_next),
                _ => expr_next
            }
        } else {
            whnf_rep(expr_next, spine)
        }
    } else {
        whnf_rep(expr_next, spine)
    }
}
// modified marks whether or not any modification actually happened, if it didn't we are done evaluating (maybe)
fn whnf_step(expr: Rc<Expr>, spine: &mut Vec<Rc<Expr>>, modified: &mut bool) -> Rc<Expr> {
    match &*expr {
        Apply(_, _) => {
            *modified = true;
            redex(expr, spine)
        },
        Lambda(head, body) => {
            let arg = spine.pop();
            match arg {
                Some(x) => {
                    let expr_new = substitute(Rc::clone(body), Rc::clone(head), x);
                    *modified = true;
                    expr_new
                }
                _ => expr
            }
        },
        LetRec(vars, vals, let_expr, env) => {
            let mut definitions = HashMap::new();
            for i in 0..vars.len() {
                if let Variable(s, _) = Rc::deref(&vars[i]) {
                    definitions.insert(s.to_string(), Rc::clone(&vals[i]));
                } else {
                    panic!("Can only define variables.")
                }
            }
            let new_env = Rc::new(Context(definitions, Rc::clone(&env.borrow())));
            push_env(&expr, &new_env);
            *modified = true;
            Rc::clone(&let_expr)
        },
        Variable(s, env) => {
            if let Some(val) = env.borrow().lookup(s) {
//                println!("Looking up {} with {}", s, env.borrow());
                *modified = true;
                Rc::clone(val)
            } else {
                println!("expr: {}", expr);
                println!("env: {}", env.borrow());
                panic!("Variable not defined.")
            }
        }
        Builtin(args, _, func) => {
            if (*args as usize) <= spine.len() {
                let mut inputs = Vec::new();
                for _ in 0..*args {
                    if let Some(a) = spine.pop() {
                        inputs.push(a);
                    }
                }
                // assume all builtins are strict in all arguments
                for i in 0..inputs.len() {
                    let update = whnf(Rc::clone(&inputs[i]));
                    inputs[i] = update;
                }
                let res:Rc<Expr> = func(inputs);
                *modified = true;
                res
            } else {
                expr
            }
        },
        Data(args, show, typ, initialized, _) => {
//            println!("Data step");
            if !initialized {
                if (*args as usize) <= spine.len() {
                    let mut inputs = Vec::new();
                    for _ in 0..*args {
                        if let Some(a) = spine.pop() {
                            inputs.push(a);
                        }
                    }
                    let new = Rc::new(Data(*args, show.to_string(), typ.to_string(), true, inputs));
                    *modified = true;
                    whnf_step(new, spine, modified)
                } else {
                    expr
                }
            } else {
                expr
            }
        },
        Case(case_expr, pats, branches, env) => {
            // the case_expr needs to be in whnf to be pattern matched on
            // when matching a pattern want to push the variables into the environment
            match Rc::deref(case_expr) {
                Data(_, _, show, _, fields) => {
                    let mut branch = None;
                    for i in 0..pats.len() {
                        match Rc::deref(&pats[i]) {
                            Data(_, _, s, _, vars) => {
                                if s.eq(show) {
                                    let mut definitions = HashMap::new();
                                    for i in 0..vars.len() {
                                        if let Variable(name, _) = Rc::deref(&vars[i]) {
                                            definitions.insert(name.to_string(), Rc::clone(&fields[i]));
                                        }
                                    }
                                    let new_env = Rc::new(Context(definitions, Rc::clone(&env.borrow())));
                                    let new_branch = Rc::clone(&branches[i]);
                                    push_env(&new_branch, &new_env);
                                    branch = Some(new_branch);
                                }
                            }
                            _ => panic!("A pattern has to be a data constructor.")
                        }
                    }
                    match branch {
                        Some(v) => {
                            *modified = true;
                            v
                        },
                        _ => panic!("Could not find a matching pattern, maybe a type error or non-exhaustive.")
                    }
                },
                _ => {
                    let case_update = whnf(Rc::clone(case_expr));
                    *modified = true;
                    Rc::new(Case(case_update, Vec::clone(pats), Vec::clone(branches), env.clone()))
                },
            }
        },
        Bottom => panic!("Evaluated bottom."),
        _ => expr
    }
}


fn push_env(expr: &Rc<Expr>, env: &Rc<Env>) {
    match Rc::deref(expr) {
        Lambda(_, body) => {
            push_env(body, env);
        },
        Apply(left, right) => {
            push_env(left, env);
            push_env(right, env);
        }
        LetRec(_, vals, let_expr, old_env) => {
            for i in 0..vals.len() {
                push_env(&vals[i], env);
            }
            push_env(let_expr, env);
            *old_env.borrow_mut() = Rc::clone(env);
        }
        Case(case_expr, _, branches, old_env) => {
            for branch in branches {
                push_env(branch, env);
            }
            push_env(case_expr, env);
            *old_env.borrow_mut() = Rc::clone(env);
        }
        Variable(_, old_env) => {
            *old_env.borrow_mut() = Rc::clone(env);
        }
        _ => {},
    }
}


//fn make_var(sym: &str) -> Rc<Expr> {
//    Rc::new(Variable(sym.to_string(), RefCell::new(Rc::new(Env::new()))))
//}
//
//fn make_lam(head: Rc<Expr>, body: Rc<Expr>) -> Rc<Expr> {
//    Rc::new(Lambda(head, body))
//}
//
//fn make_app(left: Rc<Expr>, right: Rc<Expr>) -> Rc<Expr> {
//    Rc::new(Apply(left, right))
//}
