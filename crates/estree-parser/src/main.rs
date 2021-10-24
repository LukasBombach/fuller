mod program;
mod variable_declaration;

fn main() {
    program::get_ast(";".chars());
}
