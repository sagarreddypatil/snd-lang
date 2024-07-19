mod lexer;

use lexer::*;

fn main() {
    let path = std::env::args().nth(1).expect("no source file given");

    let lexer = Lexer::new(&path);
    let tokens = lexer.lex();

    for token in tokens {
        println!("{}", token.in_context());
    }
}
