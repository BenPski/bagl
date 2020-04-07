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

pub fn eval(expr: Rc<Expr>, env: Rc<Env>, spine: Vec<Rc<Expr>>) -> Rc<Expr> {
    match &*expr {
        Expr::If(cond, b1, b2) => {
            let d = eval(Rc::clone(cond), Rc::clone(&env), spine.clone());
            if let Expr::Data(_, _, s, _) = &*d {
                if s == "True" {
                    eval(Rc::clone(b1), env, spine)
                } else if s == "False" {
                    eval(Rc::clone(b2), env, spine)
                } else {
                    panic!("If expression needs condition to be a boolean.");
                }
            } else {
                panic!("Condition in if needs to be a boolean.");
            }
        }
        Expr::Var(s, depth) => {
            // println!("Variable looking up {:?}", expr);
            if let Some(val) = env.lookup_in(s, *depth.borrow()) {
                eval(Rc::clone(val), env, spine)
            } else {
                panic!("Variable not found, {:?}, in {}", expr, env)
            }
        }
        Expr::App(left, right) => {
            let mut spine = spine;
            spine.push(Rc::clone(right));
            eval(Rc::clone(left), env, spine)
        }
        Expr::Lam(head, body) => {
            // pop from spine and treat like let
            if Rc::deref(head) == &Expr::Var("_".to_string(), RefCell::new(0)) {
                // skip
                let mut spine = spine;
                spine.pop();
                eval(Rc::clone(body), env, spine)
            } else {
                let mut spine = spine;
                if let Some(def) = spine.pop() {
                    let res = eval(def, Rc::clone(&env), spine.clone());
                    eval(
                        Rc::new(Expr::Let(vec![Rc::clone(head)], vec![res], Rc::clone(body))),
                        env,
                        spine,
                    )
                } else {
                    panic!("Not enough arguments supplied")
                }
            }
        }
        Expr::Data(args, t, s, fields) => {
            if fields.len() < *args {
                let mut spine = spine;
                if let Some(def) = spine.pop() {
                    let mut new_fields = fields.to_vec();
                    new_fields.push(eval(def, Rc::clone(&env), spine.clone()));
                    eval(
                        Rc::new(Expr::Data(
                            *args,
                            t.to_string(),
                            s.to_string(),
                            new_fields.to_vec(),
                        )),
                        env,
                        spine,
                    )
                } else {
                    panic!("Not enough arguments to constructor")
                }
            } else {
                expr
            }
        }
        Expr::Builtin(args, s, func, fields, _) => {
            if *args == fields.len() {
                func(fields.to_vec())
            } else if fields.len() < *args {
                let mut spine = spine;
                if let Some(def) = spine.pop() {
                    let mut fields = fields.to_vec();
                    fields.push(eval(def, Rc::clone(&env), spine.clone()));
                    eval(
                        Rc::new(Expr::Builtin(
                            *args,
                            s.to_string(),
                            *func,
                            fields.to_vec(),
                            Vec::new(),
                        )),
                        env,
                        spine,
                    )
                } else {
                    panic!("Not enough arguments supplied to builtin")
                }
            } else {
                expr
            }
        }
        Expr::Let(vars, defs, body) => {
            // define a new layer for env
            let mut new_defs = HashMap::new();
            for i in 0..vars.len() {
                if let Expr::Var(s, _) = &*vars[i] {
                    new_defs.insert(s.to_string(), Rc::clone(&defs[i]));
                } else {
                    panic!("Can only define variables.");
                }
            }
            let new_env = Rc::new(Env::Context(new_defs, Rc::clone(&env)));
            eval(Rc::clone(body), new_env, spine)
        }
        Expr::LetRec(vars, defs, body) => {
            // add a new layer to the environment
            let mut new_defs = HashMap::new();
            for i in 0..vars.len() {
                if let Expr::Var(s, _) = &*vars[i] {
                    new_defs.insert(s.to_string(), Rc::clone(&defs[i]));
                } else {
                    panic!("Can only define variables.");
                }
            }
            let new_env = Rc::new(Env::Context(new_defs, Rc::clone(&env)));
            eval(Rc::clone(body), new_env, spine)
        }
        Expr::Case(expr, pats, branches) => {
            // expr should be a data constructor
            let expr = eval(Rc::clone(expr), Rc::clone(&env), spine.clone());
            // replace branch with let using the assigned variables
            let mut i = 0;
            loop {
                if i >= pats.len() {
                    break;
                } else if pat_match(
                    eval(Rc::clone(&expr), Rc::clone(&env), spine.clone()),
                    &pats[i],
                ) {
                    let (vars, defs) = assign(Rc::clone(&expr), &pats[i]);
                    let expr_new = Rc::new(Expr::Let(vars, defs, Rc::clone(&branches[i])));
                    return eval(expr_new, env, spine);
                } else {
                    i += 1;
                }
            }
            panic!("No pattern matched.");
        }
        Expr::Error(s) => panic!("Error: {}", s),
        Expr::Bottom => panic!("Ran into undefined."),
        _ => expr,
    }
}

