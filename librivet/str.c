#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "util.h"


typedef struct {
    rutil_string* key;
    rutil_string* val;
}kv_pair;

int cmprator_rstring(const void* lhs,const void* rhs){
        rutil_string* _a = *(rutil_string**)lhs;
        rutil_string* _b = *(rutil_string**)rhs;

        return rutil_string_cmp(_a,_b);
}

rutil_monomorphized_vector(rutil_string*,strings)

void rutil_reflect_string(rutil_string* string){
    printf("%s\n",string->vec_chars->data);
}

int main(){
/*
    rutil_string str = rutil_string_from("Maze");
    rutil_string_push_str(str, rutil_string_from("Jeff"));
    rutil_string_format(str, " %s: %d", "HAHA",13);
*/

    rutil_string* str1 = rutil_string_from("Mazeed");
    rutil_string* str2 = rutil_string_from("Faizan");
    rutil_string* str3 = rutil_string_from("Al Pacino");
    rutil_string* str4 = rutil_string_from("Robert DeNiro");
    rutil_string* str5 = rutil_string_from("Matthew Mconugahey");
    
    rutil_vector_strings* strings = newVector_strings();
    pushBack_strings(strings,str1);
    pushBack_strings(strings,str2);
    pushBack_strings(strings,str3);
    pushBack_strings(strings,str4);
    pushBack_strings(strings,str5);

    sortVector_strings(strings,cmprator_rstring);
    
    printVector_strings(strings,rutil_reflect_string);


    return 0;
}