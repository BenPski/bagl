/*

Now want to specialize the top level definitions to make the syntax more pleasant

because of laziness need a specialized definition to start evaluation

want to have the let be implicit for the top level and main is the place to enter evaluation

id = \x.x

main = id 1

is equivalent to

let
    id = \x.x
    main = id 1
in main

now parsing could be something like parsing a list of definitions and then converted to the let expression

could also handle the mixing of tokens and expressions slightly differently
    have an enum of: tokens, sugar, and expressions
    use same recursive ascent parsing strategy (is it really recursive ascent?)

also the lexer is gross with the edge case checking

need to check if recursive calls with blow the stack as well in evaluation or change it to mutation


Starting to include the syntactic sugar into the parsing

first things are:
    \x y . z => \x.\y.z
    f x y = z => f = \x.\y.z

could sort of deal with these internal to parsing or could separate them into specific sugar structures and then convert them

Now have the desired syntax sugar for function definitions and now want data constructor definitions

Bool = True | False
    true = Data(0, "Bool", "True",false,Vec::new())
    false = Data(0, "Bool", "False", false, Vec::new())

List a = Cons a (List a) | Nil
    cons = Data(2, "List", "Cons", false, Vec::new())
    nil = Data(0, "List", "Nil", false, Vec::new())

also do
Bool = True
     | False

 List a = Cons a (List a)
        | Nil


So for dealing with data constructors may just want to define the variables to be looked up in the environment

Bool = True | False
main = True
=>
let
    True = Data()
    False = Data()
    main = True
in main

got cases and data working however a couple issues
    environemnt and variables not being properly passed through case it seems
    pattern is strict in variables that it shouldn't be


*/
use std::env;
use std::fs;

pub mod environment;
pub mod expr;
pub mod either;
pub mod builtin;

pub mod parse;
use parse::*;

//pub mod parse_old;
//use parse_old::*;

pub mod eval;
use eval::*;


fn main() {
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

}
