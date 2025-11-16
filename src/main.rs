use std::io::BufRead;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Simplify error
    #[clap(long)]
    pub hint: bool,

    /// Print code from string version
    #[clap(long)]
    pub code: bool,
}

fn main() {
    let args = Args::parse();

    if args.hint {
        eprintln!("Please paste the line that includes: \n  - \"org.opentest4j.AssertionFailedError: expected: <*> but was: <*>\" \n");

        let lines = std::io::stdin().lock().lines();
        for line in lines.flatten() {
            if let Some(message) = coffee_stain::get_hint(&line, true) {
                println!("{}", message);
            }
        }
    }

    if args.code {
        eprintln!(
            "Please paste the a toString() version for example: \"User(name=first, other=null)\""
        );
        let lines = std::io::stdin().lock().lines();
        for line in lines.flatten() {
            if let Some(message) = coffee_stain::to_code(&line) {
                println!("{}", message);
            }
        }
    }
}
