use crate::env::Env;
use num::bigint::BigInt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Var(String, RefCell<usize>), // has to store the associated environment
    Int(BigInt),
    Float(f64),
    Lam(Rc<Expr>, Rc<Expr>),
    App(Rc<Expr>, Rc<Expr>),
    Let(Vec<Rc<Expr>>, Vec<Rc<Expr>>, Rc<Expr>), // vars, defs, body
    Data(usize, String, String, Vec<Rc<Expr>>),  // arguments, type, constructor, fields
    Case(Rc<Expr>, Vec<Pattern>, Vec<Rc<Expr>>), // expression, patterns, branches
    Lazy(Rc<Expr>),                              // unevaluated value
    Force(Rc<Expr>),                             // force a lazy value
    If(Rc<Expr>, Rc<Expr>, Rc<Expr>),            // condition, branch 1, branch 2
    Builtin(usize, String, fn(Vec<Rc<Expr>>) -> Rc<Expr>, Vec<Rc<Expr>>), //arguments, representation, list of args to result, fields
    Bottom,
}

use crate::ast::Expr::*;

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Var(s, _) => write!(f, "{}", s),
            Lam(head, body) => write!(f, "(\\ {} . {})", head, body),
            App(left, right) => write!(f, "({} {})", left, right),
            Int(n) => write!(f, "{}", n),
            Float(n) => write!(f, "{}", n),
            Data(_args, _typ, str, fields) => {
                write!(f, "({}", str)?;
                for field in fields {
                    write!(f, " {}", field)?;
                }
                write!(f, ")")
            }
            Let(vars, defs, body) => {
                write!(f, "let ")?;
                for i in 0..vars.len() {
                    write!(f, "{} = {}; ", vars[i], defs[i])?;
                }
                write!(f, "in {}", body)
            }
            Case(expr, pats, branches) => {
                write!(f, "case {} of [", expr)?;
                for i in 0..pats.len() {
                    write!(f, "{} -> {}; ", pats[i], branches[i])?;
                }
                write!(f, "]")
            }
            Lazy(expr) => write!(f, "~{}", expr),
            Force(expr) => write!(f, "force {}", expr),
            If(expr, b1, b2) => write!(f, "if {} {} {}", expr, b1, b2),
            Builtin(_, str, _, fields) => {
                write!(f, "({}", str)?;
                for field in fields {
                    write!(f, " {}", field)?;
                }
                write!(f, ")")
            }
            Bottom => write!(f, "_|_"),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Var(s1, _), Var(s2, _)) => s1 == s2,
            _ => false,
        }
    }
}

// more convenient to hold the top level definitions rather than trying to copmress it into a single function to evaluate
#[derive(Debug, Clone)]
pub struct Toplevel {
    pub data: Vec<Definition>,
    pub defs: Vec<Definition>,
}

impl Display for Toplevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "data:\n")?;
        for d in &self.data {
            write!(f, "  {}\n", d)?;
        }
        write!(f, "defs:\n")?;
        for d in &self.defs {
            write!(f, "  {}\n", d)?;
        }
        write!(f, "")
    }
}

impl Toplevel {
    // convert the toplevel definition to a single let expression
    pub fn to_let(&self) -> Rc<Expr> {
        let mut vars = Vec::new();
        let mut defs = Vec::new();
        for d in &self.data {
            vars.push(Rc::clone(&d.assign));
            defs.push(Rc::clone(&d.def));
        }
        for d in &self.defs {
            vars.push(Rc::clone(&d.assign));
            defs.push(Rc::clone(&d.def));
        }
        Rc::new(Let(
            vars,
            defs,
            Rc::new(Var("main".to_string(), RefCell::new(1))),
        ))
    }

    // convert toplevel to environment definitions
    pub fn to_env(&self) -> Env {
        let env = Env::Empty;
        let mut defs = HashMap::new();
        for d in &self.data {
            if let Expr::Var(s, _) = &*d.assign {
                defs.insert(s.to_string(), Rc::clone(&d.def));
            }
        }
        for d in &self.defs {
            if let Expr::Var(s, _) = &*d.assign {
                defs.insert(s.to_string(), Rc::clone(&d.def));
            }
        }
        Env::Context(defs, Rc::new(env))
    }
}

#[derive(Debug, Clone)]
pub struct Definition {
    assign: Rc<Expr>, // the key to assign to, either a constructor or a variable
    def: Rc<Expr>,    //the definition
}

impl Definition {
    pub fn new(assign: Rc<Expr>, def: Rc<Expr>) -> Definition {
        Definition { assign, def }
    }
}

impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.assign, self.def)
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Irrefutable(String),            // just a variable, variables always match
    Construct(String, Vec<String>), // constructor name and the variables
}

use crate::ast::Pattern::*;

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Irrefutable(s) => write!(f, "{}", s),
            Construct(cons, vars) => {
                write!(f, "{}", cons)?;
                for var in vars {
                    write!(f, " {}", var)?;
                }
                write!(f, "")
            }
        }
    }
}
