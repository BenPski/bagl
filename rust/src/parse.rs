use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;

use crate::expr::Expr;
use crate::expr::Expr::*;
use crate::environment::Env;
use crate::builtin::*;


#[derive(Debug, Clone)]
pub enum Token {
    // keywords
    Let,
    In,
    Case,
    Data,
    Undefined,

    //symbols
    Dash,
    RAngle,
    Slash,
    Dot,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Colon,
    Equal,
    Bar,
    Arrow,

    // special
    Newline,
    Str(String),
    Integer(i64),
    Double(f64),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
            '[' => tokens.push(Token::LBrace),
            ']' => tokens.push(Token::RBrace),
            '=' => tokens.push(Token::Equal),
            '|' => tokens.push(Token::Bar),
            ':' => tokens.push(Token::Colon),
            ';' => tokens.push(Token::Semicolon),
            '-' => {
                if src[index+1] == '>' {
                    index+=1;
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Dash);
                }
            },
            '>' => tokens.push(Token::RAngle),
            '+' => tokens.push(Token::Str("+".to_string())),
            '/' => {
                if src[index+1] == '/' {
                    let mut c = c;
                    while !(c == '\n') {
                        index += 1;
                        c = src[index];
                    }
                }
            },
            '\n' => tokens.push(Token::Newline),
            // strings and keywords (for now only ascii characters
            _ => {
                if c.is_lowercase() {
                    let mut word = "".to_string();
                    word.push(c);
                    if !(index+1 == src.len()){
                        index += 1;
                        let mut c = src[index];
                        while c.is_alphanumeric() && index < src.len() {
                            word.push(c);
                            index += 1;
                            if index == src.len() {
                                c = '\0';
                            } else {
                                c = src[index];
                            }
                        }
                        index -= 1;
                    }

                    match word.as_str() {
                        "let" => tokens.push(Token::Let),
                        "in" => tokens.push(Token::In),
                        "case" => tokens.push(Token::Case),
                        "data" => tokens.push(Token::Data),
                        "undefined" => tokens.push(Token::Undefined),
                        _ => tokens.push(Token::Str(word)),
                    }
                } else if c.is_uppercase() {
                    let mut word = "".to_string();
                    word.push(c);
                    if !(index+1 == src.len()){
                        index += 1;
                        let mut c = src[index];
                        while c.is_alphanumeric() && index < src.len() {
                            word.push(c);
                            index += 1;
                            if index == src.len() {
                                c = '\0';
                            } else {
                                c = src[index];
                            }
                        }
                        index -= 1;
                    }
                    tokens.push(Token::Str(word));
                } else if c.is_numeric() {
                    let mut num = "".to_string();
                    num.push(c);
                    let mut c;
                    if !(index+1 == src.len()) {
                        index += 1;
                        c = src[index];
                        while c.is_numeric() && index < src.len() {
                            num.push(c);
                            index += 1;
                            if index == src.len() {
                                c = '\0';
                            } else {
                                c = src[index];
                            }
                        }

                        if c == '.' && !(index+1 == src.len()) {
                            num.push(c);
                            index += 1;
                            c = src[index];
                            while c.is_numeric() && index < src.len() {
                                num.push(c);
                                index += 1;
                                if index == src.len() {
                                    c = '\0';
                                } else {
                                    c = src[index];
                                }
                            }
                            index -= 1;
                            tokens.push(Token::Double(num.parse().unwrap()));
                        } else {
                            index -= 1;
                            tokens.push(Token::Integer(num.parse().unwrap()));
                        }

                    } else {
                        tokens.push(Token::Integer(num.parse().unwrap()));
                    }
                }
            },
        }
        index += 1;
    }
    tokens
}

#[derive(Debug, Clone)]
pub enum Sugar {
    Definition(Rc<Expr>, Rc<Expr>), // var, expr
    DataDefinition(Rc<Expr>, Vec<Rc<Expr>>), // type, constructors
    FunctionDefinition(Rc<Expr>, Rc<Expr>), // left side (apply), right side
    CaseDefinition(Rc<Expr>, Vec<Rc<Expr>>, Vec<Rc<Expr>>), // expression, patterns, branches
    MultiLambda(Rc<Expr>, Rc<Expr>), // head(apply form), body
}

