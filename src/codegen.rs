use std::collections::HashMap;
use std::any::type_name;
#[derive(Debug,Default)]
pub struct CodeGenerator{
    pub source: String,
    pub rmap  : RMap,
}
type RMapTypeParam = (String,String);

#[derive(Debug,Default)]
pub struct RMap{
    pub map_combo: Vec<RMapTypeParam>
}

impl CodeGenerator{
    pub fn headers(&mut self){
        let headers = "#include<stdio.h>\n#include<stdlib.h>\n#include<string.h>\n#include<stdbol.h>\n#include<stdint.h>#include\"util.h\"";
        self.source.push_str(headers);
    }
    
    pub fn map_define(&mut self,key: &str,val: &str){
    
        //let mut mapCode = String::new();

        self.rmap.map_combo.push((key.to_owned(),val.to_owned()));

        let mkey = Self::mangle_ptr(key);
        let mval = Self::mangle_ptr(val);

        
        let mut map_generator_body = String::new();

        map_generator_body.push_str(format!("   rutil_monomorphized_vector({},{})  \n",val,mval).as_str());
        
        map_generator_body.push_str(format!("   typedef rutil_vector_{} Bucket; \n",mval).as_str());
        map_generator_body.push_str(format!("   const u64 default_map_size = {}; \n",30000).as_str());

        map_generator_body.push_str(format!("   typedef struct{{  \n       Bucket* HashTable[default_map_size];  \n     }}map_{}_{};  \n",mkey,mval).as_str());

        let mono_map = format!("map_{}_{}",mkey,mval);

        map_generator_body.push_str(format!("   {} {}_create(){{  \n",mono_map,mono_map).as_str());
        map_generator_body.push_str(format!("       {} mp = {{}};  \n",mono_map).as_str());
        map_generator_body.push_str(format!("       rutil_for_each_in_c_arr(idx,mp.HashTable){{  \n").as_str());
        map_generator_body.push_str(format!("           mp.HashTable[idx] = newVector_{}();  \n",mval).as_str());
        map_generator_body.push_str(format!("       }} \n").as_str());
        map_generator_body.push_str(format!("   }} \n").as_str());


        map_generator_body.push_str(format!("   void {}_insert({}* self,{} key, {} val){{  \n",mono_map,mono_map,key,val).as_str());
        if key == "char*" || key == "const char*"{
            map_generator_body.push_str(format!("       u64 index = rutil_hash(key,strlen(key)) % TableSize;\n").as_str());
        }
        else{
            map_generator_body.push_str(format!("       u64 index = rutil_hash(key,sizeof(key)) % TableSize;\n").as_str());
        }
        map_generator_body.push_str(format!("       pushBack_{}(self->HashTable[index],value);\n",mval).as_str());
        map_generator_body.push_str("   }\n");

        map_generator_body.push_str(format!("   {} {}_at({}* self,{} key, {} val){{  \n",val,mono_map,mono_map,key,val).as_str());
        if key == "char*" || key == "const char*"{
            map_generator_body.push_str(format!("       u64 index = rutil_hash(key,strlen(key)) % TableSize;\n").as_str());
        }
        else{
            map_generator_body.push_str(format!("       u64 index = rutil_hash(key,sizeof(key)) % TableSize;\n").as_str());
        }

        map_generator_body.push_str(format!("       getAtIndex_{}(self->HashTable[index],0);\n",mval).as_str());
        map_generator_body.push_str("   }\n");

        self.source.push_str(map_generator_body.as_str());
    }

    pub fn map_create(&mut self,key: &str,val: &str){
        if self.rmap.map_combo.contains(&(key.to_owned(),val.to_owned())) == false{
            self.map_define(key, val);
        }
        
    }



    pub fn mangle_ptr(_type: &str) -> String{
        let mut mangled_type = String::new();
        for ch in _type.chars(){
            if ch == '*'{
                mangled_type.push_str("ptr");
            }
            else{
                mangled_type.push(ch);
            }
        }
        mangled_type
    }
}