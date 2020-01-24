use std::rc::Rc;
use std::ops::Deref;
use std::cell::{RefCell, Ref};
use crate::expr::Expr;
use crate::expr::Expr::*;
use crate::environment::Env;
use crate::either::Either;
use crate::either::Either::*;


#[derive(Debug)]
pub enum Token {
    // keywords
    Let,
    In,
    Case,
    Of,

    //symbols
    Dash,
    RAngle,
    Slash,
    Dot,
    LParen,
    RParen,
    Semicolon,
    Equal,

    // special
    Newline,
    Str(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            _ => write!(f, "Tok")
        }
    }
}


struct Lexer {
    source: String,
    cursor: usize,
    tokens: Vec<Token>,
}

pub fn lex(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let src: Vec<char> = source.chars().collect();
    let mut index = 0;
    while index < src.len() {
        let c = src[index];
        match c {
            // key symbols
            '\\' => tokens.push(Token::Slash),
            '.' => tokens.push(Token::Dot),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '=' => tokens.push(Token::Equal),
            ';' => tokens.push(Token::Semicolon),
            '-' => tokens.push(Token::Dash),
            '>' => tokens.push(Token::RAngle),
            '\n' => tokens.push(Token::Newline),
            // strings and keywords (for now only ascii characters
            _ => {
                if c.is_lowercase() {
                    let mut word = "".to_string();
                    word.push(c);
                    if !(index +1 == src.len()){
                        index += 1;
                        let mut c = src[index];
                        while c.is_alphabetic() && index < src.len() {
                            word.push(c);
                            index += 1;
                            c = src[index];
                        }
                        index -= 1;
                    }

                    match word.as_str() {
                        "let" => tokens.push(Token::Let),
                        "in" => tokens.push(Token::In),
                        "case" => tokens.push(Token::Case),
                        "of" => tokens.push(Token::Of),
                        _ => tokens.push(Token::Str(word)),
                    }
                }
            },
        }
        index += 1;
    }
    tokens
}

pub fn parse(mut tokens: Vec<Token>) -> Rc<Expr> {
    let init_env = Rc::new(Env::new()); // the initial environment that everything should point at
    tokens.reverse(); // just so it is more stack like
    let mut stack: Vec<Either<Token,Rc<Expr>>> = Vec::new();
    while tokens.len() > 0 || stack.len() > 1 {
//        println!("{:?}", stack);
        let mut shift = true;
        if match_lambda(&stack) {
            parse_lambda(&mut stack);
            shift = false;
        } else if match_variable(&stack) {
            parse_variable(&mut stack, &init_env);
            shift = false;
        } else if match_single_let(&stack) {
            parse_single_let(&mut stack, &init_env);
            shift = false;
        } else if match_multi_let(&stack) {
            parse_multi_let(&mut stack, &init_env);
            shift = false;
//        } else if match_case(&stack) {
//            parse_case(&mut stack);
//            shift = false;
        } else if match_apply(&stack) {
            parse_apply(&mut stack);
            shift = false;
        } else if match_group(&stack) {
            parse_group(&mut stack);
            shift = false;
        }


        if shift {
            if let Some(v) = tokens.pop() {
                stack.push(Left(v));
            } else {
                panic!("Tried to pop from empty token stack.")
            }
        }
    }
    if let Some(v) = stack.pop() {
        match v {
            Right(expr) => expr,
            Left(_) => panic!("Only simplified to a token."),
        }
    } else {
        panic!("Parse error")
    }

}


fn match_lambda(stack: &Vec<Either<Token,Rc<Expr>>>) -> bool {
    // Slash Variable Dot Expression
    if stack.len() >= 4 {
        let l = stack.len();
        if let Left(Token::Slash) = &stack[l-4] {
            if let Right(var) = &stack[l-3] {
                if let Left(Token::Dot) = &stack[l-2] {
                    if let Right(_) = &stack[l-1] {
                        if let Variable(_, _) = Rc::deref(var) {
                            true
                        } else {false}
                    } else {false}
                } else {false}
            } else {false}
        } else {false}
    } else {
        false
    }
}

fn parse_lambda(stack: & mut Vec<Either<Token, Rc<Expr>>>) {
    // should have already matched
    if let Some(Right(expr)) = stack.pop() {
        if let Some(Left(_)) = stack.pop() {
            if let Some(Right(var)) = stack.pop() {
                if let Some(Left(_)) = stack.pop() {
                    stack.push(Right(Rc::new(Lambda(var, expr))));
                }
            }
        }
    } else {
        panic!("Lambda matched, but couldn't parse.")
    }
}

fn match_variable(stack: &Vec<Either<Token, Rc<Expr>>>) -> bool {
    if stack.len() >= 1 {
        let l = stack.len();
        if let Left(Token::Str(_)) = &stack[l-1] {
            true
        } else {false}
    } else {false}
}

fn parse_variable(stack: &mut Vec<Either<Token, Rc<Expr>>>, env: &Rc<Env>) {
    if let Some(Left(Token::Str(s))) = stack.pop() {
        stack.push(Right(Rc::new(Variable(s, RefCell::new(Rc::clone(env))))));
    } else {
        panic!("Variable matched, but it couldn't parse.")
    }
}

fn match_group(stack: &Vec<Either<Token, Rc<Expr>>>) -> bool {
    if stack.len() >= 3 {
        let l = stack.len();
        if let Left(Token::LParen) = &stack[l-3] {
            if let Right(_) = &stack[l-2] {
                if let Left(Token::RParen) = &stack[l-1] {
                    true
                } else {false}
            } else {false}
        } else {false}
    } else {false}
}

