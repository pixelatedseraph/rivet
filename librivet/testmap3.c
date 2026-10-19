#include<stdio.h>
        #include<stdlib.h>
        #include<string.h>
        #include<stdbool.h>
        #include<stdint.h>
        #include"util.h"
        
typedef struct{ rutil_string* key; rutil_string* val;}pair_rutil_stringptr_rutil_stringptr;  

        rutil_monomorphized_vector(pair_rutil_stringptr_rutil_stringptr,pair_rutil_stringptr_rutil_stringptr)

        typedef rutil_vector_pair_rutil_stringptr_rutil_stringptr bucket;
        #define default_map_size 30000

        typedef struct{
            bucket* hashtable[default_map_size];
        }map_rutil_stringptr_rutil_stringptr; 
map_rutil_stringptr_rutil_stringptr map_rutil_stringptr_rutil_stringptr_create(){
            map_rutil_stringptr_rutil_stringptr mp = {};
            rutil_for_each_in_c_arr(idx,mp.hashtable){
                mp.hashtable[idx] = newVector_pair_rutil_stringptr_rutil_stringptr();
            }
            return mp;
        }
void map_rutil_stringptr_rutil_stringptr_put(map_rutil_stringptr_rutil_stringptr* self,rutil_string* key,rutil_string* val){
            u64 index = rutil_hash(key->vec_chars->data, strlen(key->vec_chars->data)) % default_map_size;
            pair_rutil_stringptr_rutil_stringptr pair = {.key = key,.val = val};
            pushBack_pair_rutil_stringptr_rutil_stringptr(self->hashtable[index],pair);
        }
int rutil_cmprator_rstring(const void* lhs,const void* rhs){
            pair_rutil_stringptr_rutil_stringptr _lhs = *(pair_rutil_stringptr_rutil_stringptr*)lhs;
            pair_rutil_stringptr_rutil_stringptr _rhs = *(pair_rutil_stringptr_rutil_stringptr*)rhs;

            rutil_string* _a = _lhs.key;
            rutil_string* _b = _rhs.key;

            return rutil_string_cmp(_a,_b);
        }

rutil_string* map_rutil_stringptr_rutil_stringptr_get(map_rutil_stringptr_rutil_stringptr* self,rutil_string* key){
            u64 index =  rutil_hash(key->vec_chars->data, strlen(key->vec_chars->data)) % default_map_size;

            bucket* _bucket = self->hashtable[index];

            if(_bucket->length == 1){
                pair_rutil_stringptr_rutil_stringptr pair = getAtIndex_pair_rutil_stringptr_rutil_stringptr(_bucket,0);
                return pair.val;
            }
            else{
                sortVector_pair_rutil_stringptr_rutil_stringptr(_bucket,rutil_cmprator_rstring);
                rutil_vec_for_each(idx, _bucket){
                    if(rutil_string_cmp(_bucket->data[idx].key,key) == 0){
                        return _bucket->data[idx].val; 
                    }
                }
                return NULL;
            }
        }
bool map_rutil_stringptr_rutil_stringptr_contains_key(map_rutil_stringptr_rutil_stringptr* self,rutil_string* key){
            u64 index =  rutil_hash(key->vec_chars->data, strlen(key->vec_chars->data)) % default_map_size;
            bucket* _bucket = self->hashtable[index];
            if(_bucket -> length == 1){
                pair_rutil_stringptr_rutil_stringptr pair = getAtIndex_pair_rutil_stringptr_rutil_stringptr(_bucket,0);
                if(rutil_string_eq(pair.key,key)){
                    return true;
                }
            }
            else{
                sortVector_pair_rutil_stringptr_rutil_stringptr(_bucket,rutil_cmprator_rstring);
                rutil_vec_for_each(idx, _bucket){
                    if(rutil_string_cmp(_bucket->data[idx].key,key) == 0){
                        return true; 
                    }
                }
                return false;
            }
    }
usize map_rutil_stringptr_rutil_stringptr_length(map_rutil_stringptr_rutil_stringptr* self){
            usize map_length = 0;
            usize array_length = rutil_array_length(self->hashtable);
            for(usize idx = 0; idx < array_length; ++idx){
                map_length += self->hashtable[idx]->length;
            }
            return map_length;
        }
