/*
  *  Author: Mazeed .A
  *  SPDX identifier: AGPL v3
  *  About: Implementation of Revo's Table Construct as specified by the language docuementation
*/

//RTable is implemeted by holding an vector of RTableUnit* which themselves hold a void* (Type Erasure)
//TypeCasts are inserted accordingly during CodeGen phase

//Supports : Generic<T:Any> Sequence(List),SubTables,Methods(FunctionPointers)
//Upcoming : Map interface,Cycle Detection(WeakRefrences),Polished API interface

#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include "util.h"

typedef struct{
    void* data;
}RTableUnit;

static inline
RTableUnit* newRTableUnit(){
    RTableUnit* rtunit = malloc(sizeof(RTableUnit));
    return rtunit;
}

static inline
RTableUnit* newRTableUnitWith(void* data){
    RTableUnit* rtunit = newRTableUnit();
    rtunit->data = data;
    return rtunit;
}

static inline
void deleteRTableUnit(RTableUnit** rtunit){
    free(*rtunit);
    *rtunit = NULL;
}

static inline
void initRTableUnitWith(RTableUnit* rtunit,void* data){
    rtunit->data = data;
}

//RTable owns RTableUnit instances and is responisble for cleanup of those objects
//all allocations of storage objects must be made prior to calling the RTableAPI,
//RTable only refreneces the allocated data and must not allocate for an object by itself
//RTable must rely on external Object Constructor and destructor(no implicit allocations)

typedef RTableUnit* RtableUnitPtr ;

rutil_monomorphized_vector(RtableUnitPtr,RT)

typedef rutil_vector_RtableUnitPtr RTable; 

#define rutil_rtable_for_each(rt,itr)                        \
    RTableUnit** begin = vectorFront_RT((rt));               \
    RTableUnit** end = vectorBack_RT((rt));                  \
    for(RTableUnit** itr = begin; itr != end ; ++itr)        \

typedef struct{
    i32  (*add)  (i32,i32);
    i32  (*sub)  (i32,i32);
}fn_t;

i32 add_impl(i32 x,i32 y){
    return x + y;
}

i32 sub_impl(i32 x,i32 y){
    return x - y;
}

i32 main(){
    fn_t fn = {.add = add_impl,.sub =sub_impl};

    const static char* months[] = {
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "Decemeber",
    };

    RTable* rt = newVector_RT();

    rutil_for_each_in_c_arr(idx,months){
        pushBack_RT(rt, newRTableUnitWith(months[idx]));
    }

    rutil_vec_for_each(idx,rt){
        printf("%s\n",(char*)rt->data[idx]->data);
    }

    pushBack_RT(rt,newRTableUnitWith(fn.add));
    pushBack_RT(rt,newRTableUnitWith(fn.sub));

    int foo = ((int (*)(int,int))getAtIndex_RT(rt, 12)->data)(100,2);
    int bar = ((int (*)(int,int))getAtIndex_RT(rt, 13)->data)(200,foo);
    
    printf("%d\n",foo);

    //subtable
    RTable* subrt = newVector_RT();
    const double fnums[] = {
        3.154,
        2.322,
        8.377,
        999.129,
    };

    rutil_for_each_in_c_arr(idx, fnums){
        pushBack_RT(subrt, newRTableUnitWith(&fnums[idx]));
    }
    
    //inserting subtable (subrt)  in table (rt) 
    pushBack_RT(rt,newRTableUnitWith(subrt));

    RTableUnit* subtableunit = *vectorGetLast_RT(rt);

    RTable* srt = (RTable*)subtableunit->data;

    rutil_rtable_for_each(srt, itr){
        printf("%.2f ",*(double*)(*itr)->data);
    }



    deleteVectorByDestructor_RT(&subrt,deleteRTableUnit);
    deleteVectorByDestructor_RT(&rt,deleteRTableUnit);

    return 0;
}

/*
i32 main(){

    rutil_vector_RtableUnitPtr* RTable = newVector_RT();

    int x = 10;
    int y = 20;
    int z = 30;

    char* str1 = "Hello";
    char* str2 = "World";

    pushBack_RT(RTable,newRTableUnitWith(&x));
    pushBack_RT(RTable,newRTableUnitWith(&y));
    pushBack_RT(RTable,newRTableUnitWith(&z));

    pushBack_RT(RTable,newRTableUnitWith(str1));
    pushBack_RT(RTable, newRTableUnitWith(str2));

    for(usize i = 0; i < 3; ++i)
        printf("%d ",*(int*)getAtIndex_RT(RTable, i)->data);

    for(usize i = 3; i < 5; ++i)
        printf("%s ",(char*)getAtIndex_RT(RTable, i)->data);
    

    return 0;
}*/