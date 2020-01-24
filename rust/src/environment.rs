use crate::environment::Env::*;
use crate::expr::Expr;
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Env {
    Empty,
    //empty environment
    Context(HashMap<String, Rc<Expr>>, Rc<Env>), //current defintions and the next environemnt up
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Empty => write!(f, "Empty"),
            Context(defs, next) => {
                write!(f, "Context(")?;
                for key in defs.keys() {
                    write!(f, "{}, ", key)?;
                }
                write!(f, "{})", next)
            }
        }
    }
}

impl Env {
    pub fn new() -> Env { Empty }
    pub fn lookup(&self, s: &String) -> Option<&Rc<Expr>> {
        match self {
            Empty => None,
            Context(defs, next) => {
                if let Some(val) = defs.get(s) {
                    Some(val)
                } else {
                    next.lookup(s)
                }
            }
        }
    }

//    pub fn add(self, defs: HashMap<String, Rc<Expr>>) -> Env {
//        Context(defs, Rc::new(self))
//    }
}
