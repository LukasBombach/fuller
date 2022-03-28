pub trait Matcher {
    fn test(c: &char) -> ();
}


pub struct Program;
impl Matcher for Program {
    fn test(c: &char) -> () {
        match c {
            'a'..='z' => KeywordOrIdent::test(c)
        };
    }
}

pub struct KeywordOrIdent;
impl Matcher for KeywordOrIdent {
    fn test(c: &char) -> () {

    }
}