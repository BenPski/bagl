/*
Going to do strict evaluation of the language
since everything should be one function arguments short cicuiting should still behave properly

actually short circuiting won't work because the argument is evaluated before being passed into the function
meaning if we were to do (and x y) even if x is false we wil still evaluate y before being able to return

could deal with this by bolting on laziness via thunks or could make a special function that does not evaluate its argument, but still consumes it

dealing with all lets as letrecs
*/

use crate::ast::Expr;
use crate::ast::Pattern;
use crate::env::Env;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

pub fn eval(expr: Rc<Expr>, env: Rc<Env>) -> Rc<Expr> {
    // lookup variables in the environment
    match &*expr {
        Expr::Lazy(_) => expr, //do nothing
        Expr::Force(val) => {
            if let Expr::Lazy(x) = Rc::deref(val) {
                eval(Rc::clone(x), env)
            } else {
                // forcing a non lazy value does nothing
                Rc::clone(val)
            }
        }
        Expr::Builtin(args, _, func, fields) => {
            if *args == fields.len() {
                func(fields.to_vec())
            } else {
                expr
            }
        }
        Expr::If(cond, b1, b2) => {
            if let Expr::Data(_, _, s, _) = Rc::deref(&eval(Rc::clone(cond), Rc::clone(&env))) {
                if s == "True" {
                    eval(Rc::clone(b1), env)
                } else if s == "False" {
                    eval(Rc::clone(b2), env)
                } else {
                    panic!("If expression needs condition to be a boolean.");
                }
            } else {
                panic!("Condition in if needs to be a boolean.");
            }
        }
        Expr::Var(s, env_index) => {
            if let Some(val) = env.lookup_at(s, *env_index.borrow()) {
                eval(Rc::clone(val), env)
            } else {
                panic!("Variable not found, {:?}, in {}", expr, env)
            }
        }
        Expr::App(left, right) => eval_app(
            eval(Rc::clone(left), Rc::clone(&env)),
            eval(Rc::clone(&right), Rc::clone(&env)),
            // Rc::clone(left),
            // Rc::clone(right),
            env,
        ),
        Expr::Let(vars, defs, body) => {
            let mut new_defs = HashMap::new();
            let curr_depth = env.depth();
            for i in 0..vars.len() {
                if let Expr::Var(s, _) = &*vars[i] {
                    new_defs.insert(s.to_string(), Rc::clone(&defs[i]));
                    set_depth(Rc::clone(&vars[i]), curr_depth + 1);
                } else {
                    panic!("Can only define variables.");
                }
            }
            let new_env = Rc::new(Env::Context(new_defs, Rc::clone(&env)));
            push_depth(Rc::clone(body), curr_depth + 1);
            eval(Rc::clone(body), new_env)
        }
        Expr::Case(expr, pats, branches) => {
            // expr should be a data constructor
            let mut i = 0;
            loop {
                if i >= pats.len() {
                    break;
                } else if pat_match(Rc::clone(expr), &pats[i]) {
                    let (vars, defs) = assign(Rc::clone(expr), &pats[i]);
                    let mut expr_update = Rc::clone(&branches[i]);
                    for j in 0..vars.len() {
                        expr_update = substitute(
                            Rc::clone(&defs[j]),
                            Rc::clone(&vars[j]),
                            Rc::clone(&expr_update),
                        );
                    }
                    // println!("{:?}, {:?}", vars, defs);
                    return eval(Rc::clone(&expr_update), env);
                // return eval(Rc::new(Expr::Let(vars, defs, Rc::clone(&branches[i]))), env);
                } else {
                    i += 1;
                }
            }
            panic!("No pattern matched.");
        }
        Expr::Bottom => panic!("Tried to evaluate bottom."),
        _ => expr,
    }
}

fn eval_app(left: Rc<Expr>, right: Rc<Expr>, env: Rc<Env>) -> Rc<Expr> {
    match &*left {
        Expr::Lam(head, body) => eval(
            substitute(
                eval(right, Rc::clone(&env)),
                Rc::clone(head),
                Rc::clone(body),
            ),
            env,
        ),
        Expr::App(left2, right2) => eval_app(
            eval_app(Rc::clone(left2), Rc::clone(right2), Rc::clone(&env)),
            Rc::clone(&right),
            env,
        ),
        Expr::Data(args, t, s, fields) => {
            if fields.len() < *args {
                let mut fields = fields.to_vec();
                fields.push(right);
                Rc::new(Expr::Data(
                    *args,
                    t.to_string(),
                    s.to_string(),
                    fields.to_vec(),
                ))
            } else {
                panic!(
                    "Too many arguments supplied to constructor. {}, {}",
                    left, right
                );
            }
        }
        Expr::Builtin(args, s, func, fields) => {
            if fields.len() < *args {
                let mut fields = fields.to_vec();
                fields.push(right);
                eval(
                    Rc::new(Expr::Builtin(*args, s.to_string(), *func, fields.to_vec())),
                    env,
                )
            } else {
                panic!(
                    "Too many arguments aupplied to function. {}, {}",
                    left, right
                );
            }
        }
        _ => eval(left, env),
    }
}

// pub fn strict(expr: Rc<Expr>) -> Rc<Expr> {
//     // want to evaluate strictly on the first argument to a lambda
//     match &*expr {
//         Expr::App(left, right) => strict(strict_app(Rc::clone(left), Rc::clone(right))),
//         Expr::Bottom => panic!("Tried to evaluate bottom."),
//         _ => expr,
//     }
// }

