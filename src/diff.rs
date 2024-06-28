use itertools::Itertools;

use crate::parser::{Dto, DtoField, ValueKind};

#[derive(Debug, PartialEq)]
pub enum Difference<'a> {
    Equal,
    TypeDifference(&'a str, &'a str),
    Child(Vec<Difference<'a>>),
    DtoChange((&'a str, Vec<Difference<'a>>)),
    CharsEqual(String),
    CharsRemove(String),
    CharsAdd(String),
    UndefinedLeft(Option<ValueKind<'a>>),
    UndefinedRight(Option<ValueKind<'a>>),
    ClassChange(Box<Difference<'a>>),
    FieldNameChange((&'a str, Box<Difference<'a>>)),
    FieldValueChange((&'a str, Box<Difference<'a>>)),
}

pub fn diff<'a>(a: ValueKind<'a>, b: ValueKind<'a>) -> Difference<'a> {
    match (a, b) {
        (ValueKind::Null, ValueKind::Null) => Difference::Equal,
        (ValueKind::Null, ValueKind::String(_)) => Difference::TypeDifference("null", "String"),
        (ValueKind::Null, ValueKind::Array(_)) => Difference::TypeDifference("null", "Array"),
        (ValueKind::Null, ValueKind::Map(_)) => Difference::TypeDifference("null", "Map"),
        (ValueKind::Null, ValueKind::Dto(_)) => Difference::TypeDifference("null", "Class"),
        (ValueKind::Null, ValueKind::Field(_)) => Difference::TypeDifference("null", "Field"),
        (ValueKind::String(_), ValueKind::Null) => Difference::TypeDifference("String", ""),
        (ValueKind::String(a), ValueKind::String(b)) => diff_string(a, b),
        (ValueKind::String(_), ValueKind::Array(_)) => {
            Difference::TypeDifference("String", "Array")
        }
        (ValueKind::String(_), ValueKind::Map(_)) => Difference::TypeDifference("String", "Map"),
        (ValueKind::String(_), ValueKind::Dto(_)) => Difference::TypeDifference("String", "Class"),
        (ValueKind::String(_), ValueKind::Field(_)) => {
            Difference::TypeDifference("String", "Field")
        }
        (ValueKind::Array(_), ValueKind::Null) => Difference::TypeDifference("Array", "null"),
        (ValueKind::Array(_), ValueKind::String(_)) => {
            Difference::TypeDifference("Array", "String")
        }
        (ValueKind::Array(a), ValueKind::Array(b)) => diff_array(a, b),
        (ValueKind::Array(_), ValueKind::Map(_)) => Difference::TypeDifference("Array", "Map"),
        (ValueKind::Array(_), ValueKind::Dto(_)) => Difference::TypeDifference("Array", "Dto"),
        (ValueKind::Array(_), ValueKind::Field(_)) => Difference::TypeDifference("Array", "Field"),
        (ValueKind::Map(_), ValueKind::Null) => Difference::TypeDifference("Map", "null"),
        (ValueKind::Map(_), ValueKind::String(_)) => Difference::TypeDifference("Map", "String"),
        (ValueKind::Map(_), ValueKind::Array(_)) => Difference::TypeDifference("Map", "Array"),
        (ValueKind::Map(a), ValueKind::Map(b)) => diff_array(a, b),
        (ValueKind::Map(_), ValueKind::Dto(_)) => Difference::TypeDifference("Map", "Class"),
        (ValueKind::Map(_), ValueKind::Field(_)) => Difference::TypeDifference("Map", "Field"),
        (ValueKind::Dto(_), ValueKind::Null) => Difference::TypeDifference("Dto", "null"),
        (ValueKind::Dto(_), ValueKind::String(_)) => Difference::TypeDifference("Dto", "String"),
        (ValueKind::Dto(_), ValueKind::Array(_)) => Difference::TypeDifference("Dto", "Array"),
        (ValueKind::Dto(_), ValueKind::Map(_)) => Difference::TypeDifference("Dto", "Map"),
        (ValueKind::Dto(a), ValueKind::Dto(b)) => diff_dto(a, b),
        (ValueKind::Dto(_), ValueKind::Field(_)) => Difference::TypeDifference("Dto", "Field"),
        (ValueKind::Field(_), ValueKind::Null) => Difference::TypeDifference("Field", "null"),
        (ValueKind::Field(_), ValueKind::String(_)) => {
            Difference::TypeDifference("Field", "String")
        }
        (ValueKind::Field(_), ValueKind::Array(_)) => Difference::TypeDifference("Field", "Array"),
        (ValueKind::Field(_), ValueKind::Map(_)) => Difference::TypeDifference("Field", "Map"),
        (ValueKind::Field(_), ValueKind::Dto(_)) => Difference::TypeDifference("Field", "Class"),
        (ValueKind::Field(a), ValueKind::Field(b)) => diff_field(a, b),
    }
}

fn diff_fields<'a>(a: Vec<DtoField<'a>>, b: Vec<DtoField<'a>>) -> Difference<'a> {
    if a == b {
        return Difference::Equal;
    }
    let o = a
        .into_iter()
        .zip_longest(b.into_iter())
        .map(|d| match d {
            itertools::EitherOrBoth::Both(a, b) => diff_field(Box::new(a), Box::new(b)),
            _ => Difference::Equal,
        })
        .collect_vec();
    return Difference::Child(o);
}
fn diff_field<'a>(a: Box<DtoField<'a>>, b: Box<DtoField<'a>>) -> Difference<'a> {
    if a == b {
        return Difference::Equal;
    }

    let name_diff = diff_string(a.name, b.name);
    if name_diff != Difference::Equal {
        return Difference::FieldNameChange((a.name, Box::new(name_diff)));
    }

    let value_diff = diff(a.value, b.value);
    if value_diff != Difference::Equal {
        return Difference::FieldValueChange((a.name, Box::new(value_diff)));
    }

    return Difference::Equal;
}

fn diff_dto<'a>(a: Dto<'a>, b: Dto<'a>) -> Difference<'a> {
    if a == b {
        return Difference::Equal;
    }

    if a.name != b.name {
        return Difference::ClassChange(Box::new(diff_string(a.name, b.name)));
    }

    let fields_change = diff_fields(a.fields, b.fields);
    if let Difference::Child(o) = fields_change {
        return Difference::DtoChange((a.name, o));
    }
    fields_change
}

fn diff_array<'a>(a: Vec<ValueKind<'a>>, b: Vec<ValueKind<'a>>) -> Difference<'a> {
    if a == b {
        return Difference::Equal;
    }
    let o = a
        .into_iter()
        .zip_longest(b.into_iter())
        .map(|d| match d {
            itertools::EitherOrBoth::Both(a, b) => diff(a, b),
            itertools::EitherOrBoth::Left(d) => Difference::UndefinedRight(Some(d)),
            itertools::EitherOrBoth::Right(d) => Difference::UndefinedLeft(Some(d)),
        })
        .collect_vec();
    return Difference::Child(o);
}

fn diff_string<'a>(a: &'a str, b: &'a str) -> Difference<'a> {
    if a == b {
        return Difference::Equal;
    }
    let diffs = difference::Changeset::new(a, b, "").diffs;

    let o: Vec<Difference<'_>> = diffs
        .into_iter()
        .map(|d| match d {
            difference::Difference::Same(d) => Difference::CharsEqual(d),
            difference::Difference::Add(d) => Difference::CharsAdd(d),
            difference::Difference::Rem(d) => Difference::CharsRemove(d),
        })
        .collect();
    return Difference::Child(o);
}

#[cfg(test)]
mod tests {

    use crate::{
        diff::{self, Difference},
        test_data,
    };
    use pretty_assertions::assert_eq;

    use super::diff_string;

    #[test]
    fn string_test() {
        let out = diff_string("hey ", "hey there");

        assert_eq!(
            out,
            Difference::Child(vec![
                Difference::CharsEqual("hey ".to_string()),
                Difference::CharsAdd("there".to_string())
            ])
        );
    }

    #[test]
    fn complicated() {
        let complicated = test_data::get_complicated_expected();
        let out = diff::diff(complicated.expected, complicated.real);

        assert_eq!(
            out,
            Difference::DtoChange((
                "Complicated",
                vec![
                    Difference::Equal,
                    Difference::Equal,
                    Difference::Equal,
                    Difference::Equal,
                    Difference::FieldValueChange((
                        "e",
                        Box::new(Difference::Child(vec![Difference::FieldValueChange((
                            "eee",
                            Box::new(Difference::DtoChange(("Complicated", vec![
                                Difference::FieldValueChange((
                                    "a",
                                    Box::new(Difference::Child(vec![
                                        Difference::CharsRemove("a".to_string()),
                                        Difference::CharsAdd("b".to_string())
                                    ]))
                                )),
                                Difference::Equal,
                                Difference::Equal,
                                Difference::Equal,
                                Difference::Equal,
                                Difference::Equal,
                                Difference::Equal
                            ])))
                        ))]))
                    )),
                    Difference::Equal,
                    Difference::Equal
                ]
            ))
        );
    }
}
