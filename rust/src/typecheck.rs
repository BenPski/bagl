/*

type checking the ast

following from implementation of functional language

need to determine how to do cases since I'm not converting it to lambdas and special builtins

can treat each branch as lets
    need to know types of products

need to check the inputs into cases


for dealing with substitution create a data type to represent the different kinds of substitution
    couldn't figure out the functoinal way




*/

use crate::ast::Expr;
use crate::ast::Pattern;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub enum TExpr {
    TVar(String),
    TCons(String, Vec<Rc<TExpr>>),
}

use crate::typecheck::TExpr::*;

impl PartialEq for TExpr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TVar(a), TVar(b)) => a == b,
            (TCons(a, xs), TCons(b, ys)) => a == b && xs == ys,
            _ => false,
        }
    }
}

impl Display for TExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TVar(s) => write!(f, "{}", s),
            TCons(name, types) => {
                write!(f, "({}", name)?;
                for t in types {
                    write!(f, " {}", t)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug)]
pub enum Substitute {
    IdSub,
    DeltaSub(String, Rc<TExpr>),
    ComposeSub(Rc<Substitute>, Rc<Substitute>),
    ExcludeSub(Rc<Substitute>, Vec<String>),
    EnvSub(HashMap<String, String>),
}

use crate::typecheck::Substitute::*;

fn substitute_type(phi: Rc<Substitute>, texpr: Rc<TExpr>) -> Rc<TExpr> {
    match &*texpr {
        TVar(s) => phi.sub(s.to_string()),
        TCons(name, ts) => {
            let mut new_ts = Vec::new();
            for t in ts {
                new_ts.push(substitute_type(Rc::clone(&phi), Rc::clone(&t)));
            }
            Rc::new(TCons(name.to_string(), new_ts))
        }
    }
}

impl Substitute {
    fn sub(&self, name: String) -> Rc<TExpr> {
        match self {
            IdSub => Rc::new(TVar(name.to_string())),
            DeltaSub(x, texpr) => {
                if x == &name {
                    Rc::clone(texpr)
                } else {
                    Rc::new(TVar(name.to_string()))
                }
            }
            ComposeSub(phi, psi) => substitute_type(Rc::clone(phi), psi.sub(name)),
            ExcludeSub(phi, names) => {
                if names.contains(&name) {
                    Rc::new(TVar(name.to_string()))
                } else {
                    phi.sub(name)
                }
            }
            EnvSub(assoc) => {
                if assoc.contains_key(&name) {
                    if let Some(s) = assoc.get(&name) {
                        Rc::new(TVar(s.to_string()))
                    } else {
                        panic!("Idk bro")
                    }
                } else {
                    Rc::new(TVar(name))
                }
            }
        }
    }
}

fn arrow(t1: Rc<TExpr>, t2: Rc<TExpr>) -> Rc<TExpr> {
    Rc::new(TCons("arrow".to_string(), vec![t1, t2]))
}

fn from_str(s: String) -> Rc<TExpr> {
    Rc::new(TCons(s.to_string(), vec![]))
}

fn from_vec(args: Vec<String>) -> Rc<TExpr> {
    // convert list of arguments to function type
    if args.len() == 1 {
        from_str(args[0].to_string())
    } else {
        arrow(from_str(args[0].to_string()), from_vec(args[1..].to_vec()))
    }
}

pub fn add_arrows(args: Vec<Rc<TExpr>>) -> Rc<TExpr> {
    // just add arrows to vector af type areguments
    if args.len() == 1 {
        Rc::clone(&args[0])
    } else {
        arrow(Rc::clone(&args[0]), add_arrows(args[1..].to_vec()))
    }
}

// fn pair(t1: Rc<TExpr>, t2: Rc<TExpr>) -> Rc<TExpr> {
//     Rc::new(TCons("pair".to_string(), vec![t1, t2]))
// }

// fn list(t: Rc<TExpr>) -> Rc<TExpr> {
//     Rc::new(TCons("list".to_string(), vec![t]))
// }

// vector of variables that occur in a type expression
fn tvars_in(texpr: Rc<TExpr>) -> Vec<String> {
    let mut list = Vec::new();
    match &*texpr {
        TVar(s) => list.push(s.to_string()),
        TCons(_, ts) => {
            for t in ts {
                list.append(&mut tvars_in(Rc::clone(t)));
            }
        }
    }
    list
}