#[derive(Debug, Clone)]
pub enum LangComponent {
    Tok(Token),
    Sug(Sugar),
    Exp(Rc<Expr>),
}

use crate::LangComponent::*;
use crate::Sugar::*;


pub fn parse(mut tokens: Vec<Token>) -> Rc<Expr> {
    let init_env = Rc::new(Env::new()); // the initial environment that everything should point at
    tokens.push(Token::Newline); // just to make sure it ends with a newline
    tokens.reverse(); // just so it is more stack like
    let mut stack = Vec::new();
    while !parsed_toplevel(&tokens, &stack) {
        let mut shift = true;
        if match_lambda(&stack) {
            parse_lambda(&mut stack);
            shift = false;
        } else if match_data_definition(&stack) {
            parse_data_definition(&mut stack);
            shift = false;
        } else if match_case_definition(&stack) {
            parse_case_definition(&mut stack);
            shift = false;
        } else if match_function_definition_sugar(&stack) {
            parse_function_definition_sugar(&mut stack);
            shift = false;
        } else if match_function_definition(&stack) {
            parse_function_definition(&mut stack);
            shift = false;
        } else if match_multilambda_sugar(&stack) {
            parse_multilambda_sugar(&mut stack);
            shift = false;
        } else if match_multilambda(&stack) {
            parse_multilambda(&mut stack);
            shift = false;
        } else if match_data(&stack) {
            parse_data(&mut stack);
            shift = false;
        } else if match_variable(&stack) {
            parse_variable(&mut stack, &init_env);
            shift = false;
        } else if match_integer(&stack) {
            parse_integer(&mut stack);
            shift = false;
        } else if match_double(&stack) {
            parse_double(&mut stack);
            shift = false;
        } else if match_single_let(&stack) {
            parse_single_let(&mut stack, &init_env);
            shift = false;
        } else if match_multi_let(&stack) {
            parse_multi_let(&mut stack, &init_env);
            shift = false;
        } else if match_case(&stack) {
            parse_case(&mut stack, &init_env);
            shift = false;
        } else if match_group(&stack) {
            parse_group(&mut stack);
            shift = false;
        } else if match_definition(&stack) {
            parse_definition(&mut stack);
            shift = false;
        } else if match_apply(&stack) {
            parse_apply(&mut stack);
            shift = false;
        } else if match_undefined(&stack) {
            parse_undefined(&mut stack);
            shift = false;
        }


        if shift {
            if let Some(v) = tokens.pop() {
                stack.push(Tok(v));
            } else {
                println!("{:#?}", stack);
                panic!("Tried to pop from empty token stack.")
            }
        }
    }
    toplevel(&mut stack,&init_env)
}

/*
helpers for parsing
*/

fn parsed_toplevel(tokens: &Vec<Token>, stack: &Vec<LangComponent>) -> bool {
    if tokens.len() == 0 {
//        let mut passing = true;
        for item in stack {
            match item {
                Sug(Definition(_,_)) => continue,
                Tok(Token::Newline) => continue,
                _ => return false,
            }
        }
//        passing
        return true
    } else {
        false
    }
}

fn toplevel(stack: &mut Vec<LangComponent>, env: &Rc<Env>) -> Rc<Expr> {
    let mut vars = Vec::new();
    let mut vals = Vec::new();
    let mut found_main = false;
    let main = Rc::new(Variable("main".to_string(), RefCell::new(Rc::clone(env))));
    while stack.len() > 0 {
        let item = stack.pop();
        match item {
            Some(Sug(Definition(var, val))) => {
                vars.push(Rc::clone(&var));
                vals.push(val);
                match Rc::deref(&var) {
                    Variable(s, _) => {
                        if s == "main" {
                            found_main = true;
                        }
                    }
                    _ => continue,
                }
            },
            Some(Tok(Token::Newline)) => continue,
            _ => panic!("Encountered something other than a definition and a blank line."),
        }
    }
    if found_main {
        Rc::new(LetRec(vars, vals, main, RefCell::new(Rc::clone(env))))
    } else {
        panic!("Need main defined for the script to run.")
    }
}


