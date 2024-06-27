use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{char, multispace0},
    combinator::opt,
    sequence::pair,
    IResult,
};

// https://github.com/rust-bakery/nom
#[derive(Debug, PartialEq)]
pub struct AssertionFailedError<'a> {
    pub expected: DtoValueKind<'a>,
    pub real: DtoValueKind<'a>,
}

#[derive(Debug, PartialEq)]
pub enum DtoValueKind<'a> {
    String(&'a str),
    Dto(Dto<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Dto<'a> {
    pub name: &'a str,
    pub fields: Vec<DtoField<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct DtoField<'a> {
    pub name: &'a str,
    pub value: DtoFieldValue<'a>,
}

#[derive(Debug, PartialEq)]
pub enum DtoFieldValue<'a> {
    Null,
    String(&'a str),
}

/// Parses the last step the value
/// demo input: null
fn parse_field_value(input: &str) -> IResult<&str, DtoFieldValue> {
    let (input, value) = take_till(|c: char| c == ',' || c == ')')(input)?;
    // Assuming the next character is either ',' or ')', consume it without assigning.
    match value {
        "null" => Ok((input, DtoFieldValue::Null)),
        _ => Ok((input, DtoFieldValue::String(value))),
    }
}

/// demo input: firstName=null
fn parse_field(input: &str) -> IResult<&str, DtoField> {
    let (input, _) = opt(multispace0)(input)?;
    let (input, name) = take_until("=")(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = parse_field_value(input)?;
    Ok((
        input,
        DtoField {
            name: name.trim(),
            value,
        },
    )) // Trim the field name to remove any leading/trailing whitespace.
}

/// demo input: firstName=null, lastname=asd
fn parse_field_list(input: &str) -> IResult<&str, Vec<DtoField>> {
    let (input, fields) = separated_list0(pair(char(','), multispace0), parse_field)(input)?;
    Ok((input, fields))
}

/// demo input: User(firstName=null, lastname=asd)
fn parse_dto(input: &str) -> IResult<&str, DtoValueKind> {
    dbg!(input);
    let (input, name) = take_until("(")(input)?;
    let (input, fields) = delimited(tag("("), parse_field_list, tag(")"))(input)?;
    let (input, _) = opt(multispace0)(input)?;
    Ok((
        input,
        DtoValueKind::Dto(Dto {
            name: name.trim(),
            fields,
        }),
    ))
}

fn parse_string_dto(input: &str) -> IResult<&str, DtoValueKind> {
    let (input, value) = take_till(|c: char| c == '>')(input)?;
    Ok((input, DtoValueKind::String(value)))
}

fn parse_dto_value_kind<'a>(input: &'a str) -> IResult<&'a str, DtoValueKind<'a>> {
    alt((parse_dto, parse_string_dto))(input)
}

/// demo input: org.opentest4j.AssertionFailedError: expected: <User(firstName=null, lastname=asd)> but was: <User(firstName=null, lastname=aaa)>
fn parse(input: &str) -> IResult<&str, AssertionFailedError> {
    let (input, _) = take_until("expected: ")(input)?;
    let (input, _) = tag("expected: ")(input)?;
    let (input, expected) = delimited(tag("<"), parse_dto_value_kind, tag(">"))(input)?;
    let (input, _) = tag(" but was: ")(input)?;
    let (input, real) = delimited(tag("<"), parse_dto_value_kind, tag(">"))(input)?;
    Ok((input, AssertionFailedError { expected, real }))
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse, AssertionFailedError, Dto, DtoField, DtoFieldValue, DtoValueKind};
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_string() {
        let input = "org.opentest4j.AssertionFailedError: expected: <1> but was: <2>";
        let expected = AssertionFailedError {
            expected: DtoValueKind::String("1"),
            real: DtoValueKind::String("2"),
        };

        assert_eq!(parse(input), Ok(("", expected)));
    }

    #[test]
    fn parse_object() {
        let input = "org.opentest4j.AssertionFailedError: expected: <User(firstName=null, lastname=asd)> but was: <User(firstName=null, lastname=aaa)>";
        let expected = AssertionFailedError {
            expected: DtoValueKind::Dto(Dto {
                name: "User",
                fields: vec![
                    DtoField {
                        name: "firstName",
                        value: DtoFieldValue::Null,
                    },
                    DtoField {
                        name: "lastname",
                        value: DtoFieldValue::String("asd"),
                    },
                ],
            }),
            real: DtoValueKind::Dto(Dto {
                name: "User",
                fields: vec![
                    DtoField {
                        name: "firstName",
                        value: DtoFieldValue::Null,
                    },
                    DtoField {
                        name: "lastname",
                        value: DtoFieldValue::String("aaa"),
                    },
                ],
            }),
        };

        assert_eq!(parse(input), Ok(("", expected)));
    }
}
