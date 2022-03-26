#[macro_export]
macro_rules! match_keyword {
    ($a:block) => {
        $a
    };
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        enum Keyword {
            Const,
            Unknown,
        }

        let input = "const";

        let result = match_keyword! {
            "const" => Keyword::Const,
            _ => Keyword::Unknown
        };

        assert_eq!(result, Keyword::Const);
    }
}
