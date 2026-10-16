// use std::collections::HashMap;
// use std::any::type_name;
// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::fmt::Write;


// fn emit(out: &mut String,code: &str){
//     out.push_str(code);
// }

// fn emit_fmt(out: &mut String,args: std::fmt::Arguments<'_>){
//     out.write_fmt(args).expect("writing to String failed");
// }


// #[derive(Debug,Default)]
// pub struct CodeGenerator{
//     pub source: String,
// }


// /* 
// contains_key(key)	Check whether a key exists	contains_key(&K) -> bool
// remove(key)	Delete an entry	remove(&K)
// len()	Get the number of entries	len() -> usize
// is_empty()	Check whether the map is empty	is_empty() -> bool
// clear()	Remove all entries	clear()
// collison_detection()
// capacity()	Return current storage capacity
// reserve(n)	Ensure space for additional entries
// get_or_insert(key, value)	Insert only when absent
// iter()	Iterate over key-value pairs
// keys()	Iterate over keys
// values()	Iterate over values
// retain(fn)	Keep entries satisfying a predicate
// */

// type RMapTypeParam = (String,String);


// #[derive(Debug,Default)]
// pub struct RMap{
//     pub map_name:  String, //variable name
//     pub map_type_name: String, //monomorphized type
//     pub map_type_params: RMapTypeParam, //
//     pub map_type_specialized_code: String, //monomorphized function definitions
//     pub map_usage_code: String, //calling  function definitions with varibale names 
// }

// //holds info all the maps that were registered
// #[derive(Debug,Default)]
// pub struct RMapMetadata{
//     pub map_meta_all_maps: Vec<RMap>,   //holds map instances(variables)
//     pub map_meta_key_value_pair: Vec<RMapTypeParam>, //holds all the monomorphized map types & methods
// }

// impl CodeGenerator{
//     pub fn headers(&mut self) -> String{
//         let _headers = "#include<stdio.h>\n#include<stdlib.h>\n#include<string.h>\n#include<stdbool.h>\n#include<stdint.h>\n#include\"util.h\"\n".to_string();
//         //self.source.push_str(headers);
//         _headers
//     }

//     pub fn mangle_ptr(_type: &str) -> String{
//         let mut mangled_type = String::new();
//         for ch in _type.chars(){
//             if ch == '*'{
//                 mangled_type.push_str("ptr");
//             }
//             else{
//                 mangled_type.push(ch);
//             }
//         }
//         mangled_type
//     }

//     pub fn cmain_begin(&mut self) -> String{
//         "int main(){\n".to_string()
//     }

//     pub fn cmain_end(&mut self) -> String{
//         "return 0;}\n".to_string()
//     }
// }

// impl RMap{
//     fn hash_index_code(key: &str) -> String {
//         if key == "char*" || key == "const char*" {
//             "rutil_hash(key, strlen(key))".to_string()
//         }
//         else {
//             "rutil_hash(&key, sizeof(key))".to_string()
//         }
//     }

//     pub fn _define(&mut self,rmapmeta: &mut RMapMetadata,key: &str,val: &str){
//         let kv_pair = (String::from(key),String::from(val)); 

//         rmapmeta.map_meta_key_value_pair.push(kv_pair);
//         let mkey = CodeGenerator::mangle_ptr(key);
//         let mval = CodeGenerator::mangle_ptr(val);

//         let 


//     }

//     pub fn define(&mut self,rmapmeta: &mut RMapMetadata,key: &str,val: &str){

//         let kv_pair = (String::from(key),String::from(val)); 

//         rmapmeta.map_meta_key_value_pair.push(kv_pair);

//         let mkey = CodeGenerator::mangle_ptr(key);
//         let mval = CodeGenerator::mangle_ptr(val);

        
//         let mut map_generator_body = String::new();

//         map_generator_body.push_str(format!("   rutil_monomorphized_vector({},{})  \n",val,mval).as_str());
        
