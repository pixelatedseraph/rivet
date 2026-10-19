#include<stdio.h>
        #include<stdlib.h>
        #include<string.h>
        #include<stdbool.h>
        #include<stdint.h>
        #include"util.h"
        
typedef struct{ 
    rutil_string* key;
    rutil_string* val;
}pair_rstr_rstr;  

        rutil_monomorphized_vector(pair_rstr_rstr,pair_rstr_rstr)

        typedef rutil_vector_pair_rstr_rstr bucket;
        #define default_map_size 30000

        typedef struct{
            bucket* hashtable[default_map_size];
        }map_rstr_str; 
map_rstr_str map_rstr_str_create(){
            map_rstr_str mp = {};
            rutil_for_each_in_c_arr(idx,mp.hashtable){
                mp.hashtable[idx] = newVector_pair_rstr_rstr();
            }
            return mp;
        }
void map_rstr_str_put(map_rstr_str* self,rutil_string* key,rutil_string* val){
            u64 index = rutil_hash(key->vec_chars->data, strlen(key->vec_chars->data)) % default_map_size;

            //printf("String: %s,Index: %llu\n",key->vec_chars->data,index);

            pair_rstr_rstr pair = {.key = key,.val = val};
            pushBack_pair_rstr_rstr(self->hashtable[index],pair);
        }
int comparator_rstring(const void* lhs,const void* rhs){
    
            pair_rstr_rstr _lhs = *(pair_rstr_rstr*)lhs;
            pair_rstr_rstr _rhs = *(pair_rstr_rstr*)rhs;
        
            rutil_string* _a = _lhs.key;
            rutil_string* _b = _rhs.key;

            return rutil_string_cmp(_a, _b);
        }

rutil_string* map_rstr_str_get(map_rstr_str* self,rutil_string* key){
        u64 index =  rutil_hash(key->vec_chars->data, strlen(key->vec_chars->data)) % default_map_size;

        //printf("String: %s,Index: %llu\n",key->vec_chars->data,index);

        bucket* _bucket = self->hashtable[index];
            if(_bucket->length == 1){
                pair_rstr_rstr pair = getAtIndex_pair_rstr_rstr(_bucket,0);
                return pair.val;
            }
            else{
                sortVector_pair_rstr_rstr(_bucket,comparator_rstring);
                rutil_vec_for_each(idx, _bucket){
                    if(rutil_string_cmp(_bucket->data[idx].key,key) == 0){
                        return _bucket->data[idx].val;
                    }
                }
                return NULL;
            }
        }

int main(){
    /*
    map_rstr_str mp = map_rstr_str_create();

    rutil_string* ken =  rutil_string_from("ken");
    rutil_string* _unix =  rutil_string_from("unix");
    rutil_string* linus =  rutil_string_from("linus");
    rutil_string* _linux =  rutil_string_from("linux");
    rutil_string* dennis =  rutil_string_from("dennis");
    rutil_string* _c =  rutil_string_from("c");
    rutil_string* stallman =  rutil_string_from("stallman");
    rutil_string* _gcc =  rutil_string_from("gcc");
    rutil_string* bjarne =  rutil_string_from("bjarne");
    rutil_string* _cpp =  rutil_string_from("c++");


    map_rstr_str_put(&mp,ken,_unix);
    map_rstr_str_put(&mp,dennis,_c);
    map_rstr_str_put(&mp,linus,_linux);
    map_rstr_str_put(&mp,stallman,_gcc);
    map_rstr_str_put(&mp,bjarne,_cpp);



    rutil_string* _ken = map_rstr_str_get(&mp,ken);
    rutil_string* _dennis = map_rstr_str_get(&mp,dennis);
    rutil_string* _linus = map_rstr_str_get(&mp,linus);
    rutil_string* _bjarne = map_rstr_str_get(&mp,bjarne);
    rutil_string* _stallman = map_rstr_str_get(&mp,stallman);

    if(_dennis == NULL){
        fprintf(stderr,"%s","dennis's value not found");
        exit(EXIT_FAILURE);
    }
    if(_linus == NULL){
        fprintf(stderr,"%s","linus's value not found");
        exit(EXIT_FAILURE);
    }
    if(_bjarne== NULL){
        fprintf(stderr,"%s","_bjarne's value not found");
        exit(EXIT_FAILURE);
    }
    if(_stallman == NULL){
        fprintf(stderr,"%s","_stallman's value not found");
        exit(EXIT_FAILURE);
    }

    if(_ken == NULL){
        fprintf(stderr,"%s","ken's value not found");
        exit(EXIT_FAILURE);
    }

    printf("%s %s %s %s %s",_dennis->vec_chars->data,_bjarne->vec_chars->data,_stallman->vec_chars->data,_linus->vec_chars->data,_ken->vec_chars->data);
*/

    map_rstr_str mp = map_rstr_str_create();
    
    rutil_string* str1 = rutil_string_from("key_1098");
    rutil_string* str2 = rutil_string_from("key_3272");
    rutil_string* str3 = rutil_string_from("key_5249");
    rutil_string* str4 = rutil_string_from("key_10847");
    rutil_string* str5 = rutil_string_from("key_12184");

    rutil_string* str6 = rutil_string_from("mazeed");
    rutil_string* str7 = rutil_string_from("faizan");
    rutil_string* str8 = rutil_string_from("lizzie");
    rutil_string* str9 = rutil_string_from("gloria");
    rutil_string* str10 = rutil_string_from("sabrina");

    map_rstr_str_put(&mp,str1,str6);
    map_rstr_str_put(&mp,str2,str7);
    map_rstr_str_put(&mp,str3,str8);
    map_rstr_str_put(&mp,str4,str9);
    map_rstr_str_put(&mp,str5,str10);


    printf("%s\n",map_rstr_str_get(&mp,str1)->vec_chars->data);
    printf("%s\n",map_rstr_str_get(&mp,str2)->vec_chars->data);
    printf("%s\n",map_rstr_str_get(&mp,str3)->vec_chars->data);
    printf("%s\n",map_rstr_str_get(&mp,str4)->vec_chars->data);
    printf("%s\n",map_rstr_str_get(&mp,str5)->vec_chars->data);

    return 0;
}