/*
parsing components
*/

fn match_undefined(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 1 {
        if let Tok(Token::Undefined) = &stack[stack.len()-1] {
            true
        } else {false}
    } else {false}
}

fn parse_undefined(stack: &mut Vec<LangComponent>) {
    if let Some(Tok(Token::Undefined)) = stack.pop() {
        stack.push(Exp(Rc::new(Bottom)));
    }
}

fn match_definition(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 4 {
        let l = stack.len();
        if let Tok(Token::Newline) = &stack[l - 1] {
            if let Exp(_) = &stack[l - 2] {
                if let Tok(Token::Equal) = &stack[l - 3] {
                    if let Exp(var) = &stack[l - 4] {
                        if let Variable(_,_) = Rc::deref(var) {
                            true
                        } else {false}
                    } else { false }
                } else { false }
            } else { false }
        } else { false }
    } else {false}
}

fn parse_definition(stack: &mut Vec<LangComponent>) {
    if let Some(Tok(Token::Newline)) = stack.pop() {
        if let Some(Exp(expr)) = stack.pop() {
            if let Some(Tok(Token::Equal)) = stack.pop() {
                if let Some(Exp(var)) = stack.pop() {
                    stack.push(Sug(Definition(var, expr)));
                }
            }
        }
    } else {
        panic!("Definition matched, but not parsed.")
    }
}

fn match_function_definition(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 4 {
        let l = stack.len();
        if let Tok(Token::Newline) = &stack[l - 1] {
            if let Exp(_) = &stack[l - 2] {
                if let Tok(Token::Equal) = &stack[l - 3] {
                    if let Exp(var) = &stack[l - 4] {
                        if let Apply(_,_) = Rc::deref(var) {
                            true
                        } else {false}
                    } else { false }
                } else { false }
            } else { false }
        } else { false }
    } else {false}
}

fn parse_function_definition(stack: &mut Vec<LangComponent>) {
    if let Some(Tok(Token::Newline)) = stack.pop() {
        if let Some(Exp(right)) = stack.pop() {
            if let Some(Tok(Token::Equal)) = stack.pop() {
                if let Some(Exp(left)) = stack.pop() {
                    stack.push(Sug(FunctionDefinition(left, right)));
                }
            }
        }
    } else {
        panic!("Definition matched, but not parsed.")
    }
}

fn match_multilambda(stack: &Vec<LangComponent>) -> bool {
    // Slash Variable Dot Expression
    if stack.len() >= 4 {
        let l = stack.len();
        if let Tok(Token::Slash) = &stack[l-4] {
            if let Exp(vars) = &stack[l-3] {
                if let Apply(_,_) = Rc::deref(vars) {
                    if let Tok(Token::Dot) = &stack[l-2] {
                        if let Exp(_) = &stack[l-1] {
                            true
                        } else {false}
                    } else {false}
                } else {false}
            } else {false}
        } else {false}
    } else {false}
}

fn parse_multilambda(stack: & mut Vec<LangComponent>) {
    // should have already matched
    if let Some(Exp(expr)) = stack.pop() {
        if let Some(Tok(_)) = stack.pop() {
            if let Some(Exp(vars)) = stack.pop() {
                if let Some(Tok(_)) = stack.pop() {
                    stack.push(Sug(MultiLambda(vars, expr)));
                }
            }
        }
    } else {
        panic!("Lambda matched, but couldn't parse.")
    }
}

fn match_multilambda_sugar(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 1 {
        let l = stack.len();
        if let Sug(MultiLambda(_,_)) = &stack[l-1] {
            true
        } else {false}
    } else {false}
}

fn parse_multilambda_sugar(stack: &mut Vec<LangComponent>) {
    if let Some(Sug(MultiLambda(head, body))) = stack.pop() {
        stack.push(Exp(fix_lambda(head, body)));
    }
}

fn match_function_definition_sugar(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 1 {
        let l = stack.len();
        if let Sug(FunctionDefinition(_,_)) = &stack[l-1] {
            true
        } else {false}
    } else {false}
}