fn pat_match(data: Rc<Expr>, pat: &Pattern) -> bool {
    match &*data {
        Expr::Data(_, _, cons, fields) => match pat {
            Pattern::Wildcard => true,
            Pattern::Irrefutable(_) => true,
            Pattern::Construct(pat_cons, vars) => {
                if cons == pat_cons && fields.len() == vars.len() {
                    true
                } else {
                    false
                }
            }
            _ => false,
        },
        Expr::Int(n) => match pat {
            Pattern::Wildcard => true,
            Pattern::Irrefutable(_) => true,
            Pattern::Int(i) => {
                if i == n {
                    true
                } else {
                    false
                }
            }
            _ => false,
        },
        Expr::Float(n) => match pat {
            Pattern::Wildcard => true,
            Pattern::Irrefutable(_) => true,
            Pattern::Float(i) => {
                if i == n {
                    true
                } else {
                    false
                }
            }
            _ => false,
        },
        Expr::Str(n) => match pat {
            Pattern::Wildcard => true,
            Pattern::Irrefutable(_) => true,
            Pattern::Str(i) => {
                if i == n {
                    true
                } else {
                    false
                }
            }
            _ => false,
        },
        _ => panic!(
            "Can only pattern match on constructors and litersl. Received: {}",
            data
        ),
    }
    // if let Expr::Data(_, _, cons, fields) = Rc::deref(&data) {
    //     match pat {
    //         Pattern::Wildcard => true,
    //         Pattern::Irrefutable(_) => true,
    //         Pattern::Construct(pat_cons, vars) => {
    //             if cons == pat_cons && fields.len() == vars.len() {
    //                 true
    //             } else {
    //                 false
    //             }
    //         }
    //         _ => false,
    //     }
    // } else {
    //     panic!(
    //         "Can only pattern match on data constructors. Received: {}",
    //         data
    //     );
    // }
}

fn assign(data: Rc<Expr>, pat: &Pattern) -> (Vec<Rc<Expr>>, Vec<Rc<Expr>>) {
    match &*data {
        Expr::Data(_, _, _, fields) => match pat {
            Pattern::Construct(_, pat_vars) => {
                let mut vars: Vec<Rc<Expr>> = Vec::new();
                let mut defs = Vec::new();
                for i in 0..fields.len() {
                    vars.push(Rc::new(Expr::Var(pat_vars[i].to_string(), RefCell::new(0))));
                    defs.push(Rc::clone(&fields[i]));
                }
                (vars, defs)
            }
            Pattern::Wildcard => (Vec::new(), Vec::new()),
            Pattern::Irrefutable(x) => (
                vec![Rc::new(Expr::Var(x.to_string(), RefCell::new(0)))],
                vec![data],
            ),
            _ => panic!("Matched pattern, but assignment didn't work."),
        },
        Expr::Int(_) => match pat {
            Pattern::Int(_) => (Vec::new(), Vec::new()),
            Pattern::Wildcard => (Vec::new(), Vec::new()),
            Pattern::Irrefutable(x) => (
                vec![Rc::new(Expr::Var(x.to_string(), RefCell::new(0)))],
                vec![data],
            ),
            _ => panic!("Matched pattern, but assignment didn't work."),
        },
        Expr::Float(_) => match pat {
            Pattern::Float(_) => (Vec::new(), Vec::new()),
            Pattern::Wildcard => (Vec::new(), Vec::new()),
            Pattern::Irrefutable(x) => (
                vec![Rc::new(Expr::Var(x.to_string(), RefCell::new(0)))],
                vec![data],
            ),
            _ => panic!("Matched pattern, but assignment didn't work."),
        },
        Expr::Str(_) => match pat {
            Pattern::Str(_) => (Vec::new(), Vec::new()),
            Pattern::Wildcard => (Vec::new(), Vec::new()),
            Pattern::Irrefutable(x) => (
                vec![Rc::new(Expr::Var(x.to_string(), RefCell::new(0)))],
                vec![data],
            ),
            _ => panic!("Matched pattern, but assignment didn't work."),
        },
        _ => panic!("Pattern matched, but assignment didn't work."),
    }
    // if let Expr::Data(_, _, _, fields) = Rc::deref(&data) {
    //     match pat {
    //         Pattern::Wildcard => (Vec::new(), Vec::new()),
    //         Pattern::Irrefutable(x) => (
    //             vec![Rc::new(Expr::Var(x.to_string(), RefCell::new(0)))],
    //             vec![data],
    //         ),
    //         Pattern::Construct(_, pat_vars) => {
    //             let mut vars: Vec<Rc<Expr>> = Vec::new();
    //             let mut defs = Vec::new();
    //             for i in 0..fields.len() {
    //                 vars.push(Rc::new(Expr::Var(pat_vars[i].to_string(), RefCell::new(0))));
    //                 defs.push(Rc::clone(&fields[i]));
    //             }
    //             (vars, defs)
    //         }
    //         _ => (Vec::new(), Vec::new()),
    //     }
    // } else {
    //     panic!("Pattern matched, but assignment didn't work.");
    // }
}
