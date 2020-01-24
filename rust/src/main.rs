/*

need to deal with cyclic references in letrec
    use weak pointers in the environment definition?


So, I know that the push_scope idea works because of the python version being fine; however, due to the cyclic references the memory management is an issue
    also there is probably a lot of duplication which would be nice to eliminate

Environment:
    provides a mapping from a variable to a definition
    needs to allow recursion
        the value associated with a variable in the environment can point back to the containing environment

    can do parent <-> child cycles by having the pointers from parent to child be strong and child to parent be weak
        child -> parent exist as long as parent exists, soon as parent stops existing so does child and therefore the pointers as well

    also the other tricky bit is the mixing of the environment and the expressions
        the expression can exist without the environment, but as soon as evaluation starts to happen an environment is necessary
    with strict evaluation this isn't too much of an issue as going into and out of scope is not too bad, but with lazy evaluation nested environments seems to cause an issue
        not actually sure about strict being easier in the nested environment case

    process:
        initial expression and environment
            (Let[vars, vals, expr], Empty)
        encounter let so update environment
            (expr, env = {vars:vals})
                anytime a variable needs to be looked up refer to the environment

    I believe with only one level of lets this works just fine, but what to do with multiple levels?
    let
        x = let a = b in a
    in x
    ast:
        (Let[Var(x),Let[Var(a), Var(b), Var(a)],Var(x)], Empty)
    let/env step:
        (Var(x), {Var(x): Let[Var(a), Var(b), Var(a)]})
    lookup and substitute var:
        (Let[Var(a), Var(b), Var(a)], {Var(x): Let[Var(a), Var(b), Var(a)]})
    let/env step: (now have multiple envs
        (Var(a), {Var(a): Var(b)}, {Var(x): Let[Var(a), Var(b), Var(a)]})
    lookup and substitute var: (now we lose the extra environment
        (Var(b), {Var(x): Let[Var(a), Var(b), Var(a)]})
    whnf

    So, the issue is adding and removing the environment properly
        easy to add, but need some extra mechanism to make sure removing happens at the right time
            could continuously check for environment being necessary, but that is slow
            could tag expressions with let depth to know how long it should be

    ast:
        (0, Let[x, Let[a, b, a], x], Empty)
    let/env:
        (1, x, [{x: Let[a,b,a]}, Empty])
    lookup/subs
        (1, Let[a,b,a], [{x: Let[a,b,a]}, Empty])
    let/env
        (2, a, [{a: b}, {x: Let[a,b,a]}, Empty])
    lookup/subs
        (2, b, [{a:b}, {x: Let[a,b,a]}, Empty])
    i dont know how to drop down


    Could possibly sidestep the problem by building up the all the environments at the beginning and then having a pattern for looking them up

    let
        x = let a = b in a
    in x

    env:
        (0) => {x: a[(0,0)]}
        (0, 0) => {a: b[(0,0)]}
    expr:
        x[(0)]

    so looking up starts at given position and then strips off any numbers from the right

    let
        t = \x.\y.x
        f = \x.\y.y
    in t

    env:
        (0) => {t: \x.\y.x, f: \x.\y.y}
    expr: t[(0)]

    let
        f = \x.x
        g = \x.(f x)
    in (g f)

    env:
        (0) => {f: \x.x,
                g: \x.(f[(0)] x)}
    expr:
        (g[(0)] f[(0)])


    let
        f = \x.(let a = x in a)
        g = \x.(f x)
    in (g f)

    env:
        (0) => {f: \x{(0,1)}.a[(0,0)],
                g: \x.(f[(0)] x)}
        (0, 0) => {a: x[(0,1)]}
        (0, 1) => {x: _}
    a little messy when considering lambdas introducing values into environments as it means nodes will be expected to be added at some point and will need to be modified during the runtime

    Could instead have strong pointers between environments and weak pointers to the relevant environent in variables?
        have to have pointers to pointers for the environment




    Env {
        Empty,
        Context(Map, Rc<Env>)
    }

    Variable(s, Refcell<Rc<Env>>)


Env {
    definitions: Map<String, Expr>
}


First for the sake of making this more organized I'd like to split this into some modules
    parsing
    evaluation
    language definition


actually I think the approach way I was doing it was right, but the pointers weren't mutable and that was the issue


Can deal with the environment using a combination of Rc, Weak, and RefCell
    Rc: with (a -> b) if a goes out of scope so does b
    Weak: with (a -> b) a goes out of scope does not mean b goes out of scope
    RefCell: allows mutation of pointers

    the refcell allows for an environment to be constructed and then gone back over to change the pointers to itself

    since variables are what hold the references to the environment they should hold the refcells to environments


*/
use std::rc::Rc;
use std::ops::Deref;
use std::collections::HashMap;

