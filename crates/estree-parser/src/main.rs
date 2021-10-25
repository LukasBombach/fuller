use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Thing {
    PossiblyConst,
    Unknown(char),
    Nothing,
}

#[allow(clippy::needless_return)]
fn detect_the_thing(code: &mut Chars) -> Thing {
    return match code.next() {
        Some('c') => Thing::PossiblyConst,
        Some(c) => Thing::Unknown(c),
        None => Thing::Nothing,
    };
}

fn main() {
    let mut code = "const x = 1;".chars();
    let what_we_have = detect_the_thing(&mut code);

    println!("\n\nWe got: {:?}\n\n", what_we_have);
}
