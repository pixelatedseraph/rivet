use std::collections::HashMap;


#[derive(Debug)]
pub struct RTableUnit<T>{
    pub data: T
}

impl <T> RTableUnit<T>{
    pub fn new(data: T) -> Self{
        Self { data }
    }
}

#[derive(Debug)]
pub struct RTable<T>{
    pub table: Vec<RTableUnit<T>>,
}

impl <T> RTable<T>{
    pub fn new() -> Self{
        Self { table : (Vec::new())}
    }
}
