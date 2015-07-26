use rational::Rational;
use lexer::Token;
use lexer::Num;

use std::fmt;

pub trait UnaryOperator : fmt::Debug {
    fn apply(&self, expr: Box<Expression>) -> Box<Expression>;
}

pub trait BinaryOperator : fmt::Debug {
    fn apply(&self, expr1: Box<Expression>, expr2: Box<Expression>) -> Box<Expression>;
}

#[derive(Debug)]

pub enum Expression {
    UnaryOp(Box<UnaryOperator>, Box<Expression>),
    BinaryOp(Box<BinaryOperator>, Box<Expression>, Box<Expression>),
    Value(Rational)
}

struct OpInfo {
    left_assoc: bool,
    precedence: u8
}

fn get_op_info (op_token: &Token) -> Option<OpInfo> {
    if let &Token::Oper(ref s) = op_token {
        return match s.as_ref() {
            "+" | "-" => Some (OpInfo {left_assoc: true, precedence: 0}),
            "*" | "/" => Some (OpInfo {left_assoc: true, precedence: 1}),
            _ => None
        }
    }

    None
}

fn parse_num(tokens: &mut Vec<Token>) -> Option<Expression> {
    return match tokens.pop() {
        Some(Token::Number(n)) =>
            Some(Expression::Value(Rational::from_num(n))),
        Some(x) => {
            tokens.push(x);
            None
        },
        None => None
    }
}

fn _parse(tokens: &mut Vec<Token>, expr_stack: &mut Vec<Expression>) {

    if let Some(x) = parse_num(tokens) {
        expr_stack.push(x);
        _parse(tokens, expr_stack);
    }

}

// Use Shunting-Yard algorithm to convert infix expression into RPN (postfix)
// notation
fn to_postfix(tokens: &mut Vec<Token>, output: &mut Vec<Token>) {
    // Shunting yard reads tokens from LTR, but popping from a vec pops from
    // the back, so to pop the left-most token first, we need to reverse it.
    tokens.reverse();

    let mut oper_stack: Vec<Token> = vec![];
    while let Some(t) = tokens.pop() {
        match t {
            Token::Number(_) => output.insert(0, t),
            Token::Word(ref s) => oper_stack.push(Token::Word(s.clone())),
            Token::Comma => {
                loop {
                    match oper_stack.pop() {
                        Some(Token::OParen) => oper_stack.push(Token::OParen),
                        Some(Token::OBracket) => oper_stack.push(Token::OBracket),
                        Some(o) => output.insert(0, o),
                        None => panic!("Mismatched parens or brackets!")
                    }
                }
            },
            Token::Oper(ref s) => {
                let info1 = get_op_info(&t).expect("Unkown operator!");

                while let Some(op2) = oper_stack.pop() {
                    if let Token::Oper(_) = op2 {
                        if let Some(info2) = get_op_info(&op2) {
                            if info1.left_assoc && info1.precedence <= info2.precedence {
                                output.insert (0, op2);
                            } else if !info1.left_assoc && info1.precedence < info2.precedence {
                                output.insert(0, op2);
                            } else {
                                oper_stack.push(op2);
                                break;
                            }
                        } else {
                            panic!("Unknown operator!");
                        }
                    } else {
                        oper_stack.push(op2);
                        break;
                    }
                }

                oper_stack.push(Token::Oper(s.clone()));
            },
            Token::OParen | Token::OBracket => oper_stack.push(t),
            Token::CParen | Token::CBracket => {
                loop {
                    match oper_stack.pop() {
                        Some(Token::OParen) | Some(Token::OBracket) => {
                            match oper_stack.pop() {
                                Some(Token::Word(ref s)) => output.insert(0, Token::Word(s.clone())),
                                Some(s) => oper_stack.push(s),
                                None => {}
                            }

                            break;
                        },
                        Some(o) => output.insert(0, o),
                        None => panic!("Mismatched parens or brackets!")
                    }
                }
            },
        }
    }

    while let Some(t) = oper_stack.pop() {
        match t {
            Token::OParen | Token::CParen | Token::OBracket | Token::CBracket =>
                panic!("Mismatched parens or brackets!"),
            _ => output.insert(0, t)
        }
    }
}

pub fn parse(tokens: &mut Vec<Token>) -> Expression {
    let mut output_queue: Vec<Token> = vec![];

    to_postfix(tokens, &mut output_queue);

    // Temporary return value
    return Expression::Value(Rational::from_num(Num::Integer(0)));
}

#[cfg(test)]
mod tests {
    use super::to_postfix;
    use lexer::Token;
    use lexer::Num;

    // Perhaps I shouldn't use other modules in test cases, but it makes them
    // very pretty
    use lexer::lex;

    // String::from_str gives a lot of warnings, this is just a workaround
    fn quiet_from_str(s: &str) -> String {
        let mut string = String::new();
        string.push_str(s);
        return string;
    }

    fn test_postfix(infix: &str, postfix: &str) {
        let mut input = lex(quiet_from_str(infix));
        let mut output = vec![];
        let expected = lex(quiet_from_str(postfix));

        to_postfix(&mut input, &mut output);

        output.reverse ();

        assert_eq!(output, expected);
    }

    #[test]
    fn trivial() {
        test_postfix("23", "23");
        test_postfix("1 + 2", "1 2 +");
        test_postfix("1 * 2", "1 2 *");
    }

    #[test]
    fn orderOfOperations() {
        test_postfix("2 + 3 * 4", "2 3 4 * +");
        test_postfix("2 + 3 - 4", "2 3 + 4 -");
        test_postfix("2 + (3 * 4) / 6", "2 3 4 * 6 / +");
    }

    #[test]
    fn functions() {
        test_postfix("fun(2)", "2 fun");
        test_postfix("fun(1, 2)", "1 2 fun");
        test_postfix("1 + fun(2 + (3-4)) * 5", "1 2 3 4 - + fun 5 * +");
        test_postfix("1 / fun(2 * 3, 4 + 5)", "1 2 3 * 4 5 + fun /");
    }
}