fn extend(phi: Rc<Substitute>, t: String, texpr: Rc<TExpr>) -> Option<Rc<Substitute>> {
    if let TVar(s) = Rc::deref(&texpr) {
        if s == &t {
            Some(Rc::clone(&phi))
        } else {
            // duplication seems dorky
            if tvars_in(Rc::clone(&texpr)).contains(&t.to_string()) {
                None
            } else {
                Some(Rc::new(ComposeSub(Rc::new(DeltaSub(t, texpr)), phi)))
            }
        }
    } else {
        if tvars_in(Rc::clone(&texpr)).contains(&t.to_string()) {
            None
        } else {
            Some(Rc::new(ComposeSub(Rc::new(DeltaSub(t, texpr)), phi)))
        }
    }
}

fn unify(phi: Rc<Substitute>, left: Rc<TExpr>, right: Rc<TExpr>) -> Option<Rc<Substitute>> {
    match (&*left, &*right) {
        (TVar(tvn), _) => {
            // is it idempotent
            let phitvn = phi.sub(tvn.to_string());
            let phit = substitute_type(Rc::clone(&phi), right);
            if phitvn == left {
                extend(phi, tvn.to_string(), phit)
            } else {
                unify(phi, phitvn, phit)
            }
        }
        // reorder
        (TCons(_, _), TVar(_)) => unify(phi, right, left),
        // unify over all subtypes
        (TCons(a, xs), TCons(b, ys)) => {
            if a == b {
                unify_list(phi, xs.to_vec(), ys.to_vec())
            } else {
                None
            }
        }
    }
}

fn unify_list(
    phi: Rc<Substitute>,
    xs: Vec<Rc<TExpr>>,
    ys: Vec<Rc<TExpr>>,
) -> Option<Rc<Substitute>> {
    let mut acc = Some(phi);
    for i in 0..xs.len() {
        match acc {
            Some(psi) => {
                acc = unify(psi, Rc::clone(&xs[i]), Rc::clone(&ys[i]));
            }
            None => {
                acc = None;
                break;
            }
        }
    }
    acc
}

#[derive(Debug, Clone)]
pub struct Scheme {
    names: Vec<String>,
    texpr: Rc<TExpr>,
}

impl Scheme {
    pub fn new(names: Vec<String>, texpr: Rc<TExpr>) -> Scheme {
        Scheme { names, texpr }
    }
}

// find the variables in the expression that are not in the scheme variables
// inefficient, but not a problem right now
fn unknowns(scheme: &Scheme) -> Vec<String> {
    let mut list = Vec::new();
    for t in tvars_in(Rc::clone(&scheme.texpr)) {
        if scheme.names.contains(&t) {
            continue;
        } else {
            list.push(t);
        }
    }
    list
}

fn scheme_sub(phi: Rc<Substitute>, scheme: &Scheme) -> Scheme {
    Scheme::new(
        scheme.names.to_vec(),
        substitute_type(
            Rc::new(ExcludeSub(phi, scheme.names.to_vec())),
            Rc::clone(&scheme.texpr),
        ),
    )
}

type TypeEnv = HashMap<String, Scheme>;
type CheckResult = Option<(Rc<Substitute>, Rc<TExpr>)>;

pub fn tenv_from_sigs(sigs: &HashMap<String, Rc<TExpr>>) -> TypeEnv {
    // associate constructor name with type signature, the scheme contains the tvars_in and the expr
    let mut tenv = HashMap::new();
    for (k, v) in sigs {
        let sch = Scheme::new(tvars_in(Rc::clone(v)), Rc::clone(v));
        tenv.insert(k.to_string(), sch);
    }
    tenv
}

fn unknowns_te(tenv: &TypeEnv) -> Vec<String> {
    let mut acc = Vec::new();
    for val in tenv.values() {
        acc.extend(unknowns(val))
    }
    acc
}

fn sub_te(phi: Rc<Substitute>, tenv: &TypeEnv) -> TypeEnv {
    let mut new = HashMap::new();
    for (key, val) in tenv.iter() {
        new.insert(key.to_string(), scheme_sub(Rc::clone(&phi), val));
    }
    new
}

