use colored::Colorize;
use itertools::Itertools;

use crate::{diff::Difference, parser::{DtoField, ValueKind}};

pub fn message(diff: Difference, color: bool) -> String {
    let mut out = String::new();

    out = match diff {
        Difference::Equal => out,
        Difference::TypeDifference(a, b) if color => format!(
            "{out} \"{}\" and \"{}\" are not the same Type",
            a.yellow(),
            b.yellow()
        ),
        Difference::TypeDifference(a, b) => {
            format!("{out} \"{a}\" and \"{b}\" are not the same Type")
        }
        Difference::Child(child) => format!(
            "{out} -> {}",
            child.into_iter().map(|c| message(c, color)).join("")
        ),
        Difference::DtoChange((name, diff)) => format!(
            "{out} -> {name}{}",
            diff.into_iter().map(|c| message(c, color)).join("")
        ),
        Difference::CharsEqual(s) => format!("{out}{s}"),
        Difference::CharsRemove(s) if color => format!("{out}{}", s.red()),
        Difference::CharsRemove(_) => format!("{out}"),
        Difference::CharsAdd(s) if color => format!("{out}{}", s.green()),
        Difference::CharsAdd(s) if !color => format!("{out}{}", s),
        Difference::CharsAdd(_) => format!("{out}"),
        Difference::UndefinedLeft(v) => format!("{out}additional [{}]", message_value(v, color)),
        Difference::UndefinedRight(v) => format!("{out} missing [{}]", message_value(v, color)),
        Difference::ClassChange(diff) => format!("{out}{}", message(*diff, color)),
        Difference::FieldNameChange((name, diff)) => {
            format!("{out}.{name} was {}", message(*diff, color))
        }
        Difference::FieldValueChange((name, diff)) => {
            format!("{out}.{name}{}", message(*diff, color))
        }
    };

    return out;
}

fn message_value<'a>(v: Option<ValueKind<'a>>, color: bool) -> String {
    match v {
        Some(ValueKind::Null) => format!("null"),
        Some(ValueKind::String(s)) => format!("\"{s}\""),
        Some(ValueKind::Map(m)) => m
            .into_iter()
            .map(|v| message_value(Some(v), color))
            .join(","),
        Some(ValueKind::Array(a)) => a
            .into_iter()
            .map(|v| message_value(Some(v), color))
            .join(","),
        Some(ValueKind::Dto(dto)) => {
            let fields = dto.fields
            .into_iter()
            .map(|v| message_field(v, color))
            .join(",");
            format!("{}({})", dto.name, fields)
        },
        Some(ValueKind::Field(field)) => message_field(*field, color),
        None => format!(""),
    }
}

fn message_field(field: DtoField<'_>, color: bool) -> String {
    format!("{}={}", field.name, message_value(Some(field.value), color))
}
