use crate::environment::Env;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub enum Expr {
    Variable(String, RefCell<Rc<Env>>),
    Lambda(Rc<Expr>, Rc<Expr>),
    Apply(Rc<Expr>, Rc<Expr>),
    LetRec(Vec<Rc<Expr>>, Vec<Rc<Expr>>, Rc<Expr>, RefCell<Rc<Env>>),
    Integer(i64),
    Double(f64),
    Builtin(usize, String, fn(Vec<Rc<Expr>>) -> Rc<Expr>), //arguments, representation, list of args to result
    Data(usize, String, String, bool, Vec<Rc<Expr>>), // args, type, show, initialized, fields
    Case(Rc<Expr>, Vec<Rc<Expr>>, Vec<Rc<Expr>>, RefCell<Rc<Env>>), // expr, patterns, branches, env
    Bottom,
}

use crate::expr::Expr::*;

// not technically true, but I only want to see if the variables are the same and this is the most convenient/intuitive wat
impl std::cmp::PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Variable(x, _), Variable(y, _)) => x == y,
            _ => false
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Variable(x, _) => write!(f, "{}", x),
            Apply(left, right) => write!(f, "( {} {} )", left, right),
            Lambda(head, body) => write!(f, "(\\ {} . {})", head, body),
            LetRec(vars, defs, expr, _) => {
                write!(f, "let [ ")?;
                for i in 0..vars.len() {
//                    write!(f, "{}{} = {};", format_args!("{: >1$}", "", 2), vars[i], defs[i])?;
                    write!(f, "{} = {}; ", vars[i], defs[i])?;
                }
                write!(f, "] in {}", expr)
            },
            Bottom => write!(f, "_|_"),
            Integer(n) => write!(f, "{}", n),
            Double(n) => write!(f, "{}", n),
            Builtin(_, str, _) => write!(f, "{}", str),
            Data(_, _, str, initialized, fields) => {
                if !initialized {
                    write!(f, "{}", str)
                } else {
                    write!(f, "({}", str)?;
                    for i in 0..fields.len() {
                        write!(f, " {}", fields[i])?;
                    }
                    write!(f, ")")
                }
            },
            Case(expr, pats, branches,_) => {
                write!(f, "case {} [", expr)?;
                for i in 0..pats.len() {
                    write!(f, "{} -> {}; ", pats[i], branches[i])?;
                }
                write!(f, "]")
            },
        }
    }
}