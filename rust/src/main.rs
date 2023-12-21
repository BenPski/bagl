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

pub mod core;
pub mod pretty;
pub mod parse;

use std::fs;

use crate::{core::*, parse::lexer::lexer};

fn main() {
    
    let main = Supercombinator::new(
            String::from("main"),
            vec![],
            Expr::app(Expr::var("double"), Expr::num(1))
        );
    let double = Supercombinator::new(
            String::from("double"),
            vec![String::from("x")],
            Expr::app(
                Expr::app(
                    Expr::var("+"),
                    Expr::var("x")
                ),
                Expr::var("x")
            )
        );
    let prog = Program::new(vec![main, double]);
    println!("{}", prog);

    let file_path = "/home/ben/stuff/bagl/rust/blah.bagl";
    let contents = fs::read_to_string(file_path).expect("Should have read the given file");
    println!("{}", contents);

    println!("{:?}", lexer("blah 123".to_string()))
} 
