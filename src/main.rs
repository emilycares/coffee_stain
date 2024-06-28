use std::io;

fn main() {
    let stdin = io::stdin();
    eprintln!("Please paste the line that includes: \n  - \"org.opentest4j.AssertionFailedError: expected: <*> but was: <*>\" \n");
    let text = io::read_to_string(stdin).expect("Can not read stdin");
    if let Some(message) = coffee_stain::get_hint(&text, true) {
        println!("{}", message);
    }
}
