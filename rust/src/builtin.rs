use std::rc::Rc;

use crate::expr::*;
use crate::expr::Expr::*;



// addition
pub fn add_func(args: Vec<Rc<Expr>>) -> Rc<Expr> {
    let a = Rc::clone(&args[0]);
    let b = Rc::clone(&args[1]);
    match (&*a,&*b) {
        (Integer(n1), Integer(n2)) => Rc::new(Integer(n1+n2)),
        (Double(n1), Double(n2)) => Rc::new(Double(n1+n2)),
        _ => panic!("Can only add numbers that are the same type.")
    }
}