fn parse_function_definition_sugar(stack: &mut Vec<LangComponent>) {
    if let Some(Sug(FunctionDefinition(left, right))) = stack.pop() {
        stack.push(Sug(fix_function_definition(left, right)));
    }
}

fn match_data(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 1 {
        if let Sug(DataDefinition(_,_)) = &stack[stack.len()-1] {
            true
        } else {false}
    } else {false}
}

fn parse_data(stack: &mut Vec<LangComponent>) {
    // make definitions out of the constructors
    if let Some(Sug(DataDefinition(name, constructors))) = stack.pop() {
        // for now not worrying about typelevel things, so just get the type name, constructor name, and number of arguments
        let typ = data_name(&name);
        for constructor in constructors {
            let cons = data_name(&constructor);
            let var = data_var(&constructor);
            let args = data_args(&constructor);
            stack.push(Sug(Definition(var, Rc::new(Data(args, typ.clone(), cons, false, Vec::new())))));
        }
    }
}

fn match_case(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 1 {
        if let Sug(CaseDefinition(_,_,_)) = &stack[stack.len()-1] {
            true
        } else {false}
    } else {false}
}

fn parse_case(stack: &mut Vec<LangComponent>, env: &Rc<Env>) {
    // want to force the patterns into an initialized form as expected by the interpreter
    if let Some(Sug(CaseDefinition(expr, pats, branches))) = stack.pop() {
        let mut pat_updated = Vec::new();
        for pat in pats {
            pat_updated.push(fix_pat(pat));
        }
        stack.push(Exp(Rc::new(Case(expr, pat_updated, branches, RefCell::new(Rc::clone(env))))));
    }
}

fn fix_pat(pat: Rc<Expr>) -> Rc<Expr> {
    // will be given a variable, assuming everything is alright can construct data
    // unfortunately type data is lost at this point
    let mut vars = Vec::new();
    let mut expr = pat;
    loop {
        match Rc::deref(&expr) {
            Variable(s,_) => {
                vars.reverse();
                return Rc::new(Data(vars.len(), "".to_string(), s.to_string(), true, vars));
            },
            Apply(exp, var) => {
                if let Variable(_,_) = Rc::deref(var) {
                    vars.push(Rc::clone(&var));
                    expr = Rc::clone(exp);
                } else {panic!("Patterns need to only have variables.")}
                continue
            }
            _ => {
                println!("{:?}", expr);
                panic!("Malformed pattern.")
            },
        }
    }
}

fn match_lambda(stack: &Vec<LangComponent>) -> bool {
    // Slash Variable Dot Expression
    if stack.len() >= 4 {
        let l = stack.len();
        if let Tok(Token::Slash) = &stack[l-4] {
            if let Exp(var) = &stack[l-3] {
                if let Variable(_,_) = Rc::deref(var) {
                    if let Tok(Token::Dot) = &stack[l-2] {
                        if let Exp(_) = &stack[l-1] {
                            true
                        } else {false}
                    } else {false}
                } else {false}
            } else {false}
        } else {false}
    } else {false}
}

fn parse_lambda(stack: & mut Vec<LangComponent>) {
    // should have already matched
    if let Some(Exp(expr)) = stack.pop() {
        if let Some(Tok(_)) = stack.pop() {
            if let Some(Exp(var)) = stack.pop() {
                if let Some(Tok(_)) = stack.pop() {
                    stack.push(Exp(Rc::new(Lambda(var, expr))));
                }
            }
        }
    } else {
        panic!("Lambda matched, but couldn't parse.")
    }
}

fn fix_lambda(head: Rc<Expr>, body: Rc<Expr>) -> Rc<Expr> {
    // unwrap application in the head to make the lambda
    match Rc::deref(&head) {
        Variable(_,_) => Rc::new(Lambda(head, body)),
        Apply(rest, var) => fix_lambda(Rc::clone(rest), Rc::new(Lambda(Rc::clone(var), body))),
        _ => {panic!("Malformed multi-argument lambda.")}
    }
}