bool map_rutil_stringptr_rutil_stringptr_is_empty(map_rutil_stringptr_rutil_stringptr* self){
            usize map_length = map_rutil_stringptr_rutil_stringptr_length(self);
            if(map_length == 0UL)
                return true;
            return false;
        }
bool map_rutil_stringptr_rutil_stringptr_clear(map_rutil_stringptr_rutil_stringptr* self){
            usize array_length = rutil_array_length(self->hashtable);
            for(usize idx = 0; idx < array_length; ++idx){
                clearVector_pair_rutil_stringptr_rutil_stringptr(self->hashtable[idx]);
            }
        }
void map_rutil_stringptr_rutil_stringptr_remove(map_rutil_stringptr_rutil_stringptr* self,rutil_string* key){
            u64 index = rutil_hash(key->vec_chars->data, strlen(key->vec_chars->data)) % default_map_size;            

            bucket* _bucket = self->hashtable[index];
            if(_bucket -> length == 1){
                (void)pop_pair_rutil_stringptr_rutil_stringptr(_bucket,0);
                return;
            }

            else{
                rutil_vec_for_each(idx,_bucket){
                    u64 index = 0;
            if(rutil_string_cmp(_bucket->data[idx].key,key) == 0){
                pop_pair_rutil_stringptr_rutil_stringptr(_bucket,index);
                return;
            }
                    index++;
                }
            }
        }
void map_rutil_stringptr_rutil_stringptr_print(map_rutil_stringptr_rutil_stringptr* self){
            usize array_length = rutil_array_length(self->hashtable);
            printf("%c ",'{');
            for(usize idx = 0; idx < array_length; ++idx){
                bucket* _bucket = self->hashtable[idx];
                rutil_vec_for_each(idx,_bucket){
                    pair_rutil_stringptr_rutil_stringptr pair = getAtIndex_pair_rutil_stringptr_rutil_stringptr(_bucket,idx);
                    printf("%s: %s ",pair.key->vec_chars->data,pair.val->vec_chars->data);
                }
            }
            printf("%c",'}');
            printf("%c",'\n');
        }
void map_rutil_stringptr_rutil_stringptr_delete(map_rutil_stringptr_rutil_stringptr* self){
            usize array_length = rutil_array_length(self->hashtable);
            for(usize idx = 0; idx < array_length; ++idx){
                deleteVector_pair_rutil_stringptr_rutil_stringptr(&(self->hashtable[idx]));
            }
        }

int main(){

    map_rutil_stringptr_rutil_stringptr mp = map_rutil_stringptr_rutil_stringptr_create();

    rutil_string* str1 = rutil_string_from("key_1098");
    rutil_string* str2 = rutil_string_from("key_3272");
    rutil_string* str3 = rutil_string_from("key_5249");
    rutil_string* str4 = rutil_string_from("key_10847");
    rutil_string* str5 = rutil_string_from("key_12184");

    rutil_string* str6 = rutil_string_from("mazeed");
    rutil_string* str7 = rutil_string_from("afnan");
    rutil_string* str8 = rutil_string_from("lizzie");
    rutil_string* str9 = rutil_string_from("gloria");
    rutil_string* str10 = rutil_string_from("sabrina");

    map_rutil_stringptr_rutil_stringptr_put(&mp,str1,str6);
    map_rutil_stringptr_rutil_stringptr_put(&mp,str2,str7);
    map_rutil_stringptr_rutil_stringptr_put(&mp,str3,str8);
    map_rutil_stringptr_rutil_stringptr_put(&mp,str4,str9);
    map_rutil_stringptr_rutil_stringptr_put(&mp,str5,str10);

    printf("%s\n",map_rutil_stringptr_rutil_stringptr_get(&mp,str1)->vec_chars->data);
    printf("%s\n",map_rutil_stringptr_rutil_stringptr_get(&mp,str2)->vec_chars->data);
    printf("%s\n",map_rutil_stringptr_rutil_stringptr_get(&mp,str3)->vec_chars->data);
    printf("%s\n",map_rutil_stringptr_rutil_stringptr_get(&mp,str4)->vec_chars->data);
    printf("%s\n",map_rutil_stringptr_rutil_stringptr_get(&mp,str5)->vec_chars->data);

    printf("%lu\n",map_rutil_stringptr_rutil_stringptr_length(&mp));

    map_rutil_stringptr_rutil_stringptr_print(&mp);

    map_rutil_stringptr_rutil_stringptr_delete(&mp);
    return 0;
}