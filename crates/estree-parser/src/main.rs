extern crate ucd;
use std::str::Chars;
use std::string::String;
use ucd::Codepoint;

#[derive(Debug, PartialEq)]
pub enum Thing {
    Const,
    Identifier,
    Unknown(char),
    Nothing,
}

fn matches_next(code: &mut Chars, fragment: &'static str) -> bool {
    for c in fragment.chars() {
        if Some(c) != code.next() {
            return false;
        }
    }
    true
}

#[allow(clippy::needless_return)]
fn detect_the_thing(code: &mut Chars) -> Thing {
    return match code.next() {
        Some('c') => {
            if matches_next(code, "onst") {
                if let Some(c) = code.next() {
                    if c.is_id_continue() {
                        return Thing::Identifier;
                    } else {
                        /* only true if first character of expression? */
                        return Thing::Const;
                    }
                }
            }
            Thing::Unknown('x')
        }
        Some(c) => Thing::Unknown(c),
        None => Thing::Nothing,
    };
}

fn main() {
    let mut code = "const x = 1;".chars();
    let what_we_have = detect_the_thing(&mut code);

    println!("\n\n");
    println!("Got   {:?}", what_we_have);
    println!("Rest  {}", code.collect::<String>());
    println!("\n\n");
}
