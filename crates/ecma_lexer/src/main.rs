use self::Keyword::*;
use match_keyword::match_keyword;

enum Keyword {
    Const,
    Ident,
    Whitespace,
    Eq,
    True,
    Semi,
}

/* break

case
catch
class
const
continue

debugger
default
delete
do

else
export
extends

false
finally
for
function

if
import
in
instanceof

new
null

return

super
switch

this
throw
true
try
typeof

var
void

while
with

yield */

fn main() {
    let input = "const conrad = true;";
    match_keyword!(match input {
        "const" => Const,
        "const" => Const,
        "=" => Eq,
        "true" => True,
        ";" => Semi,
        _ => Ident,
    });
}
