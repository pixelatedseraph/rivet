#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>

typedef struct {
    void* x;
}pointer;

pointer* create_pointer(void* x){
    pointer* ptr = malloc(sizeof(pointer));
    ptr->x = x;
    return ptr;
}


typedef struct{
    int (*add)  (int,int);
    int  (*sub)  (int,int);
}fn_t;

int add_impl(int x,int y){
    return x + y;
}

int sub_impl(int x,int y){
    return x - y;
}



int main(){
    fn_t fn = {.add = add_impl,.sub = sub_impl};

    int(*x)(int,int) = fn.add;


    pointer* p = create_pointer(x);


    if((uint64_t)x != (uint64_t)p->x){
        fprintf(stderr, "Addresses arent equal\nStruct Member: %p\nVariable Member: %p\n",p->x,x);
        exit(EXIT_FAILURE);
    }

    printf("Var: %p Struct: %p\n",x,p->x);

    printf("%d\n",((int (*)(int,int))p->x)(10,2));
    
    return 0;
}