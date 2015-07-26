pub mod lexer;
pub mod rational;
pub mod parser;
pub mod repl;

fn main() {
    loop {
        repl::handle (repl::read ());
    }
}
