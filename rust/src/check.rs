/*

checking for properly formed syntax
for now just simple true/false eventually want information on the issues

Things to check:
    case patterns unique variables
    case statements are total
    shadowing

*/

use crate::ast::Expr;
use crate::ast::Pattern;
// use crate::info::DataInfo;
// use std::ops::Deref;
use std::rc::Rc;

fn check_pattern(pat: &Pattern) -> bool {
    // only way to fail is if contructor pattern reuses variable names
    match pat {
        Pattern::Construct(_, vars) => {
            let mut found = Vec::new();
            for var in vars {
                if found.contains(&var) {
                    return false;
                } else {
                    found.push(var);
                }
            }
            true
        }
        _ => true,
    }
}

// // unfortunately not carrying around the right information, can't do it right now
// fn check_total(info: DataInfo, expr: Rc<Expr>) -> bool {
//     // only works on case statements
//     // either there is a match all pattern or all contructors are found
//     // need to provide proper type info to know which constructors to look for
//     if let Expr::Case(_, pats, _) = Rc::deref(&expr) {
//         let mut found = Vec::new();
//         for pat in pats {
//             match pat {
//                 Pattern::Wildcard => return true,
//                 Pattern::Irrefutable(_) => return true,
//                 Pattern::Construct(name, _) => found.push(name.to_string()),
//                 _ => continue,
//             }
//         }
//         let mut names = Vec::new();
//         for def in info.data_info.alts {
//             names.push(def.name);
//         }
//         if names.len() != found.len() {
//             false
//         } else {
//             for name in names {
//                 if found.contains(&name) {
//                     continue;
//                 } else {
//                     return false;
//                 }
//             }
//             true
//         }
//     } else {
//         panic!("Can only check cases for being total.")
//     }
// }

pub fn check_cases(expr: Rc<Expr>) -> bool {
    // go through ast and check all cases statements, if any are false it fails
    match &*expr {
        Expr::Case(_, pats, branches) => {
            for pat in pats {
                if check_pattern(pat) {
                    continue;
                } else {
                    return false;
                }
            }
            for branch in branches {
                if check_cases(Rc::clone(branch)) {
                    continue;
                } else {
                    return false;
                }
            }
            true
        }
        Expr::Lam(_, body) => check_cases(Rc::clone(body)),
        Expr::App(left, right) => check_cases(Rc::clone(left)) && check_cases(Rc::clone(right)),
        Expr::Let(_, defs, expr) => {
            for def in defs {
                if check_cases(Rc::clone(def)) {
                    continue;
                } else {
                    return false;
                }
            }
            check_cases(Rc::clone(expr))
        }
        Expr::LetRec(_, defs, expr) => {
            for def in defs {
                if check_cases(Rc::clone(def)) {
                    continue;
                } else {
                    return false;
                }
            }
            check_cases(Rc::clone(expr))
        }
        Expr::If(cond, b1, b2) => {
            check_cases(Rc::clone(cond)) && check_cases(Rc::clone(b1)) && check_cases(Rc::clone(b2))
        }
        _ => true,
    }
}

// determin if there are any variables being shadowed
pub fn shadowing(expr: Rc<Expr>, defined: Vec<Rc<Expr>>) -> bool {
    // traverse the tree and see if variable names are reused down lets, letrecs, and lambdas
    match &*expr {
        Expr::Lam(head, body) => {
            if defined.contains(head) {
                return true;
            } else {
                let mut defined = defined;
                defined.push(Rc::clone(head));
                shadowing(Rc::clone(body), defined.clone())
            }
        }
        Expr::Let(vars, defs, expr) => {
            // need to pass down definitions individually and all collected into the expr
            let mut def_expr = defined.clone();
            for i in 0..vars.len() {
                if defined.contains(&vars[i]) {
                    return true;
                } else {
                    let mut def_individual = defined.clone();
                    def_individual.push(Rc::clone(&vars[i]));
                    def_expr.push(Rc::clone(&vars[i]));
                    if shadowing(Rc::clone(&defs[i]), def_individual) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
            shadowing(Rc::clone(expr), def_expr)
        }
        Expr::LetRec(vars, defs, expr) => {
            // due to recursion all variables are available to all branches
            // collect all variables first
            let mut defined = defined.clone();
            for var in vars {
                defined.push(Rc::clone(var));
            }
            for def in defs {
                if shadowing(Rc::clone(def), defined.clone()) {
                    return true;
                } else {
                    continue;
                }
            }
            shadowing(Rc::clone(expr), defined)
        }
        Expr::Var(_, _) => defined.contains(&expr),
        Expr::App(left, right) => {
            shadowing(Rc::clone(left), defined.clone()) || shadowing(Rc::clone(right), defined)
        }
        Expr::Case(expr, _, branches) => {
            for branch in branches {
                if shadowing(Rc::clone(&branch), defined.clone()) {
                    return true;
                } else {
                    continue;
                }
            }
            shadowing(Rc::clone(expr), defined.clone())
        }
        Expr::If(cond, b1, b2) => {
            shadowing(Rc::clone(cond), defined.clone())
                || shadowing(Rc::clone(b1), defined.clone())
                || shadowing(Rc::clone(b2), defined)
        }
        _ => false,
    }
}
