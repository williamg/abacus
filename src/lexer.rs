//! The lexer is responsible for converting user input to a
//! well defined context-free grammar.

#[derive(PartialEq)]
#[derive(Debug)]

 /// This defines the "types" of numbers that are recognized. Currently,
/// only integers and decimals are distinguished. In the future, this could
/// be extended to include different bases like hex or binary numbers.
///
/// **NOTE** It's not immediately clear that we even need to distinguish
/// integers from decimals since one is a subset of the other. Once I have
/// the system fully fleshed out, if the distinction still seems
/// superfluous this could be removed.
pub enum Num {
    /// An integer. Due to how the "negation" operation is handled, these will
    /// actually always be lexed as positive numbers.
    Integer(i64),

    /// A decimal number. Consists of an integral part, decimal part, and
    /// exponent. Like the integer, these will always be lexed as positive.
    Decimal(i64, u64, i16)
}

#[derive(PartialEq)]
#[derive(Debug)]

/// This defines the grammar that will be used by the parser to
/// represent and evaluate mathematical expressions.
pub enum Token {
    /// Exactly what it sounds like. See lexer::Num
    Number(Num),
    /// A single open parentheses
    OParen,
    /// A single close parentheses
    CParen,
    /// A single open bracket
    OBracket,
    /// A single open bracket
    CBracket,
    /// Any sequence of purely alpha characters. Could represent a variable or
    /// function name.
    Word(String),
    /// Any sequence of non-alpha/non-whitespace/non-numeric characters. Will
    /// represent basic operations like "+", ">=" etc.
    Oper(String)
}

/// Calculate the decimal value of a decimal character, if applicable
fn to_digit(c: char) -> Option<u8> {
    let d = c as u8;
    return match d {
        48...58 => Some(d - 48),
         _ => None
    }
}

/// Determine if the given character represents a decimal digit
fn is_num(c : char) -> bool {
    return match to_digit(c) {
        Some(_) => true,
        None => false
    }
}

/// Determine if the given character is an (upper or lower case) letter
fn is_alpha(c: char) -> bool {
    let d = c as u8;
    return match d {
        65...90 | 97...122 => true,
        _ => false
    }
}

/// Determine if the given character is a grouping symbol
fn is_grouping(c: char) -> bool {
    return match c {
        ')' | '(' | '[' | ']' => true,
        _ => false
    }
}

/// Given a vector of characters such that the left-most character is on top,
/// attempt to extract a number from the front of the character list.
/// If a valid number exists, parse it and return Some(n, cs) where n is the
/// Num value parsed and cs is the remaining unlexed characters. Otherwise,
/// return None.
fn lex_num(chars: &mut Vec<char>) -> Option<Num> {
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
                // We're done, but this character still needs to be lexed
                chars.push(c);
                break;
            }
        }
    }

    if is_dec {
        return Some(Num::Decimal(whole_num, decimal, exponent));
    } else {
        return Some(Num::Integer(whole_num));
    }
}
/// Given a vector of characters such that the left-most character is on top,
/// attempt to extract a purely-alpha string of characters ("word") from the
/// front of the character list. If a valid word exists, parse it and return
/// Some(w, cs) where w is the word parsed and cs is the remaining unlexed
/// characters. Otherwise, return None.
fn lex_word(chars: &mut Vec<char>) -> Option<String> {
    let mut s = String::new();

    while let Some(c) = chars.pop() {
        if is_alpha(c) {
            s.push(c);
        } else {
            chars.push(c);
            break;
        }
    }

    if s.is_empty() {
        return None;
    } else {
        return Some(s);
    }
}

/// Given a vector of characters such that the left-most character is on top,
/// attempt to extract a non-alpha/non-num/non-whitespace string of characters
/// ("operator") from the front of the character list. If a operator exists,
/// parse it and return Some(o, cs) where o is the operator parsed and cs is
/// the remaining unlexed characters. Otherwise, return None.
fn lex_oper(chars: &mut Vec<char>) -> Option<String> {
    let mut s = String::new();

    while let Some(c) = chars.pop() {
        let notop = is_alpha(c) || is_num(c) || c == ' ' || is_grouping(c);
        if notop {
            chars.push(c);
            break;
        } else {
            s.push(c);
        }
    }

    if s.is_empty() {
        return None;
    } else {
        return Some(s);
    }
}

/// Given a vector of characters such that the left-most character is on top,
/// attempt to extract a sequence of Tokens according to the grammar defined
/// above.
fn _lex(chars: &mut Vec<char>) -> Vec<Token> {
    let mut chars = chars;
    let mut v : Vec<Token> = vec![];

    while let Some(c) = chars.pop() {
        match c {
            '(' => v.push(Token::OParen),
            ')' => v.push(Token::CParen),
            '[' => v.push(Token::OBracket),
            ']' => v.push(Token::CBracket),
            ' ' => continue,
            _ => {
                chars.push(c);

                if let Some(n) = lex_num(&mut chars) {
                    v.push(Token::Number(n));
                    continue;
                }

                if let Some(s) = lex_word(&mut chars) {
                    v.push(Token::Word(s));
                    continue;
                }

                if let Some(o) = lex_oper(&mut chars) {
                    v.push(Token::Oper(o));
                    continue;
                }

                panic!("Lex error!");
            }
        }
    }

    return v;
}

/// Given a string representing a mathematical something or other, extract
/// a sequence of tokens that represent the string according to the grammar
/// defined above.
pub fn lex(text: String) -> Vec<Token> {
    let mut chars: Vec<char> = text.chars().collect();

    // Rust pops and pushes from the back meaning the left-most char is on the
    // "bottom". Reverse chars so the left most char is on top
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