//         map_generator_body.push_str(format!("   typedef rutil_vector_{} Bucket; \n",mval).as_str());
//         map_generator_body.push_str(format!("   #define default_map_size {} \n",30000).as_str());

//         map_generator_body.push_str(format!("   typedef struct{{  \n       Bucket* HashTable[default_map_size];  \n     }}map_{}_{};  \n",mkey,mval).as_str());

//         self.map_type_name = format!("map_{}_{}",mkey,mval);

//         map_generator_body.push_str(format!("   {} {}_create(){{  \n",self.map_type_name,self.map_type_name).as_str());
//         map_generator_body.push_str(format!("       {} mp = {{}};  \n",self.map_type_name).as_str());
//         map_generator_body.push_str(format!("       rutil_for_each_in_c_arr(idx,mp.HashTable){{  \n").as_str());
//         map_generator_body.push_str(format!("           mp.HashTable[idx] = newVector_{}();  \n",mval).as_str());
//         map_generator_body.push_str(format!("       }} \n").as_str());
//         map_generator_body.push_str(format!("    return mp; \n").as_str());
//         map_generator_body.push_str(format!("   }} \n").as_str());


//         map_generator_body.push_str(format!("   void {}_insert({}* self,{} key, {} val){{  \n",self.map_type_name,self.map_type_name,key,val).as_str());
//         if key == "char*" || key == "const char*"{
//             map_generator_body.push_str(format!("       u64 index = rutil_hash(key,strlen(key)) % default_map_size;\n").as_str());
//         }
//         else{
//             map_generator_body.push_str(format!("       u64 index = rutil_hash(key,sizeof(key)) % default_map_size;\n").as_str());
//         }
//         map_generator_body.push_str(format!("       pushBack_{}(self->HashTable[index],val);\n",mval).as_str());
//         map_generator_body.push_str("   }\n");

//         map_generator_body.push_str(format!("   {} {}_at({}* self,{} key){{  \n",val,self.map_type_name,self.map_type_name,key).as_str());
//         if key == "char*" || key == "const char*"{
//             map_generator_body.push_str(format!("       u64 index = rutil_hash(key,strlen(key)) % default_map_size;\n").as_str());
//         }
//         else{
//             map_generator_body.push_str(format!("       u64 index = rutil_hash(key,sizeof(key)) % default_map_size;\n").as_str());
//         }

//         map_generator_body.push_str(format!("       return getAtIndex_{}(self->HashTable[index],0);\n",mval).as_str());
//         map_generator_body.push_str("   }\n");

//         self.map_type_params.0 = key.to_string();
//         self.map_type_params.1 = val.to_string();


//         self.map_type_specialized_code.push_str(map_generator_body.as_str());
//     }

//     //creates a map var_name
//     pub fn create(&mut self,var_name: &str,rmapmeta: &mut RMapMetadata,key: &str,val: &str){
//         let kv_pair = (String::from(key),String::from(val)); 
//         if rmapmeta.map_meta_key_value_pair.contains(&kv_pair) == false{
//             self.define(rmapmeta,key, val);
//             rmapmeta.map_meta_key_value_pair.push(kv_pair);
//         }

//         self.map_usage_code.push_str(format!("{} {} = {}_create();\n",self.map_type_name,var_name,self.map_type_name).as_str());
//         self.map_name.push_str(var_name);
//     }

//     pub fn insert(&mut self,key: &str,val: &str){
//         let funcall = format!("{}_insert(&{},{},{});\n",self.map_type_name,self.map_name,key,val);
//         self.map_usage_code.push_str(&funcall);
//     }

//     pub fn at(&mut self,key: &str){
//         static var_register: AtomicUsize = AtomicUsize::new(0);

//         let funcall = format!("{} vr{:?} = {}_at(&{},{});\n",self.map_type_params.1,var_register,self.map_type_name,self.map_name,key);
//         self.map_usage_code.push_str(&funcall);
//         var_register.fetch_add(1, Ordering::Relaxed) + 1;
//     }


// }