/*

definitions for some builtin functions

*/

use crate::ast::Expr;
use std::ops::Deref;
use std::rc::Rc;

pub fn add(args: Vec<Rc<Expr>>) -> Rc<Expr> {
    match Rc::deref(&args[0]) {
        Expr::Int(a) => {
            if let Expr::Int(b) = Rc::deref(&args[1]) {
                Rc::new(Expr::Int(a + b))
            } else {
                panic!(
                    "Can only add numbers of the same type. {} {}",
                    args[0], args[1]
                );
            }
        }
        Expr::Float(a) => {
            if let Expr::Float(b) = Rc::deref(&args[1]) {
                Rc::new(Expr::Float(a + b))
            } else {
                panic!("Can only add numbers of the same type.")
            }
        }
        _ => panic!("Can only add numbers."),
    }
}

pub fn sub(args: Vec<Rc<Expr>>) -> Rc<Expr> {
    match Rc::deref(&args[0]) {
        Expr::Int(a) => {
            if let Expr::Int(b) = Rc::deref(&args[1]) {
                Rc::new(Expr::Int(a - b))
            } else {
                panic!("Can only subtract numbers of the same type.");
            }
        }
        Expr::Float(a) => {
            if let Expr::Float(b) = Rc::deref(&args[1]) {
                Rc::new(Expr::Float(a - b))
            } else {
                panic!("Can only subtract numbers of the same type.")
            }
        }
        _ => panic!("Can only subtract numbers."),
    }
}

pub fn mult(args: Vec<Rc<Expr>>) -> Rc<Expr> {
    match Rc::deref(&args[0]) {
        Expr::Int(a) => {
            if let Expr::Int(b) = Rc::deref(&args[1]) {
                Rc::new(Expr::Int(a * b))
            } else {
                panic!("Can only multiply numbers of the same type.");
            }
        }
        Expr::Float(a) => {
            if let Expr::Float(b) = Rc::deref(&args[1]) {
                Rc::new(Expr::Float(a * b))
            } else {
                panic!("Can only multiply numbers of the same type.")
            }
        }
        _ => panic!("Can only multiply numbers."),
    }
}

pub fn div(args: Vec<Rc<Expr>>) -> Rc<Expr> {
    match Rc::deref(&args[0]) {
        Expr::Int(a) => {
            if let Expr::Int(b) = Rc::deref(&args[1]) {
                Rc::new(Expr::Int(a / b))
            } else {
                panic!("Can only divide numbers of the same type.");
            }
        }
        Expr::Float(a) => {
            if let Expr::Float(b) = Rc::deref(&args[1]) {
                Rc::new(Expr::Float(a / b))
            } else {
                panic!("Can only divide numbers of the same type.")
            }
        }
        _ => panic!("Can only divide numbers."),
    }
}

pub fn eq(args: Vec<Rc<Expr>>) -> Rc<Expr> {
    match Rc::deref(&args[0]) {
        Expr::Int(a) => {
            if let Expr::Int(b) = Rc::deref(&args[1]) {
                if a == b {
                    Rc::new(Expr::Data(
                        0,
                        "Bool".to_string(),
                        "True".to_string(),
                        Vec::new(),
                    ))
                } else {
                    Rc::new(Expr::Data(
                        0,
                        "Bool".to_string(),
                        "False".to_string(),
                        Vec::new(),
                    ))
                }
            } else {
                panic!("Can only equate numbers of the same type.");
            }
        }
        Expr::Float(a) => {
            if let Expr::Float(b) = Rc::deref(&args[1]) {
                if a == b {
                    Rc::new(Expr::Data(
                        0,
                        "Bool".to_string(),
                        "True".to_string(),
                        Vec::new(),
                    ))
                } else {
                    Rc::new(Expr::Data(
                        0,
                        "Bool".to_string(),
                        "False".to_string(),
                        Vec::new(),
                    ))
                }
            } else {
                panic!("Can only equate numbers of the same type.")
            }
        }
        _ => panic!("Can only dequate numbers."),
    }
}
