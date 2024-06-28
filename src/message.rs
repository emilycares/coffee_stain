use colored::Colorize;
use itertools::Itertools;

use crate::diff::Difference;

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
        Difference::CharsAdd(s) if color => format!("{out}{}", s.red()),
        Difference::CharsAdd(s) if !color => format!("{out}{}", s),
        Difference::CharsAdd(_) => format!("{out}"),
        Difference::UndefinedLeft(_) => format!("{out} Something was undefinded"),
        Difference::UndefinedRight(_) => format!("{out} Something was undefinded"),
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
