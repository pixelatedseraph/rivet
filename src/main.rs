use crate::lexer::Lexer;
use std::env::args;

mod lexer;

fn main() {
    let cl_args: Vec<_> = args().collect();

    if cl_args.len() != 2{
        panic!("FACKYOU");
    }

    let mut lexer = Lexer::new(&cl_args[1]);
    
    lexer.tokenize();

    println!("{:?}",lexer.tokens);

}
