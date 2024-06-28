mod diff;
mod parser;
mod test_data;

fn main() {
    println!("Hello, world!");
    if let Ok((_, data)) = parser::parse("org.opentest4j.AssertionFailedError: expected: <Complicated(a=hey, b=2, c=500, d=600, e={eee=Complicated(a=a, b=2, c=500, d=600, e={}, f=[], g=[])}, f=[Complicated(a=thing, b=2, c=500, d=600, e={}, f=[], g=[])], g=[Complicated(a=hehe, b=2, c=500, d=600, e={}, f=[], g=[])])> but was: <Complicated(a=hey, b=2, c=500, d=600, e={eee=Complicated(a=b, b=2, c=500, d=600, e={}, f=[], g=[])}, f=[Complicated(a=thing, b=2, c=500, d=600, e={}, f=[], g=[])], g=[Complicated(a=hehe, b=2, c=500, d=600, e={}, f=[], g=[])])>") {
    let difference = diff::diff(data.expected , data.real);
    dbg!(difference);
    }
}
