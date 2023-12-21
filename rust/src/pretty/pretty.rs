/*
 * A pretty printer interface
 */

use std::{collections::VecDeque, fmt::Display};

#[derive(Clone)]
pub enum Pretty {
    Nil,
    Str(String),
    Append(Box<Pretty>, Box<Pretty>),
    Indent(Box<Pretty>),
    Newline,
}

pub struct PrinterStep {
    indent: usize,
    step: Pretty,
}

impl Display for PrinterStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.step {
            Pretty::Str(s) => write!(f, "{}", s),
            Pretty::Newline => {
                let spaces = " ".repeat(self.indent);
                write!(f, "\n{spaces}",)
            },
            _ => write!(f, "")
        }
    }
}

pub struct Printer {
    column: usize,
    steps: VecDeque<PrinterStep>,
}

impl Iterator for Printer {
    type Item = PrinterStep;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.steps.pop_front();
        match next {
            None => None,
            Some(v) => match v.step {
                Pretty::Nil => None,
                Pretty::Str(ref s) => {
                    self.column += s.len();
                    Some(v)
                }
                Pretty::Append(left, right) => {
                    let right_new = PrinterStep { indent: v.indent, step: *right };
                    let left_new = PrinterStep { indent: v.indent, step: *left };
                    self.steps.push_front(right_new);
                    Some(left_new)
                }
                Pretty::Indent(step) => {
                    let step_new = PrinterStep { indent: self.column, step: *step };
                    Some(step_new)
                }
               _ => Some(v),
            }
        }
    }
}

impl IntoIterator for Pretty {
    type Item = PrinterStep;
    type IntoIter = Printer;

    fn into_iter(self) -> Self::IntoIter {
        let first = PrinterStep { indent: 0, step: self };
        Printer { column: 0, steps: VecDeque::from(vec![first]) }
    }
}

impl Pretty {
    pub fn nil() -> Self {
        Self::Nil
    }

    pub fn append(left: Self, right: Self) -> Self {
        Self::Append(Box::new(left), Box::new(right))
    }

    pub fn concat<I>(vals: I) -> Self where I: IntoIterator<Item = Self> {
        vals.into_iter().fold(Self::Nil, |acc, x| Self::append(acc, x))
    }

    pub fn interleave<I>(sep: Self, vals: I) -> Self where I: IntoIterator<Item = Self> {
        let mut iter = vals.into_iter();
        let start = iter.next();
        match start {
            Some(v) => Self::inter(v, sep, iter),
            None => Self::Nil,
        }
    }

    fn inter<I>(start: Self, sep: Self, vals: I) -> Self where I: Iterator<Item = Self> {
        vals.fold(start, |acc, x| Self::append(acc, Self::append(sep.clone(), x)))
    }

    pub fn str(s: &str) -> Self {
        Self::Str(String::from(s))
    }

    pub fn indent(seq: Self) -> Self {
        seq
    }

    pub fn newline() -> Self {
        Self::str("\n")
    }

    pub fn display(&self) -> String {
        Self::evaluate(0, VecDeque::from(vec![(self, 0)]))
        //self.into_iter().map(|step| step.to_string()).join("")
    }

    fn evaluate(col: usize, steps: VecDeque<(&Pretty, usize)>) -> String {
        match steps.front() {
            None => "".to_string(),
            Some((val, indent)) => {
                Self::action(col, val, *indent, steps)
            }
        }
    }

    fn action<'a>(col: usize, val: &'a Pretty, indent: usize, mut steps: VecDeque<(&'a Pretty, usize)>) -> String {
        match val {
            Self::Nil => {
                steps.pop_front();
                Self::evaluate(col, steps)
            },
            Self::Append(left, right) => {
                steps.pop_front();
                steps.push_front((right, indent));
                steps.push_front((left, indent));
                Self::evaluate(col, steps)
            },
            Self::Str(s) => {
                steps.pop_front();
                let res = Self::evaluate(col+s.len(), steps);
                format!("{s}{res}")
            },
            Self::Newline => {
                steps.pop_front();
                let res = Self::evaluate(indent, steps);
                let spaces = " ".repeat(indent);
                format!("\n{spaces}{res}")
            },
            Self::Indent(step) => {
                steps.pop_front();
                steps.push_front((step, col));
                Self::evaluate(col, steps)
            }
            _ => "".to_string(),
        }
    }
}

impl Display for Pretty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
