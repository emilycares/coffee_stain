use colored::Colorize;
use itertools::Itertools;

use crate::{
    diff::Difference,
    parser::{DtoField, ValueKind},
};

pub fn message(diff: Difference, color: bool) -> String {
    let mut out = String::new();

    out = match diff {
        Difference::Equal => out,
        Difference::Type(a, b) if color => format!(
            "{out} \"{}\" and \"{}\" are not the same Type",
            a.yellow(),
            b.yellow()
        ),
        Difference::Type(a, b) => {
            format!("{out} \"{a}\" and \"{b}\" are not the same Type")
        }
        Difference::Child(child) => format!(
            "{out} -> {}",
            child.into_iter().map(|c| message(c, color)).join("")
        ),
        Difference::ArrayChange(child) => format!(
            "{out} -> [{}]",
            child.into_iter().map(|c| message(c, color)).join("")
        ),
        Difference::DtoChange((name, diff)) => format!(
            "{out} -> {name}({})",
            diff.into_iter().map(|c| message(c, color)).join("")
        ),
        Difference::CharsEqual(s) => format!("{out}{s}"),
        Difference::CharsRemove(s) if color => format!("{out}{}", s.red()),
        Difference::CharsRemove(_) => out,
        Difference::CharsAdd(s) if color => format!("{out}{}", s.green()),
        Difference::CharsAdd(s) if !color => format!("{out}{}", s),
        Difference::CharsAdd(_) => out,
        Difference::UndefinedLeft(v) => format!("{out} additional {}", message_value(v, color)),
        Difference::UndefinedRight(v) => format!("{out} missing {}", message_value(v, color)),
        Difference::ClassChange(diff) => format!("{out}{}", message(*diff, color)),
        Difference::FieldNameChange((name, diff)) => {
            format!("{out}.{name} was {}", message(*diff, color))
        }
        Difference::FieldValueChange((name, diff)) => {
            format!("{out}.{name}{}", message(*diff, color))
        }
    };

    out
}

fn message_value(v: Option<ValueKind<'_>>, color: bool) -> String {
    match v {
        Some(ValueKind::Null) => "null".to_string(),
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
            let fields = dto
                .fields
                .into_iter()
                .map(|v| message_field(v, color))
                .join(",");
            format!("{}({})", dto.name, fields)
        }
        Some(ValueKind::Field(field)) => message_field(*field, color),
        None => String::new(),
    }
}

fn message_field(field: DtoField<'_>, color: bool) -> String {
    format!("{}={}", field.name, message_value(Some(field.value), color))
}