// fn strict_app(left: Rc<Expr>, right: Rc<Expr>) -> Rc<Expr> {
//     match &*left {
//         Expr::Lam(head, body) => substitute(right, Rc::clone(head), Rc::clone(body)),
//         Expr::App(left2, right2) => {
//             strict_app(strict_app(Rc::clone(left2), Rc::clone(right2)), right)
//         }
//         _ => strict(left),
//     }
// }

fn substitute(val: Rc<Expr>, var: Rc<Expr>, expr: Rc<Expr>) -> Rc<Expr> {
    match &*expr {
        Expr::Var(_, _) => {
            if expr == var {
                val
            } else {
                expr
            }
        }
        Expr::Lam(head, body) => {
            if *head == var {
                expr
            } else {
                Rc::new(Expr::Lam(
                    Rc::clone(head),
                    substitute(val, var, Rc::clone(body)),
                ))
            }
        }
        Expr::If(cond, b1, b2) => Rc::new(Expr::If(
            substitute(Rc::clone(&val), Rc::clone(&var), Rc::clone(cond)),
            substitute(Rc::clone(&val), Rc::clone(&var), Rc::clone(b1)),
            substitute(val, var, Rc::clone(b2)),
        )),
        Expr::Lazy(expr) => Rc::new(Expr::Lazy(substitute(val, var, Rc::clone(expr)))),
        Expr::Force(expr) => Rc::new(Expr::Force(substitute(val, var, Rc::clone(expr)))),
        Expr::App(left, right) => Rc::new(Expr::App(
            substitute(Rc::clone(&val), Rc::clone(&var), Rc::clone(left)),
            substitute(val, var, Rc::clone(right)),
        )),
        // not sure if necessary for builtins and data
        Expr::Builtin(args, s, func, fields) => {
            let mut new_fields = fields.to_vec();
            for field in fields {
                new_fields.push(substitute(
                    Rc::clone(&val),
                    Rc::clone(&var),
                    Rc::clone(field),
                ));
            }
            Rc::new(Expr::Builtin(*args, s.to_string(), *func, new_fields))
        }
        Expr::Data(args, t, s, fields) => {
            let mut new_fields = fields.to_vec();
            for field in fields {
                new_fields.push(substitute(
                    Rc::clone(&val),
                    Rc::clone(&var),
                    Rc::clone(field),
                ));
            }
            Rc::new(Expr::Data(*args, t.to_string(), s.to_string(), new_fields))
        }
        Expr::Let(vars, defs, body) => {
            let mut new_defs = Vec::new();
            for def in defs {
                new_defs.push(substitute(Rc::clone(&val), Rc::clone(&var), Rc::clone(def)));
            }
            Rc::new(Expr::Let(
                vars.to_vec(),
                new_defs,
                substitute(val, var, Rc::clone(body)),
            ))
        }
        Expr::Case(expr, pats, branches) => {
            let mut branches_new = Vec::new();
            for branch in branches {
                branches_new.push(substitute(
                    Rc::clone(&val),
                    Rc::clone(&var),
                    Rc::clone(branch),
                ));
            }
            Rc::new(Expr::Case(
                substitute(val, var, Rc::clone(expr)),
                pats.to_vec(),
                branches_new,
            ))
        }
        _ => expr,
    }
}

fn set_depth(var: Rc<Expr>, n: usize) {
    // update a variable to have the proper associated depth
    if let Expr::Var(_, depth) = Rc::deref(&var) {
        depth.replace(n);
    } else {
        panic!("Only variables have a depth. Given {}.", var);
    }
}

fn push_depth(expr: Rc<Expr>, n: usize) {
    // push a depth through an expression
    match &*expr {
        Expr::Var(_, _) => set_depth(expr, n),
        Expr::Lam(_, body) => push_depth(Rc::clone(body), n),
        Expr::App(left, right) => {
            push_depth(Rc::clone(left), n);
            push_depth(Rc::clone(right), n);
        }
        _ => {}
    }
}

fn pat_match(data: Rc<Expr>, pat: &Pattern) -> bool {
    if let Expr::Data(_, _, cons, fields) = Rc::deref(&data) {
        match pat {
            Pattern::Irrefutable(_) => true,
            Pattern::Construct(pat_cons, vars) => {
                if cons == pat_cons && fields.len() == vars.len() {
                    true
                } else {
                    false
                }
            }
        }
    } else {
        panic!(
            "Can only pattern match on data constructors. Received: {}",
            data
        );
    }
}

fn assign(data: Rc<Expr>, pat: &Pattern) -> (Vec<Rc<Expr>>, Vec<Rc<Expr>>) {
    if let Expr::Data(_, _, _, fields) = Rc::deref(&data) {
        match pat {
            Pattern::Irrefutable(x) => (
                vec![Rc::new(Expr::Var(x.to_string(), RefCell::new(0)))],
                vec![data],
            ),
            Pattern::Construct(_, pat_vars) => {
                let mut vars: Vec<Rc<Expr>> = Vec::new();
                let mut defs = Vec::new();
                for i in 0..fields.len() {
                    vars.push(Rc::new(Expr::Var(pat_vars[i].to_string(), RefCell::new(0))));
                    defs.push(Rc::clone(&fields[i]));
                }
                (vars, defs)
            }
        }
    } else {
        panic!("Pattern matched, but assignment didn't work.");
    }
}
