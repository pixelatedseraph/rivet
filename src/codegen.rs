/*
  *  Author: Mazeed .A
  *  SPDX identifier: AGPL v3
  *  About: CodeGenerator Module responsible for emitting ISO C11 code from optimized IR
*/


use std::fmt::Write;
use std::collections::HashMap;
use std::collections::HashSet;


//Todo: Object Model and Methods,closures,function types,higher order functions

pub fn type_mangler(_type: &str) -> String{
    _type.chars().fold(String::new(),|mut acc,ch|{
        match ch{
            '*' => acc.push_str("ptr"),
            ch if ch.is_whitespace() => {}
            ch=> acc.push(ch),
        }
        acc
    })
}

#[derive(Debug,Default)]
pub struct CCodeWriter{
    pub csource: String
}

impl CCodeWriter{

    /*
    pub fn write_c_source(&mut self,mapwriter: &RMapWriter){
        self.generate_c_headers();
        self.append_rmap_definitions(mapwriter);
        self.generate_c_main();
        self.generate_c_main_return();
    }
    */

    pub fn write_c_source(&mut self,mapwriter:&RMapWriter,tablewriter: &RTableWriter,funcwriter:&RFunctionWriter){
        self.generate_c_headers();
        self.append_rmap_definitions(mapwriter);
        self.append_rtable_defintions(tablewriter);
        self.append_func_definitions(funcwriter);
        self.generate_c_main();
        self.generate_c_main_return();
    }

    fn generate_c_headers(&mut self){
        writeln!(self.csource,"#include<stdio.h>
        #include<stdlib.h>
        #include<string.h>
        #include<stdbool.h>
        #include<stdint.h>
        #include\"util.h\"
        ").unwrap();
    }

    fn append_rtable_defintions(&mut self,tablewriter: &RTableWriter){
        let rtable_definitions = &tablewriter.table_defintions;
        writeln!(self.csource,"{rtable_definitions}").unwrap();
    }

    fn append_rmap_definitions(&mut self,mapwriter:&RMapWriter){
        let rmap_definitions = &mapwriter.monomorphized_code;
        writeln!(self.csource,"{rmap_definitions}").unwrap();
    }

    fn append_func_definitions(&mut self,funcwriter: &RFunctionWriter){
        let fndefs = &funcwriter.defintions;
        writeln!(self.csource,"{fndefs}").unwrap();
    }
    

    fn generate_c_main(&mut self){
        writeln!(self.csource,"int main(){{").unwrap();
    }

    fn generate_c_main_return(&mut self){
        writeln!(self.csource,"return 0;
        }}").unwrap();
    }

}


#[derive(Debug,Default)]
pub struct RMap{
    name:       String,
    key_type:   String,
    value_type: String,
    map_type:   String //mangled
}

type KeyValuePair = (String,String);

#[derive(Debug,Default)]
pub struct RMapWriter{
    pub maps:   HashMap<String,RMap>,
    pub pairs:  Vec<KeyValuePair>, //umangled
    pub monomorphized_code: String,
    pub user_code: String,
}

impl RMap{
    fn new(name: &str,key: &str,val: &str) ->  Self{
         RMap{ 
            name: (name.to_string()),
            key_type: (key.to_string()), 
            value_type: (val.to_string()), 
            map_type: format!("map_{}_{}",type_mangler(key),type_mangler(val))
        }
    }
    fn write_map_type_name(key: &str,val: &str) -> String{
        let mkey = type_mangler(key);
        let mval = type_mangler(val);
        format!("map_{mkey}_{mval}")
    }
    fn write_pair_type_name(key: &str,val: &str) -> String{
        let mkey = type_mangler(key);
        let mval = type_mangler(val);
        format!("pair_{mkey}_{mval}")
    }
}

