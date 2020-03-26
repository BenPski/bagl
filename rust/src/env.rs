/*
The environment for looking up variables
*/

use crate::ast::Expr;
use crate::env::Env::*;
use core::fmt::Display;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Env {
    Empty,                                       //empty environment
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
    pub fn new() -> Env {
        Empty
    }
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

    pub fn lookup_top(&self, s: &String) -> Option<&Rc<Expr>> {
        // only look at the top of the environment
        match self {
            Empty => None,
            Context(defs, _) => defs.get(s),
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            Empty => 0,
            Context(_, next) => 1 + next.depth(),
        }
    }

    // drop n environments from the top
    pub fn drop(&self, n: usize) -> &Env {
        if n == 0 {
            self
        } else {
            match self {
                Empty => self,
                Context(_, next) => Rc::deref(next).drop(n - 1),
            }
        }
    }

    // get the ith sub environment, treating list as a vector where Empty is 0
    pub fn sub_env(&self, i: usize) -> &Env {
        self.drop(self.depth() - i)
    }

    pub fn lookup_at(&self, s: &String, i: usize) -> Option<&Rc<Expr>> {
        self.sub_env(i).lookup(s)
    }

    pub fn lookup_in(&self, s: &String, i: usize) -> Option<&Rc<Expr>> {
        // drop the top i envs then try to lookup variable
        self.drop(i).lookup(s)
    }
}
