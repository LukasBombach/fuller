extern crate ucd;

use phf::phf_map;
use ucd::Codepoint;

const BLOCK: u32 = 1;
const EMPTY_STATEMENT: u32 = 2;
const DO_WHILE_LOOP: u32 = 3;
const IF_STATEMENT: u32 = 4;
const FOR_LOOP: u32 = 5;
const LEXICAL_DECLARATION: u32 = 6;
const TRY_STATEMENT: u32 = 7;
const WITH_STATEMENT: u32 = 8;
const BREAK_STATEMENT: u32 = 9;
const CLASS_DECLARATION: u32 = 10;
const THROW_STATEMENT: u32 = 11;
const WHILE_LOOP: u32 = 12;
const RETURN_STATEMENT: u32 = 13;
const CONTINUE_STATEMENT: u32 = 14;
const DEBUGGER_STATEMENT: u32 = 15;
const FUNCTION_DECLARATION: u32 = 16;
const ASYNC_KEYWORD: u32 = 17;
const IDENTIFIER: u32 = 18;

static PROGRAM: phf::Map<&'static str, u32> = phf_map! {
    // 1 char
    "{" => BLOCK,
    ";" => EMPTY_STATEMENT,

    // 1 char (these are the character any other keyword cannot begin with)
    "e" => IDENTIFIER,
    "g" => IDENTIFIER,
    "h" => IDENTIFIER,
    "j" => IDENTIFIER,
    "k" => IDENTIFIER,
    "m" => IDENTIFIER,
    "n" => IDENTIFIER,
    "o" => IDENTIFIER,
    "p" => IDENTIFIER,
    "q" => IDENTIFIER,
    "s" => IDENTIFIER,
    "u" => IDENTIFIER,
    "x" => IDENTIFIER,
    "y" => IDENTIFIER,
    "z" => IDENTIFIER,

    // 2 chars + whitespace
    "do"  => DO_WHILE_LOOP,
    "if"  => IF_STATEMENT,

    // 4 chars
    "for " => FOR_LOOP,
    "let " => LEXICAL_DECLARATION,
    "try " => TRY_STATEMENT,
    "var " => LEXICAL_DECLARATION,

    // 5 chars
    "with " => WITH_STATEMENT,

    // 6 chars
    "break " => BREAK_STATEMENT,
    "class " => CLASS_DECLARATION,
    "const " => LEXICAL_DECLARATION,
    "throw " => THROW_STATEMENT,
    "while " => WHILE_LOOP,
    "async " => ASYNC_KEYWORD,

    // 7 chars
    "return " => RETURN_STATEMENT,

    // 9 chars
    "continue " => CONTINUE_STATEMENT,
    "debugger " => DEBUGGER_STATEMENT,
    "function " => FUNCTION_DECLARATION,
};

fn main() {
    // let source_code = "let myvar = \"value\";";
    let source_code = "while (true) { console.log(); }";


    let chars = source_code.chars();

    match chars.nth(0) {
        "{" => println!("BLOCK"),
        ";" => println!("EMPTY_STATEMENT"),
        "e"| "g"| "h"| "j"| "k"| "m"| "n"| "o"| "p"| "q"| "s"| "u"| "x"| "y"| "z" =>

    }  

    match PROGRAM.get(&source_code[..1]) {
        Some(&BLOCK) => {
            if &source_code[1..2].is_id_continue() {
                println!("BLOCK")
            }
        }
        Some(&EMPTY_STATEMENT) => {
            if Codepoint::is_id_continue(&source_code[1..2]) {
                println!("EMPTY_STATEMENT")
            }
        }
        _ => {}
    }

    match PROGRAM.get(&source_code[..2]) {
        Some(&DO_WHILE_LOOP) => println!("DO_WHILE_LOOP"),
        Some(&IF_STATEMENT) => println!("IF_STATEMENT"),
        _ => {}
    }

    match PROGRAM.get(&source_code[..4]) {
        Some(&FOR_LOOP) => println!("FOR_LOOP"),
        Some(&LEXICAL_DECLARATION) => println!("LEXICAL_DECLARATION"),
        Some(&TRY_STATEMENT) => println!("TRY_STATEMENT"),
        _ => {}
    }

    if let Some(&WITH_STATEMENT) = PROGRAM.get(&source_code[..5]) {
        println!("WITH_STATEMENT");
    }

    match PROGRAM.get(&source_code[..6]) {
        Some(&BREAK_STATEMENT) => println!("BREAK_STATEMENT"),
        Some(&CLASS_DECLARATION) => println!("CLASS_DECLARATION"),
        Some(&LEXICAL_DECLARATION) => println!("LEXICAL_DECLARATION"),
        Some(&THROW_STATEMENT) => println!("THROW_STATEMENT"),
        Some(&WHILE_LOOP) => println!("WHILE_LOOP"),
        Some(&ASYNC_KEYWORD) => println!("ASYNC_KEYWORD"),
        _ => {}
    }

    if let Some(&RETURN_STATEMENT) = PROGRAM.get(&source_code[..7]) {
        println!("RETURN_STATEMENT");
    }

    match PROGRAM.get(&source_code[..9]) {
        Some(&CONTINUE_STATEMENT) => println!("CONTINUE_STATEMENT"),
        Some(&DEBUGGER_STATEMENT) => println!("DEBUGGER_STATEMENT"),
        Some(&FUNCTION_DECLARATION) => println!("FUNCTION_DECLARATION"),
        _ => {}
    }
}