impl RMapWriter{
    pub fn create_map(&mut self,name: &str,key: &str,val: &str) {
        let map = RMap::new(name, key, val);
        self.maps.insert(name.to_string(),map);
        self.pairs.push((key.to_owned(),val.to_owned()));
    }
    pub fn write_monomorphized_code(&mut self){
        let mut seen = HashSet::new();
        self.pairs.retain(|x| seen.insert(x.clone()));   

        for unique_type_pair in self.pairs.clone(){
            self.generate_rutil_map_struct(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_create(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_put(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_get(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_contains_key(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_length(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_is_empty(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_clear(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_remove(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_print(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
            self.generate_rutil_map_function_delete(unique_type_pair.0.as_str(), unique_type_pair.1.as_str());
        }
    }
    fn generate_rutil_map_struct(&mut self,key: &str,val: &str){
        let mkey = type_mangler(key);
        let mval= type_mangler(val);
        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);

        let default_map_size = 1024; 
        writeln!(self.monomorphized_code,"typedef struct{{ {key} key; {val} val;}}{pair_typename};  

        rutil_monomorphized_vector({pair_typename},{pair_typename})

        typedef rutil_vector_{pair_typename} bucket;
        #define default_map_size {default_map_size}

        typedef struct{{
            bucket* hashtable[default_map_size];
        }}{map_typename}; ").unwrap();
    }

    fn generate_rutil_map_function_create(&mut self,key:&str,val: &str){
        let mkey = type_mangler(key);
        let mval= type_mangler(val);

        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);

        writeln!(self.monomorphized_code,"{map_typename} {map_typename}_create(){{
            {map_typename} mp = {{}};
            rutil_for_each_in_c_arr(idx,mp.hashtable){{
                mp.hashtable[idx] = newVector_{pair_typename}();
            }}
            return mp;
        }}").unwrap();
    }

    fn generate_rutil_map_function_put(&mut self,key: &str,val: &str){
        let mkey = type_mangler(key);
        let mval= type_mangler(val);

        let hashing_strategy = Self::hash_index_code(key);

        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);

        writeln!(self.monomorphized_code,"void {map_typename}_put({map_typename}* self,{key} key,{val} val){{
            u64 index = {hashing_strategy} & (default_map_size - 1);
            {pair_typename} pair = {{.key = key,.val = val}};
            pushBack_{pair_typename}(self->hashtable[index],pair);
        }}").unwrap();
    }
    //key,val refer to ctype
    fn generate_rutil_map_function_get(&mut self,key: &str,val: &str){
        let mkey = type_mangler(key);
        let mval= type_mangler(val);
        let hashing_strategy = Self::hash_index_code(key);
        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);
        let (comparator_body,comparator_fn) = self.comparator_strategy(key,val,key);
        let collision_handler = Self::collision_handler(key);

        writeln!(self.monomorphized_code,"{comparator_body}").unwrap();

        writeln!(self.monomorphized_code,"{val} {map_typename}_get({map_typename}* self,{key} key){{
            u64 index =  {hashing_strategy} & (default_map_size - 1);

            bucket* _bucket = self->hashtable[index];

            if(_bucket->length == 1){{
                {pair_typename} pair = getAtIndex_{pair_typename}(_bucket,0);
                return pair.val;
            }}
            else{{
                sortVector_{pair_typename}(_bucket,{comparator_fn});
                rutil_vec_for_each(idx, _bucket){{
                    {collision_handler}
                }}
                return NULL;
            }}
        }}").unwrap();
    }

    fn generate_rutil_map_function_contains_key(&mut self,key: &str,val: &str){
        let mkey = type_mangler(key);
        let mval= type_mangler(val);
        let hashing_strategy = Self::hash_index_code(key);
        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);
        let (comparator_body,comparator_fn) = self.comparator_strategy(key,val,key);
        let collision_handler = Self::collision_handler(key);
        let equality_strategy_0 = Self::trait_equality(key,"pair.key","key");
        let equality_strategy_1 = Self::find_key_handler(key);

        writeln!(self.monomorphized_code,"bool {map_typename}_contains_key({map_typename}* self,{key} key){{
            u64 index =  {hashing_strategy} & (default_map_size - 1);
            bucket* _bucket = self->hashtable[index];
            if(_bucket -> length == 1){{
                {pair_typename} pair = getAtIndex_{pair_typename}(_bucket,0);
                if({equality_strategy_0}){{
                    return true;
                }}
            }}
            else{{
                sortVector_{pair_typename}(_bucket,{comparator_fn});
                rutil_vec_for_each(idx, _bucket){{
                    {equality_strategy_1}
                }}
                return false;
            }}
            return false;
    }}").unwrap();
    }

    fn generate_rutil_map_function_length(&mut self,key:  &str,val: &str){
        let map_typename = RMap::write_map_type_name(key, val);

        writeln!(self.monomorphized_code,"usize {map_typename}_length({map_typename}* self){{
            usize map_length = 0;
            usize array_length = rutil_array_length(self->hashtable);
            for(usize idx = 0; idx < array_length; ++idx){{
                map_length += self->hashtable[idx]->length;
            }}
            return map_length;
        }}").unwrap();
    }

    fn generate_rutil_map_function_is_empty(&mut self,key: &str,val: &str){
        let map_typename = RMap::write_map_type_name(key, val);

        writeln!(self.monomorphized_code,"bool {map_typename}_is_empty({map_typename}* self){{
            usize map_length = {map_typename}_length(self);
            if(map_length == 0UL)
                return true;
            return false;
        }}").unwrap();
    }

