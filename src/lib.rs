mod code;
mod diff;
mod message;
mod parser;

#[cfg(test)]
mod test_data;

pub fn get_hint(text: &str, color: bool) -> Option<String> {
    match parser::parse(text) {
        Ok((_, data)) => {
            let difference = diff::diff(data.expected, data.real);
            let message = message::message(difference, color);
            Some(message)
        }
        Err(_) => None,
    }
}

pub fn to_code(text: &str) -> Option<String> {
    match parser::parse_value_kind(text) {
        Ok((_, value)) => {
            let code = code::value_code(value, 0);
            Some(code)
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_hint, to_code};
    use pretty_assertions::assert_eq;

    #[test]
    fn string_hint() {
        assert_eq!(get_hint("", false), None);
        assert_eq!(
            get_hint(
                "org.opentest4j.AssertionFailedError: expected: <1> but was: <2>",
                false
            ),
            Some(" -> 2".to_string())
        );
    }

    #[test]
    fn basic_hint() {
        assert_eq!(
            get_hint(
                "org.opentest4j.AssertionFailedError: expected: <User(firstName=null, lastname=asd)> but was: <User(firstName=null, lastname=aaa)>",
                false
            ),
            Some(" -> User(.lastname -> aaa)".to_string())
        );
    }

    #[test]
    fn nested_hint() {
        assert_eq!(
            get_hint(
                "org.opentest4j.AssertionFailedError: expected: <User(name=1, other=User(name=2, other=null))> but was: <User(name=1, other=null)>",
                false
            ),
            Some(" -> User(.other \"User\" and \"null\" are not the same Type)".to_string())
        );
    }

    #[test]
    fn list_hint() {
        assert_eq!(
            get_hint(
                "org.opentest4j.AssertionFailedError: expected: <[User(name=first, other=null)]> but was: <[User(name=first, other=null), User(name=second, other=null)]>",
                false
            ),
            Some(" -> [ additional User(name=\"second\",other=null)]".to_string())
        );
    }

    #[test]
    fn complicated_hint() {
        assert_eq!(
            get_hint(
                "org.opentest4j.AssertionFailedError: expected: <Complicated(a=hey, b=2, c=500, d=600, e={eee=Complicated(a=a, b=2, c=500, d=600, e={}, f=[], g=[])}, f=[Complicated(a=thing, b=2, c=500, d=600, e={}, f=[], g=[])], g=[Complicated(a=hehe, b=2, c=500, d=600, e={}, f=[], g=[])])> but was: <Complicated(a=hey, b=2, c=500, d=600, e={eee=Complicated(a=b, b=2, c=500, d=600, e={}, f=[], g=[])}, f=[Complicated(a=thing, b=2, c=500, d=600, e={}, f=[], g=[])], g=[Complicated(a=hehe, b=2, c=500, d=600, e={}, f=[], g=[])])>",
                false
            ),
            Some(" -> Complicated(.e -> [.eee -> Complicated(.a -> b)])".to_string())
        );
    }

    #[test]
    fn list_code() {
        println!(
            "{}",
            to_code("[User(name=first, other=null), User(name=second, other=null)]").unwrap()
        );
        assert_eq!(
            to_code("[User(name=first, other=null), User(name=second, other=null)]"),
            Some("List.of(\n  User.builder()\n        .name(\"first\")\n    .other(null)\n.build(),\n  User.builder()\n        .name(\"second\")\n    .other(null)\n.build()\n)".to_string())
        )
    }
}