fn fix_function_definition(left: Rc<Expr>, right: Rc<Expr>) -> Sugar {
    // turns funtion defintion into regular definition
    match Rc::deref(&left) {
        Variable(_, _) => Definition(left, right),
        Apply(rest, var) => fix_function_definition(Rc::clone(&rest), Rc::new(Lambda(Rc::clone(&var), right))),
        _ => {panic!("Malformed function definition.")}
    }
}

fn data_name(expr: &Rc<Expr>) -> String {
    match Rc::deref(expr) {
        Variable(s,_) => s.to_string(),
        Apply(rest, _) => data_name(rest),
        _ => panic!("Malformed name for type of data.")
    }
}

fn data_var(expr: &Rc<Expr>) -> Rc<Expr> {
    match Rc::deref(expr) {
        Variable(_,_) => Rc::clone(expr),
        Apply(rest, _) => data_var(rest),
        _ => panic!("Malformed constructor definition.")
    }
}

fn data_args(expr: &Rc<Expr>) -> usize {
    match Rc::deref(expr) {
        Variable(_,_) => 0,
        Apply(rest, _) => 1 + data_args(rest),
        _ => panic!("Malformed constructor definition.")
    }
}


fn match_variable(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 1 {
        let l = stack.len();
        if let Tok(Token::Str(_)) = &stack[l-1] {
            true
        } else {false}
    } else {false}
}

fn parse_variable(stack: &mut Vec<LangComponent>, env: &Rc<Env>) {
    if let Some(Tok(Token::Str(s))) = stack.pop() {
        match &s[..] {
            "+" => stack.push(Exp(Rc::new(Builtin(2, "+".to_string(), add_func)))),
            _ => stack.push(Exp(Rc::new(Variable(s, RefCell::new(Rc::clone(env)))))),
        }
    } else {
        panic!("Variable matched, but it couldn't parse.")
    }
}

