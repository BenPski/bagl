/*
 * convert the input string to tokens
 */

use itertools::Itertools;

type Token = String;

pub fn lexer(input: String) -> Vec<Token> {
    let mut iter = input.chars();
    let mut res = Vec::new();
    loop {
        let next = iter.next();
        match next {
            None => break,
            Some(v) => {
                if v.is_alphabetic() {
                    let mut s = String::from(v);
                    s.extend(iter.peeking_take_while(|x| is_id_char(x)));
                    res.push(s);
                    continue;
                } else if v.is_digit(10) {
                    let mut s = String::from(v);
                    s.extend(iter.peeking_take_while(|x| x.is_digit(10)));
                    res.push(s);
                    continue;
                } else if v.is_whitespace() {
                    continue;
                } else {
                    // could be a symbol
                    // clumsy lookahead
                    let mut iter_copy = iter.clone();
                    if let Some(v2) = iter_copy.next() {
                        let s = String::from(v);
                        let mut s2 = String::from(v);
                        s2.push(v2);
                        if [String::from(">="), String::from("=="), String::from("<=")].contains(&s2) {
                            res.push(s2);
                            iter = iter_copy;
                        } else {
                            res.push(s);
                        }
                    }
                    res.push(String::from(v));
                    continue;
                }
            }
        }
    }
    res
}

fn is_id_char(char: &char) -> bool {
    char.is_alphabetic() || char.is_digit(10) || char == &'_'
}
