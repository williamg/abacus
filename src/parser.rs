use rational::Rational;
use lexer::Token;
use lexer::Num;

extern crate core;

trait UnaryOperator {
    fn apply(&self, expr: Expression) -> Expression;
}

trait BinaryOperator {
    fn apply(&self, expr1: Expression, expr2: Expression) -> Expression;
}

pub enum Expression {
    UnaryOp(UnaryOperator, Expression),
    BinaryOp(BinaryOperator, Expression, Expression),
    Value(Rational)
}

/*
fn parseNum(tokens: &mut Vec<Token>) -> Option<Expression> {
    return match tokens.pop() {
        Some(Token::Number(n)) => Some(Rational::fromNum(n)),
        Some(x) => {
            tokens.push(x);
            None;
        },
        None => None
    }
}

fn _parse(tokens: &mut Vec<Token>, exprStack: &mut Vec<Expression>) ->
    Vec<Expression> {

    if let Some(x) = parseNum(tokens) {
        exprStack.push(x);
        return _parse(tokens, exprStack);
    } else {
        return exprStack;
    }

}

pub fn parse(tokens: Vec<Token>) -> Expression {
    let exprStack = _parse(tokens, vec![]);

    if exprStack.len == 1 {
        return exprStack[0];
    } else {
        panic!("Parse error!");
    }
}*/
