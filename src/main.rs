use crate::{lexer::Lexer};
use std::{collections::HashMap, env::args};

mod lexer;
mod codegen;
/* 
fn main() {
    let cl_args: Vec<_> = args().collect();

    if cl_args.len() != 2{
        panic!("HEHE");
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
*/


fn main(){
    let mut cg = codegen::CodeGenerator::default();

    cg.map_define("char*","int");

    println!("{}",cg.source);

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_type_mangler() {
        assert_eq!(codegen::CodeGenerator::mangle_ptr("int"),"int");
        assert_eq!(codegen::CodeGenerator::mangle_ptr("char"),"char");
        assert_eq!(codegen::CodeGenerator::mangle_ptr("Foo"),"Foo");
        
        assert_eq!(codegen::CodeGenerator::mangle_ptr("char*"),"charptr");
        assert_eq!(codegen::CodeGenerator::mangle_ptr("char**"),"charptrptr");
        assert_eq!(codegen::CodeGenerator::mangle_ptr("char*char"),"charptrchar");
    }
}

