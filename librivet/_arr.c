#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>

typedef uint64_t usize;

typedef struct{
    int data[100];
}arr_t;

arr_t newArray(){
    arr_t arr = {};
    return arr;
}
#define rutil_array_length(array)         \
    sizeof((array))/sizeof((array[0])) 
#define rutil_for_each_in_c_arr(idx,c_arr) for(usize idx = 0 ; idx < rutil_array_length(c_arr);++idx)


int main(){
    arr_t arr = newArray();
    arr.data[0] = 1;
    arr.data[1] = 2;
    arr.data[2] = 3;
    arr.data[3] = 4;
    arr.data[4] = 5;
    rutil_for_each_in_c_arr(idx,arr.data){
        printf("%d ",arr.data[idx]);
    }

    return 0;
}