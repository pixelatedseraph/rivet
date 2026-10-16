#include<stdio.h>
        #include<stdlib.h>
        #include<string.h>
        #include<stdbool.h>
        #include<stdint.h>
        #include"util.h"
        
typedef struct{ int key; int val;}pair_int_int;  

        rutil_monomorphized_vector(pair_int_int,pair_int_int)

        typedef rutil_vector_pair_int_int bucket;
        #define default_map_size 30000

        typedef struct{
            bucket* hashtable[default_map_size];
        }map_int_int; 
map_int_int map_int_int_create(){
            map_int_int mp = {};
            rutil_for_each_in_c_arr(idx,mp.hashtable){
                mp.hashtable[idx] = newVector_pair_int_int();
            }
            return mp;
        }
void map_int_int_put(map_int_int* self,int key,int val){
            u64 index = rutil_hash(&key, sizeof(key)) % default_map_size;
            pair_int_int pair = {.key = key,.val = val};
            pushBack_pair_int_int(self->hashtable[index],pair);
        }
int comparator(const void* lhs,const void* rhs){
    
            pair_int_int _lhs = *(pair_int_int*)lhs;
            pair_int_int _rhs = *(pair_int_int*)rhs;
        
            int _a = _lhs.val;
            int _b = _rhs.val;

            if(_a > _b)
                return 1;
            else if(_a < _b)
                return -1;
            else 
                return 0;
        }

int map_int_int_get(map_int_int* self,int key){
        u64 index =  rutil_hash(&key, sizeof(key)) % default_map_size;

        bucket* _bucket = self->hashtable[index];

            if(_bucket->length == 1){
                pair_int_int pair = getAtIndex_pair_int_int(_bucket,0);
                return pair.val;
            }
            else{
                sortVector_pair_int_int(_bucket,comparator);
                pair_int_int* begin = vectorFront_pair_int_int(_bucket);
                pair_int_int* end   = vectorBack_pair_int_int(_bucket);
                for(pair_int_int* i = begin ; i != end; ++i){
                    if(i->key == key){
                        return i->val;
                    }
                }
            }
        }

int main(){
    map_int_int mp = map_int_int_create();

    map_int_int_put(&mp,123,456);
    map_int_int_put(&mp,111,738);
    map_int_int_put(&mp,132,48382);

    int v1 = map_int_int_get(&mp,123);
    int v2 = map_int_int_get(&mp,111);
    int v3 = map_int_int_get(&mp,132);

    printf("%d %d %d",v1,v2,v3);

    return 0;
}

