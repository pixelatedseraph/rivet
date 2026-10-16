use std::fmt::Write;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn type_mangler(_type: &str) -> String{
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

#[derive(Debug,Default)]
pub struct CCodeWriter{
    pub csource: String
}

impl CCodeWriter{

    pub fn write_c_source(&mut self,mapwriter: &RMapWriter){
        self.generate_c_headers();
        self.append_rmap_definitions(mapwriter);
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

    fn append_rmap_definitions(&mut self,mapwriter:&RMapWriter){
        let rmap_definitions = &mapwriter.monomorphized_code;
        writeln!(self.csource,"{rmap_definitions}").unwrap();
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
        }

    }
    fn generate_rutil_map_struct(&mut self,key: &str,val: &str){
        let mkey = type_mangler(key);
        let mval= type_mangler(val);
        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);

        let default_map_size = 30_000; 
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
            u64 index = {hashing_strategy} % default_map_size;
            {pair_typename} pair = {{.key = key,.val = val}};
            pushBack_{pair_typename}(self->hashtable[index],pair);
        }}").unwrap();
    }

    fn generate_rutil_map_function_get(&mut self,key: &str,val: &str){
        let mkey = type_mangler(key);
        let mval= type_mangler(val);
        let hashing_strategy = Self::hash_index_code(key);
        let map_typename = RMap::write_map_type_name(key, val);
        let pair_typename = RMap::write_pair_type_name(key, val);
        let comparator = self.generate_comparator_int(key,val);

        writeln!(self.monomorphized_code,"{comparator}").unwrap();

        writeln!(self.monomorphized_code,"{val} {map_typename}_get({map_typename}* self,{key} key){{
            u64 index =  {hashing_strategy} % default_map_size;

            bucket* _bucket = self->hashtable[index];

            if(_bucket->length == 1){{
                {pair_typename} pair = getAtIndex_{pair_typename}(_bucket,0);
                return pair.val;
            }}
            else{{
                sortVector_{pair_typename}(_bucket,comparator);
                {pair_typename}* begin = vectorFront_{pair_typename}(_bucket);
                {pair_typename}* end   = vectorBack_{pair_typename}(_bucket);
                for({pair_typename}* i = begin ; i != end; ++i){{
                    if(i->key == key){{
                        return i->val;
                    }}
                }}
            }}
        }}").unwrap();
    }
    fn hash_index_code(key: &str) -> String {
        if key == "char*" || key == "const char*" {
             "rutil_hash(key, strlen(key))".to_string()
         }
         else {
             "rutil_hash(&key, sizeof(key))".to_string()
         }
     }

    fn generate_comparator_int(&self,key: &str,val: &str) -> String{
        let mut comparator = String::new();
        let pair_typename = RMap::write_pair_type_name(key, val);

        writeln!(comparator,"int comparator(const void* lhs,const void* rhs){{
    
            {pair_typename} _lhs = *({pair_typename}*)lhs;
            {pair_typename} _rhs = *({pair_typename}*)rhs;
        
            int _a = _lhs.val;
            int _b = _rhs.val;

            if(_a > _b)
                return 1;
            else if(_a < _b)
                return -1;
            else 
                return 0;
        }}").unwrap();

        comparator
    }

}