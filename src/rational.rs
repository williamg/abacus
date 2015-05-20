use lexer::Token;
use lexer::Num;

pub struct Rational {
    num: i64,
    den: i64,
}

impl Rational {
    fn fromNum (number: Num) -> Rational {
        return Rational { num: 0, den: 0 }
    }
}

