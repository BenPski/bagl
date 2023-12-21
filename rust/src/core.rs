/*
 * The core language that gets compiled to
 * a more decorated lambda calculus
 */

#[derive(Debug, Clone)]
pub enum Expr<A> {
    EVar(String),
    ENum(u64),
    EConstructor(u64, u64),
    EAp(Box<Expr<A>>,Box<Expr<A>>),
    ELet(bool, Vec<(A, Expr<A>)>, Box<Expr<A>>),
    ECase(Box<Expr<A>>, Vec<Alternative<A>>),
    ELam(Vec<A>, Box<Expr<A>>),
}

#[derive(Debug, Clone)]
pub struct Alternative<A> {
    pub tag: u64,
    pub vars: Vec<A>,
    pub expr: Expr<A>,
}

#[derive(Debug, Clone)]
pub struct Supercombinator<A> {
    pub name: String,
    pub args: Vec<A>,
    pub body: Expr<A>,
}

#[derive(Debug, Clone)]
pub struct Program<A> {
    pub definitions: Vec<Supercombinator<A>>,
}

fn binders_of<A, B>(arr: Vec<(A, B)>) -> Vec<A> {
    arr.into_iter().map(|x| x.0).collect()
}

fn rhs_of<A, B>(arr: Vec<(A, B)>) -> Vec<B> {
    arr.into_iter().map(|x| x.1).collect()
}

impl<A> Expr<A> {
    pub fn is_atomic(&self) -> bool {
        match self {
            Self::EVar(_) => true,
            Self::ENum(_) => true,
            _ => false,
        }
    }

    pub fn var(name: &str) -> Self {
        Self::EVar(String::from(name))
    }

    pub fn num(n: u64) -> Self {
        Self::ENum(n)
    }

    pub fn app(left: Self, right: Self) -> Self {
        Self::EAp(Box::new(left), Box::new(right))
    }
}

impl<A> Alternative<A> {
    pub fn new(tag: u64, vars: Vec<A>, expr: Expr<A>) -> Self {
        Alternative { tag, vars, expr }
    }
}

impl<A> Supercombinator<A> {
    pub fn new(name: String, args: Vec<A>, body: Expr<A>) -> Self {
        Supercombinator { name, args, body }
    }
}

impl<A> Program<A> {
    pub fn new(definitions: Vec<Supercombinator<A>>) -> Self {
        Program { definitions }
    }
}