trait NameSupply {
    fn next_name(&self) -> String;
    fn deplete(&self) -> Self;
    fn split(&self) -> (Self, Self)
    where
        Self: std::marker::Sized;
    fn next(&self) -> (String, Self)
    where
        Self: std::marker::Sized,
    {
        (self.next_name(), self.deplete())
    }
}

#[derive(Debug, Clone)]
struct NS {
    prefix: String,
    indices: Vec<i32>,
}

impl NameSupply for NS {
    fn next_name(&self) -> String {
        let mut s = self.prefix.to_string();
        for i in self.indices.to_vec() {
            s = s + &i.to_string();
        }
        s
    }

    fn deplete(&self) -> Self {
        let mut new_indices = self.indices.to_vec();
        new_indices[0] += 2;
        NS {
            prefix: self.prefix.to_string(),
            indices: new_indices,
        }
    }

    fn split(&self) -> (Self, Self) {
        let mut new1 = self.indices.to_vec();
        let mut new2 = self.indices.to_vec();
        new1.insert(0, 0);
        new2.insert(0, 1);
        (
            NS {
                prefix: self.prefix.to_string(),
                indices: new1,
            },
            NS {
                prefix: self.prefix.to_string(),
                indices: new2,
            },
        )
    }
}

/*

Type Checker

*/

pub fn type_check_def(init_env: &TypeEnv, expr: Rc<Expr>) -> CheckResult {
    // defaults to empty tenv and namesupply
    type_check(
        init_env,
        NS {
            prefix: "T".to_string(),
            indices: vec![0],
        },
        expr,
    )
}

fn type_check(tenv: &TypeEnv, supply: impl NameSupply + Clone, expr: Rc<Expr>) -> CheckResult {
    match &*expr {
        Expr::Var(s, _) => type_check_var(tenv, supply, s.to_string()),
        Expr::App(left, right) => {
            let res = type_check_app(tenv, supply, Rc::clone(left), Rc::clone(right));
            res
        }
        Expr::Lam(head, body) => type_check_lam(tenv, supply, Rc::clone(head), Rc::clone(body)),
        Expr::Let(vars, defs, body) => {
            type_check_let(tenv, supply, vars.to_vec(), defs.to_vec(), Rc::clone(body))
        }
        Expr::LetRec(vars, defs, body) => {
            type_check_letrec(tenv, supply, vars.to_vec(), defs.to_vec(), Rc::clone(body))
        }
        Expr::Int(_) => Some((Rc::new(IdSub), from_str("Int".to_string()))),
        Expr::Float(_) => Some((Rc::new(IdSub), from_str("Float".to_string()))),
        Expr::Str(_) => Some((Rc::new(IdSub), from_str("String".to_string()))),
        Expr::Builtin(_, _, _, _, typ) => Some((Rc::new(IdSub), from_vec(typ.to_vec()))),
        Expr::Data(_, _, cons, _) => {
            if let Some(v) = tenv.get(cons) {
                Some((Rc::new(IdSub), Rc::clone(&v.texpr)))
            } else {
                println!("Couldn't look up {}", cons);
                None
            }
        }
        Expr::If(cond, b1, b2) => {
            type_check_if(tenv, supply, Rc::clone(cond), Rc::clone(b1), Rc::clone(b2))
        }
        Expr::Case(cond, pats, branches) => type_check_case(
            tenv,
            supply,
            Rc::clone(cond),
            pats.to_vec(),
            branches.to_vec(),
        ),
        Expr::Bottom => {
            if let Some(v) = tenv.get("undefined") {
                Some((Rc::new(IdSub), Rc::clone(&v.texpr)))
            } else {
                None
            }
        }
        Expr::Error(_) => {
            if let Some(v) = tenv.get("error") {
                Some((Rc::new(IdSub), Rc::clone(&v.texpr)))
            } else {
                None
            }
        }
    }
}

