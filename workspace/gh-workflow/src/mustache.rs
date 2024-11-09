use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::space0;
use nom::combinator::map;
use nom::error::ParseError;
use nom::sequence::preceded;
use nom::{IResult, Parser};

#[derive(Debug, PartialEq)]
enum Operator {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    left: String,
    operator: Operator,
    right: String,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum ComplexCondition {
    Single(Condition),
    Compound(
        Box<ComplexCondition>,
        LogicalOperator,
        Box<ComplexCondition>,
    ),
}

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn parse_operator(input: &str) -> IResult<&str, Operator> {
    preceded(
        sp,
        alt((
            map(tag("=="), |_| Operator::Eq),
            map(tag("!="), |_| Operator::Ne),
            map(tag("<="), |_| Operator::Le),
            map(tag(">="), |_| Operator::Ge),
            map(tag("<"), |_| Operator::Lt),
            map(tag(">"), |_| Operator::Gt),
        )),
    )
    .parse(input)
}

fn parse_operand(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '.' || c == '\'' || c == '/')(
        input,
    )
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, (left, _, operator, _, right)) =
        (parse_operand, space0, parse_operator, space0, parse_operand).parse(input)?;
    Ok((
        input,
        Condition { left: left.to_string(), operator, right: right.to_string() },
    ))
}

fn parse_logical_operator(input: &str) -> IResult<&str, LogicalOperator> {
    preceded(
        sp,
        alt((
            map(tag("&&"), |_| LogicalOperator::And),
            map(tag("||"), |_| LogicalOperator::Or),
        )),
    )
    .parse(input)
}

pub fn parse_if_condition(input: &str) -> IResult<&str, ComplexCondition> {
    let (input, left) = parse_condition(input)?;
    let mut result = ComplexCondition::Single(left);

    let mut input = input;
    while let Ok((remaining_input, operator)) =
        preceded(space0, parse_logical_operator).parse(input)
    {
        let (new_input, right) = preceded(space0, parse_if_condition).parse(remaining_input)?;
        result = ComplexCondition::Compound(Box::new(result), operator, Box::new(right));
        input = new_input;
    }

    Ok((input, result))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_condition() {
        let input = "github.event_name == 'push'";
        let expected = Condition {
            left: "github.event_name".to_string(),
            operator: Operator::Eq,
            right: "'push'".to_string(),
        };
        assert_eq!(parse_condition(input).unwrap().1, expected);
    }

    #[test]
    fn parse_complex_condition_with_and_or() {
        let input = "github.event_name == 'push' && github.ref == 'refs/heads/main' ";
        let expected = ComplexCondition::Compound(
            Box::from(ComplexCondition::Single(Condition {
                left: "github.event_name".to_string(),
                operator: Operator::Eq,
                right: "'push'".to_string(),
            })),
            LogicalOperator::And,
            Box::new(ComplexCondition::Single(Condition {
                left: "github.ref".to_string(),
                operator: Operator::Eq,
                right: "'refs/heads/main'".to_string(),
            })),
        );

        assert_eq!(parse_if_condition(input).unwrap().1, expected);
    }
}
