extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let source_code = "let myvar = \"value\";";
    let mut graphemes = UnicodeSegmentation::graphemes(source_code, true);

    loop {
        match graphemes.next() {
            Some("{") => println!("block"),
            Some(";") => println!("empty stattement"),
            Some(first) => match graphemes.next() {
                Some(second) => {
                    println!("{}", first);
                    println!("{}", second);
                }
                None => break,
            },
            None => break,
        }
    }
}
