use crate::lang::Number;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

#[derive(Parser)]
#[grammar = "lang/grammar.pest"]
pub struct Grammar;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left) | Operator::new(divide, Left),
            Operator::new(power, Right),
        ])
    };
}

fn evaluate_expr(expression: Pairs<Rule>) -> Number {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::int => Number::INT(pair.as_str().parse::<i32>().unwrap()),
            Rule::float => Number::FLOAT(pair.as_str().parse::<f32>().unwrap()),
            Rule::expr => evaluate_expr(pair.into_inner()),
            _ => unreachable!(),
        },
        |lhs: Number, op: Pair<Rule>, rhs: Number| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.pow(rhs),
            _ => unreachable!(),
        },
    )
}

fn evaluate_input(pairs: Pairs<Rule>) -> Vec<Token> {
    pairs
        .map(|pair| match pair.as_rule() {
            Rule::input => evaluate_input(pair.into_inner()),
            Rule::expr => vec![Token::Expr(evaluate_expr(pair.into_inner()))],
            Rule::text => vec![Token::Text(pair.as_span().as_str().to_string())],
            _ => unreachable!(),
        })
        .flatten()
        .collect()
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Expr(Number),
    Text(String),
}

pub fn evaluate(input: &str) -> Result<Vec<Token>, pest::error::Error<Rule>> {
    Ok(Grammar::parse(Rule::calculation, input)?
        .map(|pair| match pair.as_rule() {
            Rule::input => Some(evaluate_input(pair.into_inner())),
            Rule::EOI => None,
            _ => unreachable!(),
        })
        .filter_map(|p| p)
        .flatten()
        .collect::<Vec<Token>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int_token(int: i32) -> [Token; 1] {
        [Token::Expr(Number::INT(int))]
    }

    fn float_token(float: f32) -> [Token; 1] {
        [Token::Expr(Number::FLOAT(float))]
    }

    #[test]
    fn evaluate_compute_scientific_notation_correctly() {
        assert_eq!(evaluate("0.2e1 + 0.3e-1").unwrap(), float_token(2.03));
    }

    #[test]
    fn evaluate_compute_add_correctly() {
        assert_eq!(evaluate("2 + 2.0").unwrap(), float_token(4.0));
        assert_eq!(evaluate("3 + 5").unwrap(), int_token(8));
        assert_eq!(evaluate("1.9 + 5.89").unwrap(), float_token(7.79));
        assert_eq!(evaluate("2.0 plus 12.5").unwrap(), float_token(14.5));
        assert_eq!(evaluate("7.86 with 90").unwrap(), float_token(97.86));
    }

    #[test]
    fn evaluate_compute_sub_correctly() {
        assert_eq!(evaluate("2 - 5").unwrap(), int_token(-3));
        assert_eq!(evaluate("2 - 2.0").unwrap(), float_token(0.0));
        assert_eq!(evaluate("4.5 - 3.5").unwrap(), float_token(1.0));
        assert_eq!(evaluate("7.8 minus 8.2").unwrap(), float_token(-0.39999962));
        assert_eq!(evaluate("2.0 subtract 12.5").unwrap(), float_token(-10.5));
    }

    #[test]
    fn evaluate_compute_mul_correctly() {
        assert_eq!(evaluate("2 * 5").unwrap(), int_token(10));
        assert_eq!(evaluate("2 * 12.5").unwrap(), float_token(25.0));
        assert_eq!(evaluate("4.5 * 4.2").unwrap(), float_token(18.9));
        assert_eq!(evaluate("3.5 mul 9").unwrap(), float_token(31.5));
        assert_eq!(evaluate("2.3 times 3").unwrap(), float_token(6.8999996));
        assert_eq!(evaluate("2 multiplied by 1").unwrap(), int_token(2));
    }

    #[test]
    fn evaluate_compute_div_correctly() {
        assert_eq!(evaluate("6 / 3").unwrap(), float_token(2.0));
        assert_eq!(evaluate("12 / 4.0").unwrap(), float_token(3.0));
        assert_eq!(evaluate("3.0 / 1.5").unwrap(), float_token(2.0));
        assert_eq!(evaluate("9.0 div 3.0").unwrap(), float_token(3.0));
        assert_eq!(evaluate("3.5 divide 9").unwrap(), float_token(0.388888889));
        assert_eq!(evaluate("2.0 divide by 0.5").unwrap(), float_token(4.0));
    }

    #[test]
    fn evaluate_compute_power_correctly() {
        assert_eq!(evaluate("6 ^ 2").unwrap(), float_token(36.0));
        assert_eq!(evaluate("2 ** 3").unwrap(), float_token(8.0));
    }

    #[test]
    fn evaluate_compute_expressions_following_priorities_correctly() {
        assert_eq!(evaluate("2.0 + 2 * 3").unwrap(), float_token(8.0));
        assert_eq!(evaluate("(2.0 + 2) * 3").unwrap(), float_token(12.0));
        assert_eq!(evaluate("(2.0 + 2) * 3 / 4").unwrap(), float_token(3.0));
        assert_eq!(evaluate("(2 + 2) * 4 / 2 ** 3").unwrap(), float_token(2.0));
    }

    #[test]
    fn evaluate_compute_every_expression_of_the_input_with_texts() {
        assert_eq!(
            evaluate("2 * 2 is equal to 2 ^ 2").unwrap(),
            [
                Token::Expr(Number::INT(4)),
                Token::Text("is".to_string()),
                Token::Text("equal".to_string()),
                Token::Text("to".to_string()),
                Token::Expr(Number::FLOAT(4.0))
            ]
        );
        assert_eq!(
            evaluate("3 + 1 is 4 and 4 + 2 is 6, ok?").unwrap(),
            [
                Token::Expr(Number::INT(4)),
                Token::Text("is".to_string()),
                Token::Expr(Number::INT(4)),
                Token::Text("and".to_string()),
                Token::Expr(Number::INT(6)),
                Token::Text("is".to_string()),
                Token::Expr(Number::INT(6)),
                Token::Text(",".to_string()),
                Token::Text("ok?".to_string())
            ]
        );
    }
}
