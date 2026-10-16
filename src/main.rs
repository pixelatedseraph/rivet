use crate::codegen::RMapWriter;
use crate::{lexer::Lexer};
use std::io::Write;
use std::{collections::HashMap, env::args};

mod lexer;
mod _codegen;
mod codegen;

fn main(){
    let mut rmapwriter = RMapWriter::default();
    rmapwriter.create_map("testmap", "int", "int");
    rmapwriter.write_monomorphized_code();
    
    let mut cwriter = codegen::CCodeWriter::default();
    cwriter.write_c_source(&rmapwriter);

    println!("{}",cwriter.csource);
}

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
/* 
use std::fs::File;
fn main(){
    let mut rmap = codegen::RMap::default();
    let mut testcfile = codegen::CodeGenerator::default();


    let mut accumalator = String::new();

    accumalator.push_str(&testcfile.headers());

    rmap.create("TestMap","char*", "int");
    accumalator.push_str(&rmap.map_type_specialized_code);

    accumalator.push_str(&testcfile.cmain_begin());

    rmap.insert("\"Mazeed\"", "123");
    rmap.insert("\"Faizan\"", "456");
    rmap.insert("\"Afnan\"", "789");

    rmap.at("\"Mazeed\"");
    rmap.at("\"Faizan\"");
    rmap.at("\"Afnan\"");

    accumalator.push_str(&rmap.map_usage_code);

    accumalator.push_str(&testcfile.cmain_end());

    //println!("{}",rmap.map_type_specialized_code);
    //println!("{}",rmap.map_usage_code);

    let mut file = File::create("librivet/testmap.c").unwrap();
    file.write(accumalator.as_bytes());
}*/

/* 
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

*/