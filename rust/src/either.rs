use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
pub enum Either<A, B> {
    Left(A),
    Right(B)
}

impl<A: Display, B: Display> std::fmt::Display for Either<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Left(a) => write!(f, "Left({})", a),
            Right(a) => write!(f, "Right({})", a),
        }
    }
}

use crate::Either::*;

fn is_right<A, B>(val: &Either<A, B>) -> bool {
    match val {
        Right(_) => true,
        _ => false,
    }
}

fn is_left<A, B>(val: &Either<A, B>) -> bool {
    !is_right(val)
}