/// The lexer is responsible for converting user input to a
/// well defined context-free grammar.

#[derive(PartialEq)]
#[derive(Debug)]
enum Num {
    Integer(i64),
    Decimal(i64, u64, i16)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    Number(Num),
    OParen,
    CParen,
    OBracket,
    CBracket,
    Word(String),
    Oper(String)
}

fn to_digit(c: char) -> Option<u8> {
    let d = c as u8;
    return match d {
        48...58 => Some(d - 48),
         _ => None
    }
}

fn is_num(c : char) -> bool {
    return match to_digit(c) {
        Some(_) => true,
        None => false
    }
}

fn is_alpha(c: char) -> bool {
    let d = c as u8;
    return match d {
        65...90 | 97...122 => true,
        _ => false
    }
}

fn is_grouping(c: char) -> bool {
    return match c {
        ')' | '(' | '[' | ']' => true,
        _ => false
    }
}

fn lex_num<'a>(chars: &'a mut Vec<char>) -> Option<(Num, &'a mut Vec<char>)> {
    let mut whole_num : i64 = 0;
    let mut decimal : u64 = 0;
    let mut exponent : i16 = 0;
    let mut is_dec = false;
    let mut parsed_zeroes = false;

    match chars.last() {
        None => return None,
        Some(&c) => {
            if is_num(c) == false && c != '.' {
                return None
            }
        }
    }

    while let Some(c) = chars.pop() {
        match (c, to_digit(c), is_dec, parsed_zeroes) {
            (_, Some(d), false, false) => {
                whole_num *= 10;
                whole_num += d as i64;
            },
            (_, Some(0), true, false) => exponent -= 1,
            (_, Some(d), true, false) => {
                parsed_zeroes = true;
                decimal *= 10;
                decimal += d as u64;
            },
            (_, Some(d), true, true) => {
                decimal *= 10;
                decimal += d as u64;
            },
            ('.', None, false, false) => is_dec = true,
            _ => {
                chars.push(c);
                break;
            }
        }
    }

    // TODO: Maybe I don't need an option type? Need to consider how this can
    // fail
    if is_dec {
        return Some((Num::Decimal(whole_num, decimal, exponent), chars));
    } else {
        return Some((Num::Integer(whole_num), chars));
    }
}

fn lex_word<'a>(chars: &'a mut Vec<char>) -> Option<(String, &'a mut Vec<char>)> {
    let mut s = String::new();
    
    while let Some(c) = chars.pop() {
        match is_alpha(c) {
            true => s.push(c),
            false => {
                chars.push(c);
                break;
            }
        }
    }

    return match s.is_empty() {
        true => None,
        false => Some((s, chars))
    }
}

fn lex_oper<'a>(chars: &'a mut Vec<char>) -> Option<(String, &'a mut Vec<char>)> {
    let mut s = String::new();
    
    while let Some(c) = chars.pop() {
        match is_alpha(c) || is_num(c) || c == ' ' || is_grouping(c) {
            false => s.push(c),
            true => {
                chars.push(c);
                break;
            }
        }
    }

    return match s.is_empty() {
        true => None,
        false => Some((s, chars))
    }
}

fn _lex(chars: &mut Vec<char>) -> Vec<Token> {
    match chars.pop() {
        None => return vec![],
        Some(c) => {
            match c {
                '(' => {
                    let mut rest = _lex(chars);
                    rest.insert(0, Token::OParen);
                    return rest;
                },
                ')' => {
                    let mut rest = _lex(chars);
                    rest.insert(0, Token::CParen);
                    return rest;
                },
                '[' => {
                    let mut rest = _lex(chars);
                    rest.insert(0, Token::OBracket);
                    return rest;
                },
                ']' => {
                    let mut rest = _lex(chars);
                    rest.insert(0, Token::CBracket);
                    return rest;
                },
                ' ' => return _lex(chars),
                _ => {
                    chars.push(c);

                    if let Some((n, _chars)) = lex_num(chars) {
                        let mut rest = _lex(_chars);
                        rest.insert(0, Token::Number(n));
                        return rest;
                    }
                    
                    if let Some((s, _chars)) = lex_word(chars) {
                        let mut rest = _lex(_chars);
                        rest.insert(0, Token::Word(s));
                        return rest;
                    }
                    
                    if let Some((o, _chars)) = lex_oper(chars) {
                        let mut rest = _lex(_chars);
                        rest.insert(0, Token::Oper(o));
                        return rest;
                    }
                    
                    panic!("Lex error!");
                }
            }
        }
    }
}

