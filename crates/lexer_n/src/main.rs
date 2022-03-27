use match_keyword::MatchKeyword;
use match_keyword_derive::MatchKeyword;

#[derive(MatchKeyword)]
struct Keywords;

fn main() {
    Keywords::match_keyword();
}
