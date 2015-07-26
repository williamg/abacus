use lexer;
use parser;

use std::io;
use std::io::Write;

pub fn read() -> String {
	print!(">> ");
    io::stdout().flush().ok().expect("Couldn't flush!");

	let mut input = String::new();

	match io::stdin().read_line(&mut input) {
		Ok(_) => return input,
		Err(_) => return input,
	}
}

pub fn handle(result: String) -> () {
	// Strip '\n'
	let mut input = result;
	input.pop ();

	let mut tokens = lexer::lex (input);

	println!("Lexed: {:?}", tokens);
	let parsed = parser::parse (&mut tokens);

	//println!("Parsed: {:?}", parsed);
}
