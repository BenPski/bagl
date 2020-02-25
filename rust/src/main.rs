/*
switching to strict evaluation, just probably a better approach when interpreting things

    will have to buitld in laziness somehow

still running into issues with where environments get mixed up
    not quite sure how I'd like to do it
    can store references to the environment in the variables
    store some sort of depth in the environment
    closures?

*/

pub mod ast;
pub mod builtins;
pub mod env;
pub mod eval;
pub mod info;

extern crate num;

use crate::ast::Expr;
use std::cell::RefCell;
use std::env as other_env;
use std::fs;
use std::rc::Rc;

use crate::eval::eval;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub gram); // synthesized by LALRPOP

fn main() {
    let args: Vec<String> = other_env::args().collect();
    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Couldn't read file.");

    // let str = "Bool = True | False; Maybe a = Some a | None; List a = Cons a (List a) | Nil; head = (\\ x . case x {Cons a as -> Some a; Nil -> None}); not = (\\x . case x {True -> False; False -> True}); main = (head (Nil))";
    let parse = gram::TopParser::new().parse(&source).unwrap();
    let env = Rc::new(parse.to_env());
    let expr = Rc::new(Expr::Var("main".to_string(), RefCell::new(1)));
    // println!("environment:\n\t{}\nexpr:\n\t{}", env, expr);
    println!("{}", eval(expr, env));
}
