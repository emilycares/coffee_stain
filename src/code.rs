use itertools::Itertools;

use crate::parser::{Dto, DtoField, ValueKind};

pub fn value_code(value: ValueKind) -> String {
    match value {
        ValueKind::Null => "null".to_string(),
        ValueKind::String(s) => format!("\"{s}\""),
        ValueKind::Array(a) => array_code(a),
        ValueKind::Map(m) => map_code(m),
        ValueKind::Dto(d) => dto_code(d),
        ValueKind::Field(f) => field_code(*f),
    }
}

fn dto_code(dto: Dto) -> String {
    format!(
        "{}.builder()\n{}.build()",
        dto.name,
        fields_code(dto.fields)
    )
}

fn fields_code(fields: Vec<DtoField<'_>>) -> String {
    fields
        .into_iter()
        .map(|f| format!(".{}({})\n", f.name, value_code(f.value)))
        .join("")
}

fn field_code(field: DtoField<'_>) -> String {
    format!("{}, {}", field.name, value_code(field.value))
}

fn map_code(map: Vec<ValueKind<'_>>) -> String {
    let values = map.into_iter().map(|v| value_code(v)).join(",\n");

    format!("Map.of(\n{values}\n)")
}

fn array_code(array: Vec<ValueKind<'_>>) -> String {
    let values = array.into_iter().map(|v| value_code(v)).join(",\n");

    format!("List.of(\n{values}\n)")
}
