/*

Assume all lets are letrecs and then transform them into the proper lets and letrecs by analyzing the dependency graph

Now onto type checking
    have everything worked out except cases

*/

pub mod ast;
pub mod builtins;
pub mod check;
pub mod env;
pub mod eval;
pub mod info;
pub mod rearrange;
pub mod scan;

extern crate num;

// use crate::ast::Expr;
use crate::env::Env;
use crate::rearrange::change_lets;
// use std::cell::RefCell;
// use std::collections::HashMap;
use std::env as other_env;
use std::fs;
use std::rc::Rc;

use crate::eval::eval;
use crate::scan::resolve;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub gram); // synthesized by LALRPOP

fn main() {
    let args: Vec<String> = other_env::args().collect();
    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Couldn't read file.");

    // let str = "Bool = True | False; Maybe a = Some a | None; List a = Cons a (List a) | Nil; head = (\\ x . case x {Cons a as -> Some a; Nil -> None}); not = (\\x . case x {True -> False; False -> True}); main = (head (Nil))";
    let parse = gram::TopParser::new().parse(&source).unwrap();
    let expr = parse.to_let();
    resolve(Rc::clone(&expr), 0);
    let expr = change_lets(expr);
    // println!("{}", expr);
    // println!("{}", expr);
    // let env = Rc::new(parse.to_env());
    // let expr = Rc::new(Expr::Var("main".to_string(), RefCell::new(1)));
    // println!("environment:\n\t{}\nexpr:\n\t{}", env, expr);
    // println!("{}", eval(expr, env));
    println!("{}", eval(expr, Rc::new(Env::Empty), Vec::new()));
    // println!("{}", expr);
}