fn type_check_pat(tenv: &TypeEnv, supply: impl NameSupply + Clone, pat: &Pattern) -> CheckResult {
    match pat {
        Pattern::Construct(cons, args) => {
            // let mut expr = Rc::new(Expr::Data(0, "".to_string(), cons.to_string(), Vec::new()));
            let mut expr = Rc::new(Expr::Var(cons.to_string(), RefCell::new(0)));
            let mut vars = Vec::new();
            let mut types = Vec::new();
            for arg in args {
                let var = Rc::new(Expr::Var(arg.to_string(), RefCell::new(0)));
                vars.push(Rc::clone(&var));
                types.push(Rc::new(TExpr::TVar("a".to_string()))); // just some arbitrary type
                expr = Rc::new(Expr::App(expr, var));
            }
            let sup1 = supply.deplete();
            let sup2 = sup1.deplete();
            let tenv_new = add_declarations(tenv, sup1, vars, types);
            type_check(&tenv_new, sup2, expr)
        }
        Pattern::Int(_) => Some((Rc::new(IdSub), from_str("Int".to_string()))),
        Pattern::Float(_) => Some((Rc::new(IdSub), from_str("Float".to_string()))),
        Pattern::Str(_) => Some((Rc::new(IdSub), from_str("String".to_string()))),
        _ => {
            let sub1 = supply.deplete();
            let name = sub1.next_name();
            Some((Rc::new(IdSub), Rc::new(TExpr::TVar(name))))
        }
    }
}

// this seems like a mess, could rearrange I'd hope
fn type_check_list(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    exprs: Vec<Rc<Expr>>,
) -> Option<(Rc<Substitute>, Vec<Rc<TExpr>>)> {
    if exprs.len() == 0 {
        Some((Rc::new(IdSub), Vec::new()))
    } else {
        let (supply0, supply1) = supply.split();
        type_check_list1(
            tenv,
            supply0,
            exprs[1..].to_vec(),
            type_check(tenv, supply1, Rc::clone(&exprs[0])),
        )
    }
}

fn type_check_list1(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    exprs: Vec<Rc<Expr>>,
    res: CheckResult,
) -> Option<(Rc<Substitute>, Vec<Rc<TExpr>>)> {
    match res {
        None => None,
        Some((phi, t)) => {
            let tenv_new = sub_te(Rc::clone(&phi), tenv);
            type_check_list2(
                Rc::clone(&phi),
                t,
                type_check_list(&tenv_new, supply, exprs),
            )
        }
    }
}

fn type_check_list2(
    phi: Rc<Substitute>,
    t: Rc<TExpr>,
    res: Option<(Rc<Substitute>, Vec<Rc<TExpr>>)>,
) -> Option<(Rc<Substitute>, Vec<Rc<TExpr>>)> {
    match res {
        None => None,
        Some((psi, ts)) => {
            let mut new_ts = ts;
            new_ts.insert(0, substitute_type(Rc::clone(&psi), t));
            Some((
                Rc::new(ComposeSub(Rc::clone(&psi), Rc::clone(&phi))),
                new_ts,
            ))
        }
    }
}

/*
Variables
*/
fn type_check_var(tenv: &TypeEnv, supply: impl NameSupply, name: String) -> CheckResult {
    if let Some(scheme) = tenv.get(&name) {
        let mut assoc = HashMap::new();
        let mut supply = supply;
        for i in 0..scheme.names.len() {
            let v = supply.next_name();
            supply = supply.deplete();
            assoc.insert(scheme.names[i].to_string(), v.to_string());
        }
        let phi = Rc::new(EnvSub(assoc));
        Some((
            Rc::new(IdSub),
            substitute_type(phi, Rc::clone(&scheme.texpr)),
        ))
    } else {
        panic!("Name not found in environment.")
    }
}

/*
Application
*/

fn type_check_app(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    left: Rc<Expr>,
    right: Rc<Expr>,
) -> CheckResult {
    let name = supply.next_name();
    let supply_next = supply.deplete();
    type_check_app1(name, type_check_list(tenv, supply_next, vec![left, right]))
}

fn type_check_app1(name: String, res: Option<(Rc<Substitute>, Vec<Rc<TExpr>>)>) -> CheckResult {
    match res {
        None => None,
        Some((phi, ts)) => type_check_app2(
            name.to_string(),
            unify(
                phi,
                Rc::clone(&ts[0]),
                arrow(Rc::clone(&ts[1]), Rc::new(TVar(name.to_string()))),
            ),
        ),
    }
}

