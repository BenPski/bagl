use std::fmt::Display;
use crate::Alternative;
use crate::core::{Expr, Supercombinator, Program};

use super::pretty::Pretty;

impl Expr<String> {
    fn pretty(&self) -> Pretty {
        match self {
            Self::EVar(v) => Pretty::str(v),
            Self::ENum(n) => Pretty::str(format!("{n}").as_str()),
            Self::EAp(left, right) => {
                Pretty::append(
                    Self::pretty(left),
                    Pretty::append(Pretty::str(" "), Self::pretty_app(right))
                )
            },
            Self::ELet(is_rec, defs, expr) => {
                let kword = if *is_rec { "let" } else { "letrec" };
                Pretty::concat([
                               Pretty::str(kword), Pretty::newline(),
                               Pretty::str(" "), Pretty::indent(Self::pretty_defs(defs)), Pretty::newline(),
                               Pretty::str("in "), Self::pretty(expr)
                ])
            },
            Self::ELam(args, expr) => {
                let pretty_args = args.into_iter().fold(Pretty::Nil, |acc, x| Pretty::append(acc, Pretty::append(Pretty::str(" "), Pretty::str(x))));
                Pretty::concat([
                               Pretty::str("\\"), 
                               pretty_args,
                               Pretty::str("."),
                               Self::pretty(expr),
                ])
            },
            Self::ECase(expr, branches) => {
                Pretty::concat([
                               Pretty::str("case "), Self::pretty(expr), Pretty::newline(),
                               Pretty::str(" "), Pretty::indent(Self::pretty_branches(branches))
                ])
            },
            Self::EConstructor(tag, arity) => {
                Pretty::str(format!("Pack[{tag},{arity}]").as_str())
            },
        }
    }

    fn pretty_app(&self) -> Pretty {
        if self.is_atomic() {
            Self::pretty(&self)
        } else {
            Pretty::concat([Pretty::str("("), Self::pretty(&self), Pretty::str(")")])
        }
    }

    fn pretty_defs<'a, I>(defs: I) -> Pretty where I: IntoIterator<Item = &'a (String, Self)> {
        Pretty::interleave(Pretty::newline(), defs.into_iter().map(|x| {Self::pretty_def(x)}))
    }

    fn pretty_def(def: &(String, Self)) -> Pretty {
        Pretty::concat([Pretty::str(def.0.as_str()), Pretty::str(" = "), Pretty::indent(Self::pretty(&def.1))])
    }

    fn pretty_branches<'a, I>(branches: I) -> Pretty where I: IntoIterator<Item = &'a Alternative<String>> {
        Pretty::interleave(Pretty::newline(), branches.into_iter().map(|x| Self::pretty_branch(x)))
    }

    fn pretty_branch(branch: &Alternative<String>) -> Pretty {
        let tag = branch.tag;
        Pretty::concat([
                       Pretty::str("("), Pretty::str(format!("Pack[{tag}]").as_str()),
                       Pretty::interleave(Pretty::str(" "), branch.vars.clone().into_iter().map(|x| Pretty::str(&x))),
                       Pretty::str(")"),
                       Pretty::str(" -> "), Self::pretty(&branch.expr)
        ])
    }
}

impl From<Expr<String>> for Pretty {
    fn from(value: Expr<String>) -> Self {
        value.pretty()
    }
}

impl From<&Expr<String>> for Pretty {
    fn from(value: &Expr<String>) -> Self {
        value.pretty()
    }
}

impl From<Supercombinator<String>> for Pretty {
    fn from(value: Supercombinator<String>) -> Pretty {
        Pretty::concat([
                       Pretty::str(&value.name), Pretty::str(" "), 
                       Pretty::interleave(Pretty::str(" "), value.args.clone().into_iter().map(|x| Pretty::str(&x))),
                       Pretty::str(" = "), value.body.pretty()
        ])
    }
}

impl From<&Supercombinator<String>> for Pretty {
    fn from(value: &Supercombinator<String>) -> Pretty {
        Pretty::concat([
                       Pretty::str(&value.name), Pretty::str(" "), 
                       Pretty::interleave(Pretty::str(" "), value.args.clone().into_iter().map(|x| Pretty::str(&x))),
                       Pretty::str(" = "), value.body.pretty()
        ])
    }
}

impl From<Program<String>> for Pretty {
    fn from(value: Program<String>) -> Self {
        Pretty::interleave(Pretty::Newline, value.definitions.clone().into_iter().map(|x| Pretty::from(x)))
    }
}

impl From<&Program<String>> for Pretty {
    fn from(value: &Program<String>) -> Self {
        Pretty::interleave(Pretty::Newline, value.definitions.clone().into_iter().map(|x| Pretty::from(x)))
    }
}

impl Display for Expr<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty().display())
    }
}

impl Display for Supercombinator<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Pretty::from(self))
    }
}

impl Display for Program<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Pretty::from(self))
    }
}