pub mod environment;
use environment::*;

pub mod expr;
use expr::Expr;
use expr::Expr::*;

pub mod either;
use either::Either;

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
        }
//        LetRec(vars, vals, expr, env) => {
//            // the lazy way is just adding var = val to the let, have to check if the value is already there though
//            for i in 0..vars.len() {
//                if vars[i] == var {
//                    return body
//                }
//            }
//            let mut vars_new = Vec::clone(vars);
//            let mut vals_new = Vec::clone(vals);
//            vars_new.push(var);
//            vals_new.push(val);
//            Rc::new(LetRec(vars_new, vals_new, Rc::clone(expr)))
//        },
        Variable(_, _) => if body == var {val} else {body},
        Case(expr, pats, branches) => {
            let mut branches_new = Vec::new();
            for branch in branches {
                branches_new.push(substitute(Rc::clone(branch), Rc::clone(&var), Rc::clone(&val)));
            }
            Rc::new(Case(substitute(Rc::clone(&expr), var, val), Vec::clone(pats), branches_new))
        },
        _ => body
    }
}


fn whnf(expr: Rc<Expr>) -> Rc<Expr> {
    let spine = &mut Vec::new();
    let top = redex(expr, spine);
    whnf_rep(top, spine)
}

fn whnf_rep(expr: Rc<Expr>, spine: &mut Vec<Rc<Expr>>) -> Rc<Expr> {
    let mut modified = false;
    let expr_next = whnf_step(Rc::clone(&expr), spine, &mut modified);
    println!("expr: {}, modified: {}", expr_next, modified);
//    if expr == expr_next && !modified {
//        expr
//    } else {
//        whnf_rep(expr_next, spine)
//    }

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
//        match Rc::deref(&expr_next) {
//            Apply(_, _) => whnf_rep(expr_next,spine),
//            _ => expr_next
//        }
//        expr_next
        whnf_rep(expr_next, spine)
    }
}
// modified marks whether or not any modification actually happened, if it didn't we are done evaluating (maybe)
fn whnf_step(expr: Rc<Expr>, spine: &mut Vec<Rc<Expr>>, modified: &mut bool) -> Rc<Expr> {
    match &*expr {
        Apply(left, right) => {
            *modified = true;
            redex(expr, spine)
        },
        Lambda(head, body) => {
//            println!("Lambda step");
            let arg = spine.pop();
            match arg {
                Some(x) => {
                    let expr_new = substitute(Rc::clone(body), Rc::clone(head), x);
                    *modified = true;
                    expr_new
//                    whnf_step(expr_new, spine, modified)
                }
                _ => expr
            }
        },
        LetRec(vars, vals, let_expr, env) => {
//            println!("Let step");
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
            // push the definitions into the expression
//            push_scope(&expr, Rc::new(definitions));
            // take a step
            *modified = true;
            Rc::clone(&let_expr)
//            whnf_step(Rc::clone(&let_expr), spine, modified)
        },
//        LetRec(vars, vals, _) => {
//            let mut definitions = HashMap::new(); // the definitions to add
//            // create the definitions to add
//            for i in 0..vars.len() {
//                match Rc::deref(&vars[i]) {
//                    Variable(s, _) => scope.insert(s, Rc::clone(&vals[i])),
//                    _ => panic!("Can only define variables.")
//                }
//            }
//            // update the environment for the expression
//            let mut new_env = Rc::new(Env::Context(Rc::new(definitions), Rc::clone(env)));
//
//            // make the definitions in the new_env use weak pointers to new_env
//            let weak_env = Rc::downgrade(&new_env);
////            new_env.change_ptr(weak_env);
//
//            // have the expression be updated to point at new_nev
//
//
//
////            let expr_updated = update_environment(expr, definitions);
////            let expr_updated = push_scope(expr, Rc::new(scope));
////            match Rc::deref(&expr_updated) {
////                LetRec(_, _, let_expr) => whnf_step(Rc::clone(&let_expr), spine, modified),
////                _ => panic!("Something weird happened when pushing scope into let.")
////            }
////            whnf_step(expr_updated, spine, modified)
//        }
        Variable(s, env) => {
//            println!("Variable step");
            if let Some(val) = env.borrow().lookup(s) {
                println!("Looking up {} with {}", s, env.borrow());
                *modified = true;
                Rc::clone(val)
//                whnf_step(Rc::clone(val), spine, modified)
            } else {
                panic!("Variable not defined.")
            }
        }
        Builtin(args, _, func) => {
//            println!("Builtin step");
            let mut is_mod = false;
            if (*args as usize) <= spine.len() {
                let mut inputs = Vec::new();
                for i in 0..*args {
                    if let Some(a) = spine.pop() {
                        inputs.push(a);
                    }
                }
                for i in 0..inputs.len() {
                    let mut m = false;
                    let update = whnf_step(Rc::clone(&inputs[i]), spine, &mut m);
                    is_mod = is_mod || m;
                    inputs[i] = update;
                }
                let res:Rc<Expr> = func(inputs);
                *modified = is_mod;
                res
//                whnf_step(res, spine, modified)
            } else {
                expr
            }
        },
        Data(args, show, typ, initialized, _) => {
//            println!("Data step");
            if !initialized {
                if (*args as usize) <= spine.len() {
                    let mut inputs = Vec::new();
                    for i in 0..*args {
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
        Case(case_expr, pats, branches) => {
//            println!("Case step");
            // the case_expr needs to be in whnf to be pattern matched on
            match Rc::deref(case_expr) {
//                Apply(_, _) => {
//                    // need to continue evaluating
////                    let (case_update, modified) = whnf_step(Rc::clone(case_expr), spine, modified);
//                    let case_update = whnf(Rc::clone(case_expr));
//                    (Rc::new(Case(case_update, Vec::clone(pats), Vec::clone(branches))), modified)
//                },
//                Variable(s, env) => {
//                    // its a variable, so look it up
//                    let case_update = whnf(Rc::clone(case_expr));
//                    (Rc::new(Case(case_update, Vec::clone(pats), Vec::clone(branches))), modified)
//                }
                Data(_, _, show, initialized, fields) => {
                    let mut branch = None;
                    for i in 0..pats.len() {
                        match Rc::deref(&pats[i]) {
                            Data(_, _, s, _, vars) => {
                                if s.eq(show) {
                                    branch = Some(Rc::new(LetRec(vars.to_vec(), fields.to_vec(), Rc::clone(&branches[i]), RefCell::new(Rc::new(Env::new())))));
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
                    Rc::new(Case(case_update, Vec::clone(pats), Vec::clone(branches)))
//                    println!("{}", case_expr);
//                    panic!("Case only pattern matches on data types.")
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
        Variable(s, old_env) => {
            *old_env.borrow_mut() = Rc::clone(env);
        }
        _ => {},
    }
}

//fn push_scope(expr: &Rc<Expr>, scope: Rc<HashMap<String, Rc<Expr>>>) {
//    match Rc::deref(expr) {
//        Lambda(head, body) => {
//            push_scope(&body, scope);
//        },
//        Apply(left, right) => {
//            push_scope(&left, Rc::clone(&scope));
//            push_scope(&right, scope);
//        }
//        LetRec(vars, vals, let_expr) => {
//            for i in 0..vals.len() {
//                push_scope(&vals[i], Rc::clone(&scope));
//            }
//            push_scope(&let_expr, scope);
//        }
//        Variable(s, env) => {
////            let env_new = env.borrow().add(scope);
//
//            let env_new = Env::clone(Rc::deref(&*env.borrow())).add(scope);
////            let env_new = Env::clone(Rc::deref(env.borrow())).add(scope);
//            *env.borrow_mut() = Rc::new(env_new);
////            env.replace(Rc::new(env.borrow().add(scope)));
////            let env_new = env..add(scope);
////            *env.borrow_mut() = Rc::new(env.borrow().add(scope));
//        },
//        _ => {},
//    }
//}

//fn update_environment(expr: Rc<Expr>, definitions: Rc<HashMap<String, Rc<Expr>>>) -> Rc<Expr> {
//    match expr {
//        LetRec(vars, vals, body) => {
//            // need
//        }
//        _ => expr,
//    }
//}
//
//fn push_scope(expr: Rc<Expr>, scope: Rc<HashMap<String, Rc<Expr>>>) -> Rc<Expr> {
//    match &*expr {
//        Lambda(head, body) => {
////            push_scope(body, scope);
//            Rc::new(Lambda(Rc::clone(head), push_scope(Rc::clone(body), scope)))
//        },
//        Apply(left, right) => {
////            push_scope(left, Rc::clone(&scope));
////            push_scope(right, Rc::clone(&scope));
//            Rc::new(Apply(push_scope(Rc::clone(&left), Rc::clone(&scope)), push_scope(Rc::clone(&right), scope)))
//        },
//        LetRec(vars, vals, let_expr) => {
//            let mut new_vals = Vec::new();
//            for i in 0..vars.len() {
//                new_vals.push(push_scope(Rc::clone(&vals[i]), Rc::clone(&scope)));
//            }
//            Rc::new(LetRec(Vec::clone(vars), new_vals, push_scope(Rc::clone(let_expr), Rc::clone(&scope))))
//        },
//        Variable(s, env) => {
//            Rc::new(Variable(s.to_string(), Rc::new(Env::clone(env).add(scope))))
//        },
//        Case(case_expr, pats, branches) => {
//            let mut new_branches = Vec::new();
//            for i in 0..pats.len() {
//                new_branches.push(push_scope(Rc::clone(&branches[i]), Rc::clone(&scope)));
//            }
//            Rc::new(Case(push_scope(Rc::clone(case_expr), scope), Vec::clone(pats), new_branches))
//        },
//        _ => expr,
//    }
//}


fn make_var(sym: &str) -> Rc<Expr> {
    Rc::new(Variable(sym.to_string(), RefCell::new(Rc::new(Env::new()))))
}

fn make_lam(head: Rc<Expr>, body: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Lambda(head, body))
}

fn make_app(left: Rc<Expr>, right: Rc<Expr>) -> Rc<Expr> {
    Rc::new(Apply(left, right))
}

fn add_func(args: Vec<Rc<Expr>>) -> Rc<Expr> {
    let a = Rc::clone(&args[0]);
    let b = Rc::clone(&args[1]);
    match (&*a,&*b) {
        (Number(n1), Number(n2)) => Rc::new(Number(n1+n2)),
        _ => panic!("Can only add numbers.")
    }
}


pub mod parse;
use parse::*;

use std::{env, thread};
use std::fs;
use std::cell::RefCell;
use crate::environment::Env::Context;

use std::thread::Builder;

fn main() {
//    let nil = Rc::new(Data(0, "list".to_string(), "Nil".to_string(), false, Vec::new()));
//    let cons = Rc::new(Data(2, "list".to_string(), "Cons".to_string(), false, Vec::new()));
//    let add = Rc::new(Builtin(2, "+".to_string(), |x| add_func(x)));
//    let bot = Rc::new(Bottom);
//    let a = make_var("a");
//    let b = make_var("b");
//    let x = make_var("x");
//    let y = make_var("y");
//    let z = make_var("z");
//    let n = Rc::new(Number(1.0));
//    let n2 = Rc::new(Number(2.0));
//    let t = make_lam(Rc::clone(&x), make_lam(Rc::clone(&y), Rc::clone(&x)));
//    let f = make_lam(Rc::clone(&x), make_lam(Rc::clone(&y), Rc::clone(&y)));
//    let and = make_lam(Rc::clone(&x), make_lam(Rc::clone(&y), make_app(make_app(Rc::clone(&x), Rc::clone(&y)), Rc::clone(&x))));
//    let id = make_lam(Rc::clone(&x), Rc::clone(&x));
//    let cons_pat = Rc::new(Data(2, "list".to_string(), "Cons".to_string(), true, vec!(Rc::clone(&x), Rc::clone(&y))));
//    let nil_pat = Rc::new(Data(0, "list".to_string(), "Nil".to_string(), true, Vec::new()));
//    let head_case = Rc::new(Case(Rc::clone(&z), vec!(Rc::clone(&cons_pat), Rc::clone(&nil_pat)), vec!(Rc::clone(&x), Rc::clone(&bot))));
//    let head = make_lam(Rc::clone(&z), head_case);
//    let tail_case = Rc::new(Case(Rc::clone(&z), vec!(Rc::clone(&cons_pat), Rc::clone(&nil_pat)), vec!(Rc::clone(&y), Rc::clone(&bot))));
//    let tail = make_lam(Rc::clone(&z), tail_case);
//    let inf_list = Rc::new(LetRec(vec!(Rc::clone(&x)), vec!(make_app(make_app(cons, n), Rc::clone(&x))), make_app(tail, Rc::clone(&x))));
////    let expr = make_app(make_app(and, Rc::clone(&t)), Rc::clone(&t));
////    let test = Rc::new(LetRec(vec!(Rc::clone(&x), Rc::clone(&y)), vec!(n, b), x));
////    let expr = Rc::new(LetRec(vec!(Rc::clone(&z)), vec!(test), Rc::clone(&z)));
////    let expr = make_app(make_app(add, Rc::new(Number(1.0))), Rc::new(Number(10.0)));
////    let list = make_app(make_app(Rc::clone(&cons), n), nil);
////    let list2 = make_app(make_app(Rc::clone(&cons), n2), list);
//    let expr = make_app(head, inf_list);
//
//    let source = "\\ x . x".to_string();
//    println!("{:?}", source);
//    println!();
//    println!("{:?}", lex(String::clone(&source)));
//    println!();
//    println!("{}", parse(lex(String::clone(&source))));
//    println!();
//    println!("{:?}", whnf(parse(lex(source))));


    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Couldn't read file.");
    println!("{:?}", source);
    println!();
    println!("{:?}", lex(String::clone(&source)));
    println!();
    println!("{}", parse(lex(String::clone(&source))));
    println!();
    println!("{}", whnf(parse(lex(source))));

//        let builder = thread::Builder::new()
//        .name("reductor".into())
//        .stack_size(32 * 1024 * 1024); // 32MB of stack space
//
//
//
//    let handler = builder.spawn(|| {
//        let x = make_var("x");
//        let i = make_var("i");
//        let i2 = make_var("i2");
//        let id = make_lam(Rc::clone(&x), Rc::clone(&x));
//        let id2 = make_lam(Rc::clone(&x), make_app(Rc::clone(&i), Rc::clone(&x)));
//        let expr = Rc::new(LetRec(vec!(Rc::clone(&i), Rc::clone(&i2)), vec!(Rc::clone(&id), Rc::clone(&id2)), make_app(make_app(Rc::clone(&i2), Rc::clone(&i2)), Rc::clone(&i)), RefCell::new(Rc::new(Env::new()))));
//
//
////        println!("{}", expr);
////        println!();
////        println!("{}", whnf(Rc::clone(&expr)));
////        println!("{}", whnf(whnf(expr)));
//    }).unwrap();
//
//    handler.join().unwrap();



}
