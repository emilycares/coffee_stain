use itertools::Itertools;

use crate::parser::{Dto, DtoField, ValueKind};

pub fn value_code(value: ValueKind, indent: usize) -> String {
    match value {
        ValueKind::Null => "null".to_string(),
        ValueKind::String(s) => format!("\"{s}\""),
        ValueKind::Array(a) => array_code(a, indent + 2),
        ValueKind::Map(m) => map_code(m, indent + 2),
        ValueKind::Dto(d) => dto_code(d, indent + 2),
        ValueKind::Field(f) => field_code(*f, indent),
    }
}

fn dto_code(dto: Dto, indent: usize) -> String {
    format!(
        "{}.builder()\n{}{}.build()",
        dto.name,
        " ".repeat(indent),
        fields_code(dto.fields, indent)
    )
}

fn fields_code(fields: Vec<DtoField<'_>>, indent: usize) -> String {
    fields
        .into_iter()
        .map(|f| {
            format!(
                "{}.{}({})\n",
                " ".repeat(indent),
                f.name,
                value_code(f.value, indent)
            )
        })
        .join("")
}

fn field_code(field: DtoField<'_>, indent: usize) -> String {
    format!("{}, {}", field.name, value_code(field.value, indent))
}

fn map_code(map: Vec<ValueKind<'_>>, indent: usize) -> String {
    let values = map
        .into_iter()
        .map(|v| format!("{}{}", " ".repeat(indent), value_code(v, indent)))
        .join(",\n");

    format!("Map.of(\n{values}\n)")
}

fn array_code(array: Vec<ValueKind<'_>>, indent: usize) -> String {
    let values = array
        .into_iter()
        .map(|v| format!("{}{}", " ".repeat(indent), value_code(v, indent)))
        .join(",\n");

    format!("List.of(\n{values}\n)")
}