fn type_check_app2(name: String, res: Option<Rc<Substitute>>) -> CheckResult {
    match res {
        None => None,
        Some(phi) => Some((Rc::clone(&phi), phi.sub(name))),
    }
}

/*
If
*/

fn type_check_if(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    cond: Rc<Expr>,
    b1: Rc<Expr>,
    b2: Rc<Expr>,
) -> CheckResult {
    let cond_check = type_check(tenv, supply.clone(), cond);
    match cond_check {
        None => None,
        Some((phi, t)) => match unify(phi, Rc::new(TCons("Bool".to_string(), Vec::new())), t) {
            None => None,
            Some(_) => {
                let name = supply.next_name();
                let supply_next = supply.deplete();
                let res = type_check_list(tenv, supply_next, vec![b1, b2]);
                match res {
                    Some((phi, ts)) => {
                        let un = unify(phi, Rc::clone(&ts[0]), Rc::clone(&ts[1]));
                        match un {
                            None => None,
                            Some(psi) => Some((Rc::clone(&psi), psi.sub(name))),
                        }
                    }
                    None => None,
                }
            }
        },
    }
}

/*
Case
*/

fn type_check_case(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    cond: Rc<Expr>,
    pats: Vec<Pattern>,
    branches: Vec<Rc<Expr>>,
) -> CheckResult {
    // this will be similar to if statements
    // however a lot more work has to be done
    // need to define selection functions
    // define lets on the branches
    // check that the condition is consistent
    // handle difference between literals and data?
    // may need to convert patterns to actual expressions to do this

    // unify condition and patterns
    // unify branches with selector functions

    let (sup1, sup2) = supply.split();
    if let Some((phi, t_first)) = type_check(tenv, sup1.clone(), cond) {
        let mut psi = Rc::clone(&phi);
        // type checking patterns
        let mut t_prev = Rc::clone(&t_first);
        for i in 0..pats.len() {
            let pat = &pats[i];
            if let Some((_, t)) = type_check_pat(tenv, sup2.clone(), pat) {
                let un = unify(Rc::clone(&phi), Rc::clone(&t_prev), Rc::clone(&t));
                match un {
                    Some(sub) => {
                        t_prev = t;
                        psi = Rc::new(ComposeSub(sub, psi));
                    }
                    None => return None,
                }
            }
        }
        // type check branches
        // need the first to check others against
        let mut sup3 = sup1.deplete();
        let mut sup4 = sup2.deplete();
        // let (mut sup3, mut sup4) = sup2.split();
        let branch = &branches[0];
        let pat = &pats[0];
        let vars = pat_vars(pat);
        let types = constructor_type(tenv, pat);
        let tenv_first = add_declarations(tenv, sup3.clone(), vars, types);
        if let Some((phi, t_firstb)) = type_check(&tenv_first, sup4.clone(), Rc::clone(branch)) {
            let phi = Rc::new(ComposeSub(phi, Rc::clone(&psi)));
            for i in 1..branches.len() {
                // need to determine variables to be assigned to
                // the values put in or the types of them
                // restructure branches to lets
                let branch = &branches[i];
                let pat = &pats[i];

                sup3 = sup3.deplete();
                sup4 = sup4.deplete();

                // get variables in pat
                // get type arguments of constructor
                // add_declarations to the env
                // type_check the branch
                let vars = pat_vars(pat);
                let types = constructor_type(tenv, pat);
                let tenv_item = add_declarations(tenv, sup3.clone(), vars, types);
                if let Some((_, t_item)) = type_check(&tenv_item, sup4.clone(), Rc::clone(branch)) {
                    let un = unify(Rc::clone(&phi), Rc::clone(&t_firstb), t_item);
                    match un {
                        Some(sub) => {
                            psi = Rc::new(ComposeSub(sub, psi));
                        }
                        None => return None,
                    }
                }
            }
            return Some((psi, t_firstb));
            // return Some((psi, t_first));
        }
        None
    } else {
        None
    }
}