    fn generate_rutil_map_function_clear(&mut self,key: &str,val: &str){
        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);
        
        writeln!(self.monomorphized_code,"void {map_typename}_clear({map_typename}* self){{
            usize array_length = rutil_array_length(self->hashtable);
            for(usize idx = 0; idx < array_length; ++idx){{
                clearVector_{pair_typename}(self->hashtable[idx]);
            }}
        }}").unwrap();
    }
    
    fn generate_rutil_map_function_print(&mut self,key: &str,val: &str){
        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);
        let pair_printer_strategy = self.pair_printer(key, val);

        let prettyprint_openbrace = format!("printf(\"%c \",'{{');");
        let prettyprint_newline = format!("printf(\"%c\",'\\n');");
        let prettyprint_closebrace = format!("printf(\"%c\",'}}');");
        let prettyprint_comma = format!("printf(\"%c\",',');");

        writeln!(self.monomorphized_code,"void {map_typename}_print({map_typename}* self){{
            usize array_length = rutil_array_length(self->hashtable);
            {prettyprint_openbrace}
            for(usize idx = 0; idx < array_length; ++idx){{
                bucket* _bucket = self->hashtable[idx];
                rutil_vec_for_each(idx,_bucket){{
                    {pair_typename} pair = getAtIndex_{pair_typename}(_bucket,idx);
                    {pair_printer_strategy}
                }}
            }}
            {prettyprint_closebrace}
            {prettyprint_newline}
        }}").unwrap();
    }

    fn generate_rutil_map_function_remove(&mut self,key: &str,val: &str){
        let mkey = type_mangler(key);
        let mval= type_mangler(val);
        let hashing_strategy = Self::hash_index_code(key);
        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);
        let (comparator_body,comparator_fn) = self.comparator_strategy(key,val,key);
        let collision_handler = Self::collision_handler(key);
        let equality_strategy_0 = Self::trait_equality(key,"pair.key","key");
        let equality_strategy_1 = Self::find_key_handler(key);
        let remove_handler = Self::remove_element_handler(key, val, val);

        writeln!(self.monomorphized_code,"void {map_typename}_remove({map_typename}* self,{key} key){{
            u64 index = {hashing_strategy} & (default_map_size - 1);            

            bucket* _bucket = self->hashtable[index];
            if(_bucket -> length == 1){{
                (void)pop_{pair_typename}(_bucket,0);
                return;
            }}

            else{{
                rutil_vec_for_each(idx,_bucket){{
                    {remove_handler}
                    index++;
                }}
            }}
        }}").unwrap();

    }

    fn generate_rutil_map_function_delete(&mut self,key: &str,val: &str) {
        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);
        writeln!(self.monomorphized_code,"void {map_typename}_delete({map_typename}* self){{
            usize array_length = rutil_array_length(self->hashtable);
            for(usize idx = 0; idx < array_length; ++idx){{
                deleteVector_{pair_typename}(&(self->hashtable[idx]));
            }}
        }}").unwrap();
    }

    fn pair_printer(&self,key: &str,val: &str) -> String{
        let mut pair_print = String::new();

        let mut fmtstr = String::new();
        let mut fmtargs = String::new();
        
        match key{
            "rutil_string*" =>{
                write!(fmtstr,"%s: ").unwrap();
                write!(fmtargs,"pair.key->vec_chars->data").unwrap();
                write!(pair_print,"%s,{}: ","pair.key->vec_chars->data").unwrap();
                match val{
                    "i8" | "i16" | "i32" | "i64" =>{
                        write!(pair_print,"%ld,{}","pair.val").unwrap();
                        write!(fmtstr,"%ld").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "u8" | "u16" | "u32" | "u64" =>{
                        write!(pair_print,"%lu,{}","pair.val").unwrap();
                        write!(fmtstr,"%lu").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "rutil_string*" =>{
                        write!(pair_print,"%s,{}","pair.key->vec_chars->data").unwrap();
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val->vec_chars->data").unwrap();
                    }
                    "char" =>{
                        write!(fmtstr,"%c").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "char*" | "const char*" =>{
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    _ =>{
                        panic!("Unimplemented Type");
                    }
                }
            }
            "u8 | u16 | u32 | u64" => {
                write!(fmtstr,"%lu: ").unwrap();
                write!(fmtargs,"pair.key").unwrap();
                write!(pair_print,"%lu,{}: ","pair.key").unwrap();
                match val{
                    "i8" | "i16" | "i32" | "i64" =>{
                        write!(pair_print,"%ld,{}","pair.val").unwrap();
                        write!(fmtstr,"%ld").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "u8" | "u16" | "u32" | "u64" =>{
                        write!(pair_print,"%lu,{}","pair.val").unwrap();
                        write!(fmtstr,"%lu").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "rutil_string*" =>{
                        write!(pair_print,"%s,{}","pair.key->vec_chars->data").unwrap();
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val->vec_chars->data").unwrap();
                    }
                    "char" =>{
                        write!(fmtstr,"%c").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "char*" | "const char*"=>{
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    _ =>{
                        panic!("Unimplemented Type");
                    }
                }
            }
            "i8" | "i16" | "i32" | "i64" =>{
                write!(fmtstr,"%ld: ").unwrap();
                write!(fmtargs,"pair.key").unwrap();
                write!(pair_print,"%ld,{}: ","pair.key").unwrap();
                match val{
                    "i8" | "i16" | "i32" | "i64" =>{
                        write!(pair_print,"%ld,{}","pair.val").unwrap();
                        write!(fmtstr,"%ld").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "u8" | "u16" | "u32" | "u64" =>{
                        write!(pair_print,"%lu,{}","pair.val").unwrap();
                        write!(fmtstr,"%lu").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "rutil_string*" =>{
                        write!(pair_print,"%s,{}","pair.key->vec_chars->data").unwrap();
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val->vec_chars->data").unwrap();
                    }
                    "char*" | "const char*" =>{
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "char" =>{
                        write!(fmtstr,"%c").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    _ =>{
                        panic!("Unimplemented Type");
                    }
                }
            }
            "const char*" | "char*" =>{
                write!(fmtstr,"%s").unwrap();
                write!(fmtargs,"pair.key").unwrap();
                match val{
                    "i8" | "i16" | "i32" | "i64" =>{
                        write!(pair_print,"%ld,{}","pair.val").unwrap();
                        write!(fmtstr,"%ld").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "u8" | "u16" | "u32" | "u64" =>{
                        write!(pair_print,"%lu,{}","pair.val").unwrap();
                        write!(fmtstr,"%lu").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "rutil_string*" =>{
                        write!(pair_print,"%s,{}","pair.key->vec_chars->data").unwrap();
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val->vec_chars->data").unwrap();
                    }
                    "char*" | "const char*" =>{
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "char" =>{
                        write!(fmtstr,"%c").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    _ =>{
                        panic!("Unimplemented Type");
                    }
                }
            }

            "char" =>{
                write!(fmtstr,"%c").unwrap();
                write!(fmtargs,"pair.key").unwrap();
                match val{
                    "i8" | "i16" | "i32" | "i64" =>{
                        write!(pair_print,"%ld,{}","pair.val").unwrap();
                        write!(fmtstr,"%ld").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "u8" | "u16" | "u32" | "u64" =>{
                        write!(pair_print,"%lu,{}","pair.val").unwrap();
                        write!(fmtstr,"%lu").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "rutil_string*" =>{
                        write!(pair_print,"%s,{}","pair.key->vec_chars->data").unwrap();
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val->vec_chars->data").unwrap();
                    }
                    "char*" | "const char*" =>{
                        write!(fmtstr,"%s").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    "char" =>{
                        write!(fmtstr,"%c").unwrap();
                        write!(fmtargs,",pair.val").unwrap();
                    }
                    _ =>{
                        panic!("Unimplemented Type");
                    }
                }
            }
            _ => {
                panic!("Unsupported Type");
            }
        }
        fmtstr.push(' ');
        let cprintf = format!("printf(\"{}\",{});",fmtstr,fmtargs);
        //let cprintf = format!("printf(\"{fmtstr}\",{fmtargs});");
        cprintf
    }


    fn remove_element_handler(key: &str,val: &str,ctype: &str) -> String{
        let mut rmh = String::new();
        let pair_typename = RMap::write_pair_type_name(key, val);
        
        if ctype == "rutil_string*"{
            write!(rmh,"u64 index = 0;
            if(rutil_string_cmp(_bucket->data[idx].key,key) == 0){{
                pop_{pair_typename}(_bucket,index);
                return;
            }}").unwrap();
            return rmh;
        }
        else if ctype == "const char*" || ctype == "char*"{
            write!(rmh,"u64 index = 0;
            if(strcmp(_bucket->data[index].key,key) == 0){{
                pop_{pair_typename}(_bucket,index);
                return;
            }}").unwrap();
            return rmh;
        }
        else{
            write!(rmh,"u64 index = 0;
            if(_bucket->data[idx].key == key){{
                pop_{pair_typename}(_bucket,index);
                return;
            }}").unwrap();
            return rmh;
        }
    }


    fn find_key_handler(ctype: &str) -> String{
        let mut key_search_handler = String::new();

        if ctype == "rutil_string*"{
            write!(key_search_handler,"if(rutil_string_cmp(_bucket->data[idx].key,key) == 0){{
                        return true; 
                    }}").unwrap();
            return key_search_handler;
        }
        else if ctype == "const char*" || ctype == "char*"{
            write!(key_search_handler,"if(strcmp(_bucket->data[idx].key,key) == 0){{
                        return true;
        }}").unwrap();
            return key_search_handler;
        }
        else{
            write!(key_search_handler,"if(_bucket->data[idx].key == key){{
                        return true;
                    }}").unwrap();
            return key_search_handler;
        }
    }

    fn collision_handler(ctype: &str) -> String{
        let mut collision_code = String::new();

        if ctype == "rutil_string*"{
            write!(collision_code,"if(rutil_string_cmp(_bucket->data[idx].key,key) == 0){{
                        return _bucket->data[idx].val; 
                    }}").unwrap();
            return collision_code;
        }
        else if ctype == "const char*" || ctype == "char*"{
            write!(collision_code,"if(strcmp(_bucket->data[idx].key,key) == 0) {{
                    return _bucket->data[idx].val;
                }}").unwrap();
            return collision_code;
        }
        else{
            write!(collision_code,"if(_bucket->data[idx].key == key){{
                            return _bucket->data[idx].val;
                        }}").unwrap();
            return collision_code;
        }
    }
    fn hash_index_code(key: &str) -> String {
        if key == "char*" || key == "const char*"{
            "rutil_hash(key, strlen(key))".to_string()
        }
        else if key == "rutil_string*"{
            "rutil_hash(key->vec_chars->data, strlen(key->vec_chars->data))".to_string()
        }
        else {
            "rutil_hash(&key, sizeof(key))".to_string()
         }
     }

    //the ctype binds both lhs & rhs
    fn trait_equality(ctype: &str,lhs: &str,rhs: &str) -> String{
        if ctype == "rutil_string*"{ //string
            return format!("rutil_string_eq({lhs},{rhs})");
        }
        else if ctype == "const char*" || ctype == "char*" {
            return format!("rutil_cstring_eq({lhs},{rhs})")
        }
        else{ //int,double,char
            return format!("{lhs} == {rhs}");
        }
    }

    //keys should be comparable, so they must implement behaviour/trait to compare them 
    fn comparator_strategy(&self,key: &str,val :&str,ctype : &str) -> (String,String){
        if ctype == "rutil_string*"{
            return self.generate_trait_comparator_for_type_rstring(key, val);
        }
        else if ctype == "const char*" || ctype == "char*"{
            return self.generate_trait_comparator_for_type_cstring(key, val);
        }
        else{
            return self.generate_trait_comparator_numeric(key, val, ctype);
        }
    }   

    fn generate_trait_comparator_for_type_rstring(&self,key: &str,val: &str) -> (String,String){
        let mut comparator = String::new();
        let pair_typename = RMap::write_pair_type_name(key, val);

        let comparator_fn = format!("rutil_cmprator_rstring");

        writeln!(comparator,"int {comparator_fn}(const void* lhs,const void* rhs){{
            {pair_typename} _lhs = *({pair_typename}*)lhs;
            {pair_typename} _rhs = *({pair_typename}*)rhs;

            rutil_string* _a = _lhs.key;
            rutil_string* _b = _rhs.key;

            return rutil_string_cmp(_a,_b);
        }}").unwrap();
        (comparator,comparator_fn)
    }

    fn generate_trait_comparator_for_type_cstring(&self,key: &str,val: &str) -> (String,String){
        let mut comparator = String::new();
        let pair_typename = RMap::write_pair_type_name(key, val);

        let comparator_fn = format!("rutil_cmprator_cstring");

        writeln!(comparator,"int {comparator_fn}(const void* lhs,const void* rhs){{
            {pair_typename} _lhs = *({pair_typename}*)lhs;
            {pair_typename} _rhs = *({pair_typename}*)rhs;

            const char* _a = _lhs.key;
            const char* _b = _rhs.key;

            return strcmp(_a,_b);
        }}").unwrap();
        (comparator,comparator_fn)
    }

    //int,uint,double all sizes,char also supported
    fn generate_trait_comparator_numeric(&self,key: &str,val: &str,ctype : &str) -> (String,String){
        let mut comparator = String::new();
        let pair_typename = RMap::write_pair_type_name(key, val);

        let mctype = type_mangler(ctype);
        let comparator_fn = format!("rutil_cmprator_{mctype}");

        writeln!(comparator,"int {comparator_fn}(const void* lhs,const void* rhs){{
    
            {pair_typename} _lhs = *({pair_typename}*)lhs;
            {pair_typename} _rhs = *({pair_typename}*)rhs;
        
            {ctype} _a = _lhs.key;
            {ctype} _b = _rhs.key;

            if(_a > _b)
                return 1;
            else if(_a < _b)
                return -1;
            else 
                return 0;
        }}").unwrap();
        (comparator,comparator_fn)
    }
}

type FunctionParameters = Vec<(String,String)>; //vector of tuples of parameter name and type
//ex add(a: int,b: int) -> Vec<(a,int),(b,int)>  

#[derive(Debug,Default)]
pub struct RFunction{
    name: String,
    is_method: bool,
    parameters : FunctionParameters,
    return_expr_type: String,
}

impl RFunction{
    fn new(name: String,
        is_method: bool,
        parameters: FunctionParameters,
        return_expr_type: String) 
        -> Self
    {
        Self { name, is_method, parameters, return_expr_type }
    }
}

#[derive(Debug,Default)]
pub struct RFunctionWriter{
    functions: Vec<RFunction>,
    defintions: String,
    body:  String,
}

impl RFunctionWriter{
    pub fn create_function(
        &mut self,
        name: String,
        is_method: bool,
        parameters: FunctionParameters,
        return_expr_type: String)
    {
        let func = RFunction::new(name, is_method, parameters, return_expr_type);
        self.functions.push(func);
    }

    fn mangle(function : &RFunction) -> String{
        let mut mangled_function = String::from("_Z");
        if function.is_method == true{
            mangled_function.push('T');
        }
        else {
            mangled_function.push('F');
        }
        let fn_name = &function.name;
        let fn_argc = *(&function.parameters.len());
        let fn_first_param_type = &function.parameters[0].1;
        let mut fn_label = String::new();
        let fn_return_expr = &function.return_expr_type;

        let is_phi_fn = *&function.parameters.first().map_or(true, |first| {
            *&function.parameters.iter().all(|t| t.1 == first.1)
        });

        if is_phi_fn == true{
            write!(fn_label,"phi").unwrap();
        }
        else{
            write!(fn_label,"sgm").unwrap();
        }
        write!(mangled_function,"{fn_name}_ac{fn_argc}_{fn_first_param_type}_{fn_label}_{fn_return_expr}").unwrap();

        mangled_function
    }


    pub fn write_functions(&mut self) {
        for func in self.functions.iter(){
            let cfn = Self::generate_cfunc(func);
            write!(self.defintions,"{cfn}").unwrap();
        }
    }

    fn generate_cfunc(func: &RFunction) -> String{
        let mut cfunc = String::new();
        let mangled_cfn_name = Self::mangle(func);

        let fn_ret_type = &func.return_expr_type;

        write!(cfunc,"{fn_ret_type} {mangled_cfn_name}(").unwrap();

        for (i,params) in func.parameters.iter().enumerate(){
            let pname = &params.0;
            let ptype = &params.1;

            if i > 0{
                write!(cfunc, ", ").unwrap();
            }
            write!(cfunc,"{ptype} {pname}").unwrap();
        }

        writeln!(cfunc,"); ").unwrap();
        cfunc
    }

}


#[derive(Debug,Default)]
pub struct RTable{
    name: String,
}

#[derive(Debug,Default)]
pub struct RTableWriter{
    tables: Vec<RTable>,
    table_defintions: String
}

impl RTableWriter{
    pub fn create_rtable(&mut self,name: &str){
        let mut rt = RTable{name: name.to_owned()};
        self.tables.push(rt);
    }

    pub fn write_rtable(&mut self){
        self.generate_rtable_unit_struct();
        self.generate_rtable_unit_create();
        self.generate_rtable_unit_create_with();
        self.generate_rtable_unit_init_with();
        self.generate_rtable_unit_delete();
        self.generate_rtable();
        self.generate_rtable_iterator();
    }

    fn generate_rtable_unit_struct(&mut self){
        writeln!(self.table_defintions,"typedef struct{{
            void* data;
        }}RTableUnit;").unwrap();
    }

    fn generate_rtable_unit_create(&mut self){
        writeln!(self.table_defintions,"static inline
            RTableUnit* newRTableUnit(){{
            RTableUnit* rtunit = malloc(sizeof(RTableUnit));
            return rtunit;
        }}").unwrap();
    }

    fn generate_rtable_unit_create_with(&mut self){
        writeln!(self.table_defintions,"static inline
            RTableUnit* newRTableUnitWith(void* data){{
            RTableUnit* rtunit = newRTableUnit();
            rtunit->data = data;
            return rtunit;
        }}").unwrap();
    }

    fn generate_rtable_unit_init_with(&mut self){
        writeln!(self.table_defintions,"static inline
            void initRTableUnitWith(RTableUnit* rtunit,void* data){{
            rtunit->data = data;
        }}").unwrap();
    }

     fn generate_rtable_unit_delete(&mut self){
        writeln!(self.table_defintions,"static inline
            void deleteRTableUnit(RTableUnit** rtunit){{
            free(*rtunit);
            *rtunit = NULL;
        }}").unwrap();
    }

    fn generate_rtable(&mut self){
        writeln!(self.table_defintions,"rutil_monomorphized_vector(RTableUnit*,RtableUnitPtr)
            typedef rutil_vector_RtableUnitPtr RTable; ").unwrap();
    }

    fn generate_rtable_iterator(&mut self){
        writeln!(self.table_defintions,"#define rutil_rtable_for_each(rt,itr)   \\
                RTableUnit** begin = vectorFront_RtableUnitPtr((rt));   \\
                RTableUnit** end = vectorBack_RtableUnitPtr((rt));      \\
                for(RTableUnit** itr = begin; itr != end ; ++itr)       \\
        ").unwrap();
    }

}
