use lexer::Num;

extern crate num;

#[derive(Debug)]
pub struct Rational {
    num: i64,
    den: i64,
}

impl Rational {
    pub fn from_num (number: Num) -> Rational {
        match number {
            Num::Integer (x) => Rational { num: x, den: 1 },
            Num::Decimal (whole, dec, exponent) => {
                if exponent >= 0 {
                    panic!("Positive exponent");
                }

                let denominator = num::pow (10, (-exponent) as usize);
                let numerator = (whole * denominator) + (dec as i64);

                Rational { num: numerator, den: denominator }
            }
        }
    }
}

