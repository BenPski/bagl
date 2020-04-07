/*

Assume all lets are letrecs and then transform them into the proper lets and letrecs by analyzing the dependency graph

Now onto type checking
    have everything worked out except cases


have to be able to extract constructor signatures from the parsing
    add them to toplevel during parsing


technically works right now, but I think it would be better to have the Data in the ast store the type info rather than looking it
    would have to collect the definitions in the toplevel and then push them into the ast later
    can leave it for now


for doing case checking we know case is
    case: Data -> x

then each branch is a let with the special selector functions

case data
    pat1 -> b1
    pat2 -> b2

case data (let var = select data in b1) (let var = select data in b2)


case list
    Cons a as -> a
    Nil -> 0

case list (let a = select-cons-1 list; as = select-cons-2 list in a) (0)



I suppose type checking an if statement is a simpler to understand case
    it is a case statement on a boolean
    two branches
    no destructuring data

if: Bool -> a -> a -> a
    takes boolean and returns the appropriate branch of the two given

*/

pub mod ast;
pub mod builtins;
pub mod check;
pub mod env;
pub mod eval;
pub mod info;
pub mod rearrange;
pub mod scan;
pub mod typecheck;

extern crate num;

// use crate::ast::Expr;
use crate::env::Env;
use crate::rearrange::change_lets;
use crate::typecheck::tenv_from_sigs;
use crate::typecheck::type_check_def;
// use std::cell::RefCell;

use std::env as other_env;
use std::fs;
use std::rc::Rc;

use crate::eval::eval;
use crate::scan::resolve;
use crate::typecheck::Scheme;
use crate::typecheck::TExpr;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub gram); // synthesized by LALRPOP

fn main() {
    let args: Vec<String> = other_env::args().collect();
    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Couldn't read file.");

    // let str = "Bool = True | False; Maybe a = Some a | None; List a = Cons a (List a) | Nil; head = (\\ x . case x {Cons a as -> Some a; Nil -> None}); not = (\\x . case x {True -> False; False -> True}); main = (head (Nil))";
    let parse = gram::TopParser::new().parse(&source).unwrap();
    let mut sigs = tenv_from_sigs(&parse.signatures);
    // make undefined and errors inhabit all types
    sigs.insert(
        "undefined".to_string(),
        Scheme::new(vec!["a".to_string()], Rc::new(TExpr::TVar("a".to_string()))),
    );
    sigs.insert(
        "error".to_string(),
        Scheme::new(vec!["a".to_string()], Rc::new(TExpr::TVar("a".to_string()))),
    );
    let expr = parse.to_let();
    resolve(Rc::clone(&expr), 0);
    let expr = change_lets(expr);
    // println!("{:?}", sigs);
    // println!("{:?}", type_check_def(&sigs, Rc::clone(&expr)));
    // println!("{}", expr);
    // println!("{}", expr);
    // let env = Rc::new(parse.to_env());
    // let expr = Rc::new(Expr::Var("main".to_string(), RefCell::new(1)));
    // println!("environment:\n\t{}\nexpr:\n\t{}", env, expr);
    // println!("{}", eval(expr, env));
    // println!("{}", eval(expr, Rc::new(Env::Empty), Vec::new()));
    // println!("{}", expr);

    match type_check_def(&sigs, Rc::clone(&expr)) {
        None => {
            panic!("Did not type check.");
        }
        _ => println!("{}", eval(expr, Rc::new(Env::Empty), Vec::new())),
    }
}
