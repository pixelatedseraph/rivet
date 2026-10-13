use crate::{lexer::Lexer, rtable::{RTable, RTableUnit}};
use std::{collections::HashMap, env::args};

mod lexer;
mod rtable;

fn main() {
    let cl_args: Vec<_> = args().collect();

    if cl_args.len() != 2{
        panic!("FACKYOU");
    }

    let mut lexer = Lexer::new(&cl_args[1]);
    
    //lexer.tokenize();

    println!("{:?}",lexer.tokens);

    let mut map: HashMap<String,i32> = std::collections::HashMap::new();
    map.insert("Mazd".to_string(),1);
    map.insert("Simon Peyton Jones".to_string(),44);

    let mut rt = RTable::new();

    let mut u1 = RTableUnit::new(1);
    let mut u2 = RTableUnit::new(map);


    rt.table.push(u1);
    rt.table.push(u2);

    println!("{:?}",rt.table);

}
