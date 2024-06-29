use std::io::BufRead;

fn main() {
    eprintln!("Please paste the line that includes: \n  - \"org.opentest4j.AssertionFailedError: expected: <*> but was: <*>\" \n");

    let lines = std::io::stdin().lock().lines();
    for line in lines {
        if let Ok(line) = line {
            if let Some(message) = coffee_stain::get_hint(&line, true) {
                println!("{}", message);
            }
        }
    }
}