fn match_case_definition(stack: &Vec<LangComponent>) -> bool {
    //minimal: case expr LBrace newline expr arrow expr newline RBrace
    // general: case expr LBrace newline (expr arrow expr newline)* RBrace
    if stack.len() >= 9 {
        let l = stack.len();
        if let Tok(Token::RBrace) = &stack[l-1] {
            if let Tok(Token::Newline) = &stack[l-2] {
                if let Exp(_) = &stack[l-3] {
                    if let Tok(Token::Arrow) = &stack[l-4] {
                        if let Exp(_) = &stack[l-5] {
                            let mut passing = true;
                            let mut i = 0;
                            while passing && (l >= (9 + 4*i)) {
                                if let Tok(Token::Newline) = &stack[l-6-4*i] {
                                    if let Exp(_) = &stack[l-7-4*i] {
                                        if let Tok(Token::Arrow) = &stack[l-8-4*i] {
                                            if let Exp(_) = &stack[l-9-4*i] {
                                                passing = true;
                                                i += 1;
                                            } else {passing = false;}
                                        } else {passing = false;}
                                    } else {passing = false;}
                                } else {passing = false;}
                            }
                            if let Tok(Token::Newline) = &stack[l-6-4*i] {
                                if let Tok(Token::LBrace) = &stack[l-7-4*i] {
                                    if let Exp(_) = &stack[l-8-4*i] {
                                        if let Tok(Token::Case) = &stack[l-9-4*i] {
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

fn parse_case_definition(stack: &mut Vec<LangComponent>) {
    let mut pats = Vec::new();
    let mut branches = Vec::new();
    if let Some(Tok(Token::RBrace)) = stack.pop() {
        if let Some(Tok(Token::Newline)) = stack.pop() {
            if let Some(Exp(branch)) = stack.pop() {
                if let Some(Tok(Token::Arrow)) = stack.pop() {
                    if let Some(Exp(pat)) = stack.pop() {
                        pats.push(pat);
                        branches.push(branch);
                        while stack.len() >= 3 {
                            if let Tok(Token::Newline) = &stack[stack.len()-1] {
                                if let Exp(_) = &stack[stack.len()-2] {
                                    if let Tok(Token::Arrow) = &stack[stack.len()-3] {
                                        if let Exp(_) = &stack[stack.len()-4] {
                                            stack.pop();
                                            if let Some(Exp(branch)) = stack.pop() {
                                                stack.pop();
                                                if let Some(Exp(pat)) = stack.pop() {
                                                    pats.push(pat);
                                                    branches.push(branch);
                                                }
                                            }
                                        } else {break}
                                    } else {break}
                                } else {break}
                            } else {break}
                        }
                        if let Some(Tok(Token::Newline)) = stack.pop() {
                            if let Some(Tok(Token::LBrace)) = stack.pop() {
                                if let Some(Exp(expr)) = stack.pop() {
                                    if let Some(Tok(Token::Case)) = stack.pop() {
                                        stack.push(Sug(CaseDefinition(expr, pats, branches)));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn match_data_definition(stack: &Vec<LangComponent>) -> bool {
    // data type = d1 | d2 ...
    // minimum is data exp = d1 newline
    if stack.len() >= 4 {
        let l = stack.len();
        if let Tok(Token::Newline) = &stack[l-1] {
            if let Exp(_) = &stack[l-2] {
                let mut passing = true;
                let mut i = 0;
                while passing && (l >= (6 + 2*i)) {
                    if let Tok(Token::Bar) = &stack[l-3-2*i] {
                        if let Exp(_) = &stack[l-4-2*i] {
                            passing = true;
                            i += 1;
                        } else {passing = false;}
                    } else {passing = false;}
                }
                if let Tok(Token::Colon) = &stack[l-3-2*i] {
                    if let Exp(_) = &stack[l-4-2*i] {
                        true
                    } else {false}
                } else {false}
            } else {false}
        } else {false}
    } else {false}
}

fn parse_data_definition(stack: &mut Vec<LangComponent>) {
    let mut constructors = Vec::new();
    if let Some(Tok(Token::Newline)) = stack.pop() {
        if let Some(Exp(con)) = stack.pop() {
            constructors.push(con);
            while stack.len() >= 2 {
                if let Tok(Token::Bar) = &stack[stack.len()-1] {
                    if let Exp(_) = &stack[stack.len()-2] {
                        stack.pop();
                        if let Some(Exp(con)) = stack.pop() {
                            constructors.push(con);
                        } else {panic!("Something weird happened in data constructor.")}
                    } else {break}
                } else {break}
            }
            if let Some(Tok(Token::Colon)) = stack.pop() {
                if let Some(Exp(data)) = stack.pop() {
                    stack.push(Sug(DataDefinition(data, constructors)));
                }
            }
        }
    }
}

fn match_integer(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 1 {
        let l = stack.len();
        if let Tok(Token::Integer(_)) = &stack[l-1] {
            true
        } else {false}
    } else {false}
}

fn parse_integer(stack: &mut Vec<LangComponent>) {
    if let Some(Tok(Token::Integer(i))) = stack.pop() {
        stack.push(Exp(Rc::new(Integer(i))));
    } else {
        panic!("Integer matched, but it couldn't parse.")
    }
}

fn match_double(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 1 {
        let l = stack.len();
        if let Tok(Token::Double(_)) = &stack[l-1] {
            true
        } else {false}
    } else {false}
}

fn parse_double(stack: &mut Vec<LangComponent>) {
    if let Some(Tok(Token::Double(i))) = stack.pop() {
        stack.push(Exp(Rc::new(Double(i))));
    } else {
        panic!("Double matched, but it couldn't parse.")
    }
}

fn match_group(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 3 {
        let l = stack.len();
        if let Tok(Token::LParen) = &stack[l-3] {
            if let Exp(_) = &stack[l-2] {
                if let Tok(Token::RParen) = &stack[l-1] {
                    true
                } else {false}
            } else {false}
        } else {false}
    } else {false}
}

fn parse_group(stack: &mut Vec<LangComponent>) {
    if let Some(Tok(_)) = stack.pop() {
        if let Some(Exp(expr)) = stack.pop() {
            if let Some(Tok(_)) = stack.pop() {
                stack.push(Exp(expr));
            }
        }
    } else {
        panic!("Matched group, but failed to parse.");
    }
}

fn match_apply(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 2 {
        let l = stack.len();
        if let Exp(_) = &stack[l-2] {
            if let Exp(_) = &stack[l-1] {
                true
            } else {false}
        } else {false}
    } else {false}
}

fn parse_apply(stack: &mut Vec<LangComponent>) {
    if let Some(Exp(right)) = stack.pop() {
        if let Some(Exp(left)) = stack.pop() {
            stack.push(Exp(Rc::new(Apply(left, right))));
        }
    } else {
        panic!("Matched apply, but couldn't parse.")
    }
}

fn match_single_let(stack: &Vec<LangComponent>) -> bool {
    if stack.len() >= 4 {
        let l = stack.len();
        if let Tok(Token::Let) = &stack[l-4] {
            if let Sug(_) = &stack[l-3] {
                if let Tok(Token::In) = &stack[l-2] {
                    if let Exp(_) = &stack[l-1] {
                        true
                    } else {false}
                } else {false}
            } else {false}
        } else {false}
    } else {false}
}

fn parse_single_let(stack: &mut Vec<LangComponent>, env: &Rc<Env>) {
    if let Some(Exp(body)) = stack.pop() {
        if let Some(Tok(Token::In)) = stack.pop() {
            if let Some(Sug(Definition(var, val))) = stack.pop() {
                if let Some(Tok(Token::Let)) = stack.pop() {
                    stack.push(Exp(Rc::new(LetRec(vec!(var), vec!(val), body, RefCell::new(Rc::clone(env))))));
                }
            }
        }
    } else {
        panic!("Matched let, but couldn't parse.")
    }
}

fn match_multi_let(stack: &Vec<LangComponent>) -> bool {
    // a let, but it has to be at least:
    /*
    let
        var = val
    in expr
    */
    // so needs newlines to show up
    if stack.len() >= 6 {
        let l = stack.len();
        if let Exp(_) = &stack[l-1] {
            if let Tok(Token::In) = &stack[l-2] {
                //at least one definition
                if let Tok(Token::Newline) = &stack[l-3] {
                    if let Sug(Definition(_,_)) = &stack[l-4] {
                        let mut passing = true;
                        let mut i = 1;
                        while passing && (l >= 6 + 2*i) {
                            if let Tok(Token::Newline) = &stack[l-3-2*i] {
                                if let Sug(Definition(_,_)) = &stack[l-4-2*i] {
                                    passing = true;
                                } else {passing = false;}
                            } else {passing = false;}
                            i += 1;
                        }
                        if let Tok(Token::Newline) = &stack[l-3-2*i] {
                            if let Tok(Token::Let) = &stack[l-4-2*i] {
                                true
                            } else {false}
                        } else {false}
                    } else {false}
                } else {false}
            } else {false}
        } else {false}
    } else {false}
}

fn parse_multi_let(stack: &mut Vec<LangComponent>, env: &Rc<Env>) {
    // know that it should be [Let, newline, *(var, equal, val, newline), in , body]
    let mut vars = Vec::new();
    let mut vals = Vec::new();
    if let Some(Exp(body)) = stack.pop() {
        if let Some(Tok(Token::In)) = stack.pop() {
            let mut passing = true;
            while passing && (stack.len() >= 4) {
                let l = stack.len();
                if let Tok(Token::Newline) = &stack[l-1] {
                    if let Sug(Definition(_, _)) = &stack[l-2] {
                        // fits pattern
                        if let Some(Tok(Token::Newline)) = stack.pop() {
                            if let Some(Sug(Definition(var, val))) = stack.pop() {
                                vars.push(var);
                                vals.push(val);
                            }
                        } else {panic!("Some parse error in multi-line let")};
                    } else {passing = false;}
                } else {passing = false;}
            }
            if let Some(Tok(Token::Newline)) = stack.pop() {
                if let Some(Tok(Token::Let)) = stack.pop() {
                    stack.push(Exp(Rc::new(LetRec(vars, vals, body, RefCell::new(Rc::clone(env))))));
                }
            }
        }
    }
}

