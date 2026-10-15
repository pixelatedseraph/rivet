#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>
#include <stddef.h>
#include "util.h"

/*

#define rutil_monomorphized_map(K,V)    \
    rutil_monomorphized_vector(V,V)     \
    typedef rutil_vector_v Bucket ;     \
    const default_map_size = 30000;      \
    typedef struct{                     \
        Buckets* HashTable[default_map_size];   \
    },ap_K_V;       \

    Map_K_V map_k_v_create(){
        Map_K_V mp = {};
        rutil_for_each_in_c_arr(idx,mp.HashTable){  \
        mp.HashTable[idx] = newVector_int();    
        }
    }

    void insert_k_v(Self,K key,V value){
        u64 index = rutil_hash(key, strlen(key)) % TableSize;
        pushBack_int(Self.HashTable[index],value);
    }
    
    V at_k_v(Self,K key){
        u64 index = rutil_hash(key, strlen(key)) % TableSize;
        getAtIndex_int(Self.HashTable[index],0)
    }
*/
static inline u64 rutil_hash(const void *key, usize len){
    const u8 *bytes = key;

    u64 hash = UINT64_C(14695981039346656037);

    for (usize i = 0; i < len; ++i) {
        hash ^= bytes[i];
        hash *= UINT64_C(1099511628211);
    }
    return hash;
}

#define TableSize 30000

rutil_monomorphized_vector(int, int)

typedef struct{
    rutil_vector_int* HashTable[TableSize];
}Map;


int main(){
    Map mp = {};
    rutil_for_each_in_c_arr(idx,mp.HashTable){
        mp.HashTable[idx] = newVector_int();
    }

    char* students[] = {
        "John",
        "Kimika",
        "LarpLarp",
        "Mazeed",
        "Faizan",
        "Afnan"
    };

    rutil_for_each_in_c_arr(idx,students){
        u64 index = rutil_hash(students[idx], strlen(students[idx])) % TableSize;
        int value = (idx + 1) * 100;
        pushBack_int(mp.HashTable[index],value);
    }
    int val = getAtIndex_int(mp.HashTable[rutil_hash("Afnan", strlen("Afnan")) % TableSize],0);

    printf("%d ",val);

}

/*
int main(){

    char* name1 = "Maze";
    char* name2 = "Welcome";

    HashTable[rutil_hash(name1,strlen(name1)) % TableSize] = 123;
    HashTable[rutil_hash(name2,strlen(name2)) % TableSize] = 456;

    printf("%d ",HashTable[rutil_hash(name1,strlen(name1)) % TableSize]);
    printf("%d ",HashTable[rutil_hash(name2,strlen(name2)) % TableSize]);

    return 0;
}*/


/*
int main(){

    int key = 42;

    uint64_t h1 = rutil_hash(&key, sizeof key);

    const char *str = "hello";
    uint64_t h2 = rutil_hash(str, strlen(str));

    printf("%llu\n", (ull)h1);
    printf("%llu\n", (ull)h2);


    return 0;
}*/