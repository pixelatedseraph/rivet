#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<stdbool.h>
#include<stdint.h>
#include"util.h"

   int compare_ints(const void* restrict lhs,const void* restrict rhs){
        int _lhs = *(int*)lhs;
        int _rhs = *(int*)rhs;
        if(_lhs  > _rhs){
            return 1;
        }
        else if (_lhs < _rhs){
            return -1;   
        }
        return 0;
   }

   #define rutil_monomorphized_KV_Pair(K,V) \
        typedef struct{                     \
            K key,                          \
            V Val,                          \
        }MapKeyValuePair_##MK_##MV;         \

   rutil_monomorphized_vector(MapKeyValuePair_##MK_##MV,HashBucket);  
   typedef rutil_vector_int Bucket; 
   #define default_map_size 30000 
   typedef struct{  
       Bucket* HashTable[default_map_size];  
     }map_charptr_int;  
   map_charptr_int map_charptr_int_create(){  
       map_charptr_int mp = {};  
       rutil_for_each_in_c_arr(idx,mp.HashTable){  
           mp.HashTable[idx] = newVector_int();  
       } 
    return mp; 
   } 
   void map_charptr_int_insert(map_charptr_int* self,char* key, int val){  
       u64 index = rutil_hash(key,strlen(key)) % default_map_size;
       pushBack_int(self->HashTable[index],val);
   }
   int map_charptr_int_at(map_charptr_int* self,char* key){  
       u64 index = rutil_hash(key,strlen(key)) % default_map_size;

       Bucket* bucket = self->HashTable[index];

       if (bucket->length == 1){
            return getAtIndex_int(self->HashTable[index],0);
       }
       else{
            sortVector_int(bucket,compare_ints)
            findVector_int(bucket, int *key, int (*comparator)(const void *, const void *))
       }
   }

int main(){
map_charptr_int TestMap = map_charptr_int_create();
map_charptr_int_insert(&TestMap,"Mazeed",123);
map_charptr_int_insert(&TestMap,"Faizan",456);
map_charptr_int_insert(&TestMap,"Afnan",789);
int vr0 = map_charptr_int_at(&TestMap,"Mazeed");
int vr1 = map_charptr_int_at(&TestMap,"Faizan");
int vr2 = map_charptr_int_at(&TestMap,"Afnan");

printf("%d %d %d",vr0,vr1,vr2);

return 0;}