/// Converts a string into a list of tokens
pub fn lex(text: String) -> Vec<Token> {
    let mut chars: Vec<char> = text.chars().collect();

    // Rust pops from the end and pushes to the front. Reverse chars so
    // the left most char is on top
    chars.reverse();
    return _lex(&mut chars);
}

#[cfg(test)]
mod tests {
    use super::lex;
    use super::Token;
    use super::Num;

    // String::from_str gives a lot of warnings, this is just a workaround
    fn quiet_from_str(s: &str) -> String {
        let mut string = String::new();
        string.push_str(s);
        return string;
    }

    #[test]
    fn grouping() {
        let res = lex(quiet_from_str("[[()]()]"));
        let expected = vec![
            Token::OBracket,
            Token::OBracket,
            Token::OParen,
            Token::CParen,
            Token::CBracket,
            Token::OParen,
            Token::CParen,
            Token::CBracket
        ];

        assert_eq!(res, expected);
    }
    
    #[test]
    fn numbers() {
        let res = lex(quiet_from_str("1337"));
        assert_eq!(vec![Token::Number(Num::Integer(1337))], res);

        let res = lex(quiet_from_str("98"));
        assert_eq!(vec![Token::Number(Num::Integer(98))], res);

        let res = lex(quiet_from_str("3.1415"));
        assert_eq!(vec![Token::Number(Num::Decimal(3, 1415, 0))], res);

        let res = lex(quiet_from_str(".001"));
        assert_eq!(vec![Token::Number(Num::Decimal(0, 1, -2))], res);
    }

    #[test]
    fn words() {
        let tan = quiet_from_str("tan");
        let sin = quiet_from_str("sin");
        let cos = quiet_from_str("cos");

        let res = lex(quiet_from_str("tan"));
        assert_eq!(vec![Token::Word(tan)], res);

        let res = lex(quiet_from_str("sin     cos   "));
        assert_eq!(vec![Token::Word(sin), Token::Word(cos)], res);
    }

    #[test]
    fn ops() {
        let res = lex(quiet_from_str("+"));
        assert_eq!(vec![Token::Oper(quiet_from_str("+"))], res);

        let res = lex(quiet_from_str(" >=   ++"));
        assert_eq!(vec![Token::Oper(quiet_from_str(">=")), Token::Oper(quiet_from_str("++"))], res);
    }

    #[test]
    fn expr() {
        let res = lex(quiet_from_str("2* 3.1415 >= 5"));
        
        let sol = vec![Token::Number(Num::Integer(2)),
                       Token::Oper(quiet_from_str("*")),
                       Token::Number(Num::Decimal(3, 1415, 0)),
                       Token::Oper(quiet_from_str(">=")),
                       Token::Number(Num::Integer(5))];
        assert_eq!(res, sol);

        let res = lex(quiet_from_str("2/(pi - x^2) = 2.018"));
        let sol = vec![Token::Number(Num::Integer(2)),
                       Token::Oper(quiet_from_str("/")),
                       Token::OParen,
                       Token::Word(quiet_from_str("pi")),
                       Token::Oper(quiet_from_str("-")),
                       Token::Word(quiet_from_str("x")),
                       Token::Oper(quiet_from_str("^")),
                       Token::Number(Num::Integer(2)),
                       Token::CParen,
                       Token::Oper(quiet_from_str("=")),
                       Token::Number(Num::Decimal(2, 18, -1))];
        assert_eq!(res, sol);
    }
}