// fn pat_to_texpr(pat: Pattern, supply: impl NameSupply + Clone) -> Rc<TExpr> {
//     match pat {
//         Pattern::Wildcard => {
//             let name = supply.next_name();
//             Rc::new(TExpr::TVar(name))
//         }
//         Pattern::Irrefutable(_) => {
//             let name = supply.next_name();
//             Rc::new(TExpr::TVar(name))
//         }
//         Pattern::Int(_) => from_str("Int".to_string()),
//         Pattern::Float(_) => from_str("Float".to_string()),
//         Pattern::Str(_) => from_str("String".to_string()),
//         Pattern::Construct(cons, args) => {
//             let mut name = supply.next_name();
//             let mut supply_next = supply.deplete();
//             let mut vars = Vec::new();
//             for _ in args {
//                 vars.push(Rc::new(TExpr::TVar(name)));
//                 name = supply_next.next_name();
//                 supply_next = supply_next.deplete();
//             }
//             Rc::new(TExpr::TCons(cons, vars))
//         }
//     }
// }

fn pat_vars(pat: &Pattern) -> Vec<Rc<Expr>> {
    match pat {
        Pattern::Construct(_cons, vars) => {
            let mut args = Vec::new();
            for var in vars {
                args.push(Rc::new(Expr::Var(var.to_string(), RefCell::new(0))));
            }
            args
        }
        _ => Vec::new(),
    }
}

fn constructor_type(tenv: &TypeEnv, pat: &Pattern) -> Vec<Rc<TExpr>> {
    match pat {
        Pattern::Construct(cons, _) => {
            if let Some(v) = tenv.get(&cons.to_string()) {
                unwind_arrows(Rc::clone(&v.texpr), Vec::new())
            } else {
                panic!("Constructor signature not found")
            }
        }
        _ => Vec::new(),
    }
}

fn unwind_arrows(texpr: Rc<TExpr>, acc: Vec<Rc<TExpr>>) -> Vec<Rc<TExpr>> {
    match &*texpr {
        TExpr::TVar(_s) => {
            let mut acc = acc;
            acc.push(texpr);
            acc
        }
        TExpr::TCons(cons, args) => {
            if cons == "arrow" {
                let mut acc = acc;
                acc.push(Rc::clone(&args[0]));
                unwind_arrows(Rc::clone(&args[1]), acc)
            } else {
                let mut acc = acc;
                acc.push(texpr);
                acc
            }
        }
    }
}

/*
Lambda
*/

fn type_check_lam(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    head: Rc<Expr>,
    body: Rc<Expr>,
) -> CheckResult {
    let name = supply.next_name();
    let supply_next = supply.deplete();
    if let Expr::Var(s, _) = Rc::deref(&head) {
        let mut tenv_new = HashMap::new();
        tenv_new.clone_from(tenv);
        tenv_new.insert(
            s.to_string(),
            Scheme::new(Vec::new(), Rc::new(TVar(name.to_string()))),
        );
        type_check_lam1(name.to_string(), type_check(&tenv_new, supply_next, body))
    } else {
        panic!("Lambda needs a variable in the head position.")
    }
}

fn type_check_lam1(name: String, res: Option<(Rc<Substitute>, Rc<TExpr>)>) -> CheckResult {
    match res {
        None => None,
        Some((phi, t)) => Some((Rc::clone(&phi), arrow(phi.sub(name.to_string()), t))),
    }
}

/*
let
*/

fn type_check_let(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    vars: Vec<Rc<Expr>>,
    defs: Vec<Rc<Expr>>,
    body: Rc<Expr>,
) -> CheckResult {
    let (supply0, supply1) = supply.split();
    type_check_let1(
        tenv,
        supply0,
        vars,
        body,
        type_check_list(tenv, supply1, defs),
    )
}

fn type_check_let1(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    vars: Vec<Rc<Expr>>,
    body: Rc<Expr>,
    res: Option<(Rc<Substitute>, Vec<Rc<TExpr>>)>,
) -> CheckResult {
    match res {
        None => None,
        Some((phi, ts)) => {
            let (supply0, supply1) = supply.split();
            let type_env = sub_te(Rc::clone(&phi), tenv);
            let type_env2 = add_declarations(&type_env, supply0, vars, ts);
            type_check_let2(phi, type_check(&type_env2, supply1, body))
        }
    }
}

