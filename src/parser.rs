use nom::branch::alt;
use nom::bytes::complete::{take_till, take_while};
use nom::character::complete::alpha0;
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair};
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
    pub expected: ValueKind<'a>,
    pub real: ValueKind<'a>,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum ValueKind<'a> {
    Null,
    String(&'a str),
    Array(Vec<ValueKind<'a>>),
    Map(Vec<ValueKind<'a>>),
    Dto(Dto<'a>),             // NEW
    Field(Box<DtoField<'a>>), // NEW
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Dto<'a> {
    pub name: &'a str,
    pub fields: Vec<DtoField<'a>>,
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct DtoField<'a> {
    pub name: &'a str,
    pub value: ValueKind<'a>,
}

/// Parses the last step the value
/// demo input: null
fn parse_field_string_value(input: &str) -> IResult<&str, ValueKind> {
    let (input, value) =
        take_till(|c: char| c == ',' || c == ')' || c == ']' || c == '}' || c == '>')(input)?;
    // Assuming the next character is either ',' or ')', consume it without assigning.
    match value {
        "null" => Ok((input, ValueKind::Null)),
        _ => Ok((input, ValueKind::String(value))),
    }
}

fn parse_array(input: &str) -> IResult<&str, ValueKind> {
    let (input, values) = separated_list0(pair(char(','), multispace0), parse_value_kind)(input)?;

    Ok((input, ValueKind::Array(values)))
}

fn parse_map(input: &str) -> IResult<&str, ValueKind> {
    let (input, values) = separated_list0(pair(char(','), multispace0), parse_value_kind)(input)?;

    Ok((input, ValueKind::Map(values)))
}

fn parse_field_value_kind(input: &str) -> IResult<&str, ValueKind> {
    let (input, (name, value)) = separated_pair(alpha0, tag("="), parse_value_kind)(input)?;

    Ok((input, ValueKind::Field(Box::new(DtoField { name, value }))))
}

fn parse_value_kind(input: &str) -> IResult<&str, ValueKind> {
    alt((
        delimited(tag("["), parse_array, tag("]")),
        delimited(tag("{"), parse_map, tag("}")),
        parse_field_value_kind,
        parse_dto,
        parse_field_string_value,
    ))(input)
}

/// demo input: firstName=null
fn parse_field(input: &str) -> IResult<&str, DtoField> {
    let (input, (name, value)) = separated_pair(alpha0, tag("="), parse_value_kind)(input)?;
    Ok((input, DtoField { name, value }))
}

/// demo input: firstName=null, lastname=asd
fn parse_field_list(input: &str) -> IResult<&str, Vec<DtoField>> {
    let (input, fields) = separated_list0(pair(char(','), multispace0), parse_field)(input)?;
    Ok((input, fields))
}

fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic()
}

/// demo input: User(firstName=null, lastname=asd)
fn parse_dto(input: &str) -> IResult<&str, ValueKind> {
    let (input, name) = take_while(is_alphabetic)(input)?;
    let (input, fields) = delimited(tag("("), parse_field_list, tag(")"))(input)?;
    let (input, _) = opt(multispace0)(input)?;
    Ok((input, ValueKind::Dto(Dto { name, fields })))
}

/// demo input: org.opentest4j.AssertionFailedError: expected: <User(firstName=null, lastname=asd)> but was: <User(firstName=null, lastname=aaa)>
pub fn parse(input: &str) -> IResult<&str, AssertionFailedError> {
    let (input, _) = take_until("expected: ")(input)?;
    let (input, _) = tag("expected: ")(input)?;
    let (input, expected) = delimited(tag("<"), parse_value_kind, tag(">"))(input)?;
    let (input, _) = tag(" but was: ")(input)?;
    let (input, real) = delimited(tag("<"), parse_value_kind, tag(">"))(input)?;
    Ok((input, AssertionFailedError { expected, real }))
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        parse, parse_field_value_kind, parse_map, AssertionFailedError, Dto, DtoField, ValueKind,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_string() {
        let input = "org.opentest4j.AssertionFailedError: expected: <1> but was: <2>";
        let expected = AssertionFailedError {
            expected: ValueKind::String("1"),
            real: ValueKind::String("2"),
        };

        assert_eq!(parse(input), Ok(("", expected)));
    }

    #[test]
    fn parse_object() {
        let input = "org.opentest4j.AssertionFailedError: expected: <User(firstName=null, lastname=asd)> but was: <User(firstName=null, lastname=aaa)>";
        let expected = AssertionFailedError {
            expected: ValueKind::Dto(Dto {
                name: "User",
                fields: vec![
                    DtoField {
                        name: "firstName",
                        value: ValueKind::Null,
                    },
                    DtoField {
                        name: "lastname",
                        value: ValueKind::String("asd"),
                    },
                ],
            }),
            real: ValueKind::Dto(Dto {
                name: "User",
                fields: vec![
                    DtoField {
                        name: "firstName",
                        value: ValueKind::Null,
                    },
                    DtoField {
                        name: "lastname",
                        value: ValueKind::String("aaa"),
                    },
                ],
            }),
        };

        assert_eq!(parse(input), Ok(("", expected)));
    }

    #[test]
    fn parse_nested() {
        let input = "org.opentest4j.AssertionFailedError: expected: <User(name=1, other=User(name=2, other=null))> but was: <User(name=1, other=null)>";
        let expected = AssertionFailedError {
            expected: ValueKind::Dto(Dto {
                name: "User",
                fields: vec![
                    DtoField {
                        name: "name",
                        value: ValueKind::String("1"),
                    },
                    DtoField {
                        name: "other",
                        value: ValueKind::Dto(Dto {
                            name: "User",
                            fields: vec![
                                DtoField {
                                    name: "name",
                                    value: ValueKind::String("2"),
                                },
                                DtoField {
                                    name: "other",
                                    value: ValueKind::Null,
                                },
                            ],
                        }),
                    },
                ],
            }),
            real: ValueKind::Dto(Dto {
                name: "User",
                fields: vec![
                    DtoField {
                        name: "name",
                        value: ValueKind::String("1"),
                    },
                    DtoField {
                        name: "other",
                        value: ValueKind::Null,
                    },
                ],
            }),
        };

        assert_eq!(parse(input), Ok(("", expected)));
    }

    #[test]
    fn parse_field_value_kind_test() {
        let input = "eee=as";
        let expected = ValueKind::Field(Box::new(DtoField {
            name: "eee",
            value: ValueKind::String("as"),
        }));

        assert_eq!(parse_field_value_kind(input), Ok(("", expected)));
    }

    #[test]
    fn parse_map_test() {
        let input = "eee=Comlicated(a=b, b=2, c=500, d=600)";
        let expected = ValueKind::Map(vec![ValueKind::Field(Box::new(DtoField {
            name: "eee",
            value: ValueKind::Dto(Dto {
                name: "Comlicated",
                fields: vec![
                    DtoField {
                        name: "a",
                        value: ValueKind::String("b"),
                    },
                    DtoField {
                        name: "b",
                        value: ValueKind::String("2"),
                    },
                    DtoField {
                        name: "c",
                        value: ValueKind::String("500"),
                    },
                    DtoField {
                        name: "d",
                        value: ValueKind::String("600"),
                    },
                ],
            }),
        }))]);

        assert_eq!(parse_map(input), Ok(("", expected)));
    }

    #[test]
    fn parse_complicated() {
        let input = "org.opentest4j.AssertionFailedError: expected: <Comlicated(a=hey, b=2, c=500, d=600, e={eee=Comlicated(a=a, b=2, c=500, d=600, e={}, f=[], g=[])}, f=[Comlicated(a=thing, b=2, c=500, d=600, e={}, f=[], g=[])], g=[Comlicated(a=hehe, b=2, c=500, d=600, e={}, f=[], g=[])])> but was: <Comlicated(a=hey, b=2, c=500, d=600, e={eee=Comlicated(a=b, b=2, c=500, d=600, e={}, f=[], g=[])}, f=[Comlicated(a=thing, b=2, c=500, d=600, e={}, f=[], g=[])], g=[Comlicated(a=hehe, b=2, c=500, d=600, e={}, f=[], g=[])])>";
        let expected = AssertionFailedError {
            expected: ValueKind::Dto(Dto {
                name: "Comlicated",
                fields: vec![
                    DtoField {
                        name: "a",
                        value: ValueKind::String("hey"),
                    },
                    DtoField {
                        name: "b",
                        value: ValueKind::String("2"),
                    },
                    DtoField {
                        name: "c",
                        value: ValueKind::String("500"),
                    },
                    DtoField {
                        name: "d",
                        value: ValueKind::String("600"),
                    },
                    DtoField {
                        name: "e",
                        value: ValueKind::Map(vec![ValueKind::Field(Box::new(DtoField {
                            name: "eee",
                            value: ValueKind::Dto(Dto {
                                name: "Comlicated",
                                fields: vec![
                                    DtoField {
                                        name: "a",
                                        value: ValueKind::String("a"),
                                    },
                                    DtoField {
                                        name: "b",
                                        value: ValueKind::String("2"),
                                    },
                                    DtoField {
                                        name: "c",
                                        value: ValueKind::String("500"),
                                    },
                                    DtoField {
                                        name: "d",
                                        value: ValueKind::String("600"),
                                    },
                                    DtoField {
                                        name: "e",
                                        value: ValueKind::Map(vec![ValueKind::String("")]),
                                    },
                                    DtoField {
                                        name: "f",
                                        value: ValueKind::Array(vec![ValueKind::String("")]),
                                    },
                                    DtoField {
                                        name: "g",
                                        value: ValueKind::Array(vec![ValueKind::String("")]),
                                    },
                                ],
                            }),
                        }))]),
                    },
                    DtoField {
                        name: "f",
                        value: ValueKind::Array(vec![ValueKind::Dto(Dto {
                            name: "Comlicated",
                            fields: vec![
                                DtoField {
                                    name: "a",
                                    value: ValueKind::String("thing"),
                                },
                                DtoField {
                                    name: "b",
                                    value: ValueKind::String("2"),
                                },
                                DtoField {
                                    name: "c",
                                    value: ValueKind::String("500"),
                                },
                                DtoField {
                                    name: "d",
                                    value: ValueKind::String("600"),
                                },
                                DtoField {
                                    name: "e",
                                    value: ValueKind::Map(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "f",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "g",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                            ],
                        })]),
                    },
                    DtoField {
                        name: "g",
                        value: ValueKind::Array(vec![ValueKind::Dto(Dto {
                            name: "Comlicated",
                            fields: vec![
                                DtoField {
                                    name: "a",
                                    value: ValueKind::String("hehe"),
                                },
                                DtoField {
                                    name: "b",
                                    value: ValueKind::String("2"),
                                },
                                DtoField {
                                    name: "c",
                                    value: ValueKind::String("500"),
                                },
                                DtoField {
                                    name: "d",
                                    value: ValueKind::String("600"),
                                },
                                DtoField {
                                    name: "e",
                                    value: ValueKind::Map(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "f",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "g",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                            ],
                        })]),
                    },
                ],
            }),
            real: ValueKind::Dto(Dto {
                name: "Comlicated",
                fields: vec![
                    DtoField {
                        name: "a",
                        value: ValueKind::String("hey"),
                    },
                    DtoField {
                        name: "b",
                        value: ValueKind::String("2"),
                    },
                    DtoField {
                        name: "c",
                        value: ValueKind::String("500"),
                    },
                    DtoField {
                        name: "d",
                        value: ValueKind::String("600"),
                    },
                    DtoField {
                        name: "e",
                        value: ValueKind::Map(vec![ValueKind::Field(Box::new(DtoField {
                            name: "eee",
                            value: ValueKind::Dto(Dto {
                                name: "Comlicated",
                                fields: vec![
                                    DtoField {
                                        name: "a",
                                        value: ValueKind::String("b"),
                                    },
                                    DtoField {
                                        name: "b",
                                        value: ValueKind::String("2"),
                                    },
                                    DtoField {
                                        name: "c",
                                        value: ValueKind::String("500"),
                                    },
                                    DtoField {
                                        name: "d",
                                        value: ValueKind::String("600"),
                                    },
                                    DtoField {
                                        name: "e",
                                        value: ValueKind::Map(vec![ValueKind::String("")]),
                                    },
                                    DtoField {
                                        name: "f",
                                        value: ValueKind::Array(vec![ValueKind::String("")]),
                                    },
                                    DtoField {
                                        name: "g",
                                        value: ValueKind::Array(vec![ValueKind::String("")]),
                                    },
                                ],
                            }),
                        }))]),
                    },
                    DtoField {
                        name: "f",
                        value: ValueKind::Array(vec![ValueKind::Dto(Dto {
                            name: "Comlicated",
                            fields: vec![
                                DtoField {
                                    name: "a",
                                    value: ValueKind::String("thing"),
                                },
                                DtoField {
                                    name: "b",
                                    value: ValueKind::String("2"),
                                },
                                DtoField {
                                    name: "c",
                                    value: ValueKind::String("500"),
                                },
                                DtoField {
                                    name: "d",
                                    value: ValueKind::String("600"),
                                },
                                DtoField {
                                    name: "e",
                                    value: ValueKind::Map(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "f",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "g",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                            ],
                        })]),
                    },
                    DtoField {
                        name: "g",
                        value: ValueKind::Array(vec![ValueKind::Dto(Dto {
                            name: "Comlicated",
                            fields: vec![
                                DtoField {
                                    name: "a",
                                    value: ValueKind::String("hehe"),
                                },
                                DtoField {
                                    name: "b",
                                    value: ValueKind::String("2"),
                                },
                                DtoField {
                                    name: "c",
                                    value: ValueKind::String("500"),
                                },
                                DtoField {
                                    name: "d",
                                    value: ValueKind::String("600"),
                                },
                                DtoField {
                                    name: "e",
                                    value: ValueKind::Map(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "f",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                                DtoField {
                                    name: "g",
                                    value: ValueKind::Array(vec![ValueKind::String("")]),
                                },
                            ],
                        })]),
                    },
                ],
            }),
        };

        assert_eq!(parse(input), Ok(("", expected)));
    }
}