fn parse_group(stack: &mut Vec<Either<Token, Rc<Expr>>>) {
    if let Some(Left(_)) = stack.pop() {
        if let Some(Right(expr)) = stack.pop() {
            if let Some(Left(_)) = stack.pop() {
                stack.push(Right(expr));
            }
        }
    } else {
        panic!("Matched group, but failed to parse.");
    }
}

fn match_apply(stack: &Vec<Either<Token, Rc<Expr>>>) -> bool {
    if stack.len() >= 2 {
        let l = stack.len();
        if let Right(_) = &stack[l-2] {
            if let Right(_) = &stack[l-1] {
                true
            } else {false}
        } else {false}
    } else {false}
}

fn parse_apply(stack: &mut Vec<Either<Token, Rc<Expr>>>) {
    if let Some(Right(right)) = stack.pop() {
        if let Some(Right(left)) = stack.pop() {
            stack.push(Right(Rc::new(Apply(left, right))));
        }
    } else {
        panic!("Matched apply, but couldn't parse.")
    }
}

fn match_single_let(stack: &Vec<Either<Token, Rc<Expr>>>) -> bool {
    if stack.len() >= 6 {
        let l = stack.len();
        if let Left(Token::Let) = &stack[l-6] {
            if let Right(var) = &stack[l-5] {
                if let Left(Token::Equal) = &stack[l-4] {
                    if let Right(_) = &stack[l-3] {
                        if let Left(Token::In) = &stack[l-2] {
                            if let Right(_) = &stack[l-1] {
                                if let Variable(_,_) = Rc::deref(&var) {
                                    true
                                } else {false}
                            } else {false}
                        } else {false}
                    } else {false}
                } else {false}
            } else {false}
        } else {false}
    } else {false}
}

fn parse_single_let(stack: &mut Vec<Either<Token, Rc<Expr>>>, env: &Rc<Env>) {
    if let Some(Right(body)) = stack.pop() {
        if let Some(Left(Token::In)) = stack.pop() {
            if let Some(Right(val)) = stack.pop() {
                if let Some(Left(Token::Equal)) = stack.pop() {
                    if let Some(Right(var)) = stack.pop() {
                        if let Some(Left(Token::Let)) = stack.pop() {
                            stack.push(Right(Rc::new(LetRec(vec!(var), vec!(val), body, RefCell::new(Rc::clone(env))))));
                        }
                    }
                }
            }
        }
    } else {
        panic!("Matched let, but couldn't parse.")
    }
}

fn match_multi_let(stack: &Vec<Either<Token, Rc<Expr>>>) -> bool {
    // a let, but it has to be at least:
    /*
    let
        var = val
    in expr
    */
    // so needs newlines to show up
    if stack.len() >= 8 {
        let l = stack.len();
        if let Right(_) = &stack[l-1] {
            if let Left(Token::In) = &stack[l-2] {
                //at least one defintion
                if let Left(Token::Newline) = &stack[l-3] {
                    if let Right(_) = &stack[l-4] {
                        if let Left(Token::Equal) = &stack[l-5] {
                            if let Right(var) = &stack[l-6] {
                                if let Variable(_, _) = Rc::deref(var) {
                                    let mut passing = true;
                                    let mut i = 0;
                                    while passing && (l >= 6 + 4*(i+1)) {
                                        if let Left(Token::Newline) = &stack[l-7 - 4*i] {
                                            if let Right(_) = &stack[l-8-4*i] {
                                                if let Left(Token::Equal) = &stack[l-9-4*i] {
                                                    if let Right(var) = &stack[l-10-4*i] {
                                                        if let Variable(_, _) = Rc::deref(var) {
                                                            passing = true;
                                                        } else {passing = false;}
                                                    } else {passing = false;}
                                                } else {passing = false;}
                                            } else {passing = false;}
                                        } else {passing = false;}
                                        i += 1;
                                    }
                                    // look for opening let
                                    if let Left(Token::Newline) = &stack[l-7-4*i] {
                                        if let Left(Token::Let) = &stack[l-8-4*i] {
                                            true
                                        } else {false}
                                    } else {false}
                                } else {false}
                            } else {false}
                        } else {false}
                    } else {false}
                } else {false}
            } else {false}
        } else {false}
    } else {false}
}

fn parse_multi_let(stack: &mut Vec<Either<Token, Rc<Expr>>>, env: &Rc<Env>) {
    // know that it should be [Let, newline, *(var, equal, val, newline), in , body]
    let mut vars = Vec::new();
    let mut vals = Vec::new();
    if let Some(Right(body)) = stack.pop() {
        if let Some(Left(Token::In)) = stack.pop() {
            let mut passing = true;
            while passing && (stack.len() >= 6) {
                let l = stack.len();
                if let Left(Token::Newline) = &stack[l-1] {
                    if let Right(_) = &stack[l-2] {
                        if let Left(Token::Equal) = &stack[l-3] {
                            if let Right(_) = &stack[l-4] {
                                // fits pattern, so do poping
                                if let Some(Left(Token::Newline)) = stack.pop() {
                                    if let Some(Right(val)) = stack.pop() {
                                        if let Some(Left(Token::Equal)) = stack.pop() {
                                            if let Some(Right(var)) = stack.pop() {
                                                vars.push(var);
                                                vals.push(val);
                                            }
                                        }
                                    }
                                } else {panic!("some parse error.")}
                            } else {passing = false;}
                        } else {passing = false;}
                    } else {passing = false;}
                } else {passing = false;}
            }
            if let Some(Left(Token::Newline)) = stack.pop() {
                if let Some(Left(Token::Let)) = stack.pop() {
                    stack.push(Right(Rc::new(LetRec(vars, vals, body, RefCell::new(Rc::clone(env))))));
                }
            }
        }
    }
}