fn type_check_let2(phi: Rc<Substitute>, res: CheckResult) -> CheckResult {
    match res {
        None => None,
        Some((psi, t)) => Some((Rc::new(ComposeSub(psi, phi)), t)),
    }
}

fn add_declarations(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    vars: Vec<Rc<Expr>>,
    texprs: Vec<Rc<TExpr>>,
) -> TypeEnv {
    let unk = unknowns_te(tenv);
    let mut new_env = HashMap::new();
    let mut vars_back = vars;
    vars_back.reverse();
    new_env.clone_from(tenv);
    for t in texprs {
        let mut assoc = HashMap::new();
        let mut new_vars = Vec::new();
        let names = tvars_in(Rc::clone(&t));
        for name in names {
            if !unk.contains(&name) {
                new_vars.push(name);
            }
        }
        let mut supply_new = supply.clone();
        let mut tvars = Vec::new();
        for var in new_vars {
            let tname = supply_new.next_name();
            supply_new = supply_new.deplete();
            tvars.push(tname.to_string());
            assoc.insert(var, tname);
        }
        if let Some(v) = vars_back.pop() {
            new_env.insert(
                v.to_string(),
                Scheme::new(
                    tvars,
                    substitute_type(Rc::new(EnvSub(assoc)), Rc::clone(&t)),
                ),
            );
        }
    }
    new_env
}

/*
letrec
*/

fn type_check_letrec(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    vars: Vec<Rc<Expr>>,
    defs: Vec<Rc<Expr>>,
    body: Rc<Expr>,
) -> CheckResult {
    let (supply0, s) = supply.split();
    let (supply1, supply2) = s.split();
    let mut tenv_new = HashMap::new();
    let mut new_vars = HashMap::new();
    tenv_new.clone_from(tenv);

    let mut supply = supply2;
    for var in vars {
        if let Expr::Var(s, _) = Rc::deref(&var) {
            let tname = supply.next_name();
            supply = supply.deplete();
            new_vars.insert(
                s.to_string(),
                Scheme::new(Vec::new(), Rc::new(TVar(tname.to_string()))),
            );
            tenv_new.insert(
                s.to_string(),
                Scheme::new(Vec::new(), Rc::new(TVar(tname.to_string()))),
            );
        } else {
            panic!("Not a variable.");
        }
    }

    type_check_letrec1(
        tenv,
        supply0,
        &new_vars,
        body,
        type_check_list(&tenv_new, supply1, defs),
    )
}

fn type_check_letrec1(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    new_vars: &TypeEnv,
    body: Rc<Expr>,
    res: Option<(Rc<Substitute>, Vec<Rc<TExpr>>)>,
) -> CheckResult {
    match res {
        None => None,
        Some((phi, ts)) => {
            let tenv_new = sub_te(Rc::clone(&phi), tenv);
            let new_vars_new = sub_te(Rc::clone(&phi), new_vars); //what a name
            let mut ts_new = Vec::new();
            for val in new_vars.values() {
                ts_new.push(Rc::clone(&val.texpr));
            }
            type_check_letrec2(
                &tenv_new,
                supply,
                &new_vars_new,
                body,
                unify_list(phi, ts, ts_new),
            )
        }
    }
}

fn type_check_letrec2(
    tenv: &TypeEnv,
    supply: impl NameSupply + Clone,
    vars: &TypeEnv,
    body: Rc<Expr>,
    res: Option<Rc<Substitute>>,
) -> CheckResult {
    match res {
        None => None,
        Some(phi) => {
            let (supply0, supply1) = supply.split();
            let vars_new = sub_te(Rc::clone(&phi), vars);
            let tenv_new = sub_te(Rc::clone(&phi), tenv);
            let mut ts = Vec::new();
            for val in vars_new.values() {
                ts.push(Rc::clone(&val.texpr));
            }
            let keys = vars
                .keys()
                .map(|s| Rc::new(Expr::Var(s.to_string(), RefCell::new(0))))
                .collect();
            let tenv_new2 = add_declarations(&tenv_new, supply0, keys, ts);

            type_check_let2(phi, type_check(&tenv_new2, supply1, body))
        }
    }
}
