/*
  *  Author: Mazeed .A
  *  SPDX identifier: AGPL v3
  *  About: Runtime C utility layer for revo backend dump 
*/

//Fair Warning: 
//The following source code isnt for faint of heart as it is extremely macro heavy & confusing!

//upcoming types: rutil_map & rutil_string

#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdarg.h>
#include <stdbool.h>
#include <sys/types.h>


typedef uint8_t     u8;
typedef uint16_t    u16;
typedef uint32_t    u32;
typedef uint64_t    u64;
typedef size_t      usize;

typedef int8_t     i8;
typedef int16_t    i16;
typedef int32_t    i32;
typedef int64_t    i64;
typedef i64        isize ;


const u64 freed_ptr_address = 0xdeadbeef;

typedef enum{
    ArrayIndexOutOFBound,
    ArraySpaceLimitExceeded,
}rutil_error_codes;

#define mpanic

static inline 
char* rutil_error_code_to_str(rutil_error_codes ec){
    switch (ec) {
        case ArrayIndexOutOFBound: return "ArrayIndexOutOFBound";  break;
        case ArraySpaceLimitExceeded: return "ArraySpaceLimitExceeded"; break;
        default: return "UndocumentedError"; break;
    }
};

#define rutil_array_length(array)         \
    sizeof((array))/sizeof((array[0])) 

#define rutil_panic_handler(CatchableError){\
    fprintf(stderr,"%s:%d:\033[31m panic\033[0m: %s in %s\n",__FILE__, __LINE__,rutil_error_code_to_str(CatchableError),__func__);\
    exit(CatchableError);   \
}

#define rutil_for_each_in_c_arr(idx,c_arr) for(usize idx = 0 ; idx < rutil_array_length(c_arr);++idx)

#define rutil_monomorphized_array(T) \
    typedef struct{              \
        T* data;                 \
        usize length;            \
        usize capacity;          \
    }rutil_array_##T;            \
                                 \
    static inline                                                                                               \
    rutil_array_##T* array_new_##T(usize capacity){                                                       \
        rutil_array_##T* array = malloc(sizeof(rutil_array_##T));       \
        array->data = malloc(sizeof(T) * capacity);                 \
        array->length = 0;              \
        array->capacity = capacity;     \
        return array;                   \
    }                                                                                                           \
    static inline                                                                                               \
    void mpanic array_push_##T(rutil_array_##T* self,usize element_count,...){                            \
        va_list ap;                                                                                             \
        if(self->length == self->capacity || ((self->capacity - self->length) < element_count)){                \
            rutil_panic_handler(ArraySpaceLimitExceeded)                                                        \
        }                                                                                                       \
        va_start(ap, element_count);                                                                            \
        for(usize i = 0; i < element_count; ++i){                                                               \
            self->data[self->length] =  va_arg(ap,T);                                                           \
            self->length++;                                                                                     \
        }                                                                                                       \
        va_end(ap);                                                                                             \
    }                                                                                                           \
    static inline                                                                                               \
    void array_print_##T(rutil_array_##T* self,void (*reflector)(T)){                                     \
            printf("[ ");                                                                                       \
        for(usize i = 0; i < self->length; ++i)                                                                 \
            reflector(self->data[i]);                                                                           \
        printf("]\n");                                                                                          \
    }                                                                                                           \
    static inline                                                                                               \
    void array_drop_##T(rutil_array_##T** self){                                                           \
        (*self)->length = 0;                                                                                       \
        (*self)->capacity = 0;  \
        free((*self)->data);                                                                                \
        (*self)->data = (T*)freed_ptr_address;  \
        free(*self);    \
        *self = (rutil_array_##T*)freed_ptr_address;    \
    }                                                                                                           \
    static inline                                                                                               \
    void array_reflection_##T(rutil_array_##T* self,void (*reflector)(T)){                                \
        printf("[ \n    data: ");                                                                               \
        array_print_##T(self,reflector);                                                                  \
        printf("    length: %zu,\n",self->length);                                                              \
        printf("    capacity: %zu,\n",self->capacity);                                                          \
        printf("]\n");                                                                                          \
    }                                                                                                           \
    static inline                                                                                               \
    void array_breflection_##T(rutil_array_##T* self){                                                    \
        printf("[ \n    data: %p,\n",self->data);                                                               \
        printf("    length: %zu,\n",self->length);                                                              \
        printf("    capacity: %zu,\n",self->capacity);                                                          \
        printf("]\n");                                                                                          \
    }                                                                                                           \
    static inline                                                                                               \
    T mpanic array_get_at_##T(rutil_array_##T* self,usize index){                                         \
        if(index >= self->length){                                                                              \
            rutil_panic_handler(ArrayIndexOutOFBound);                                                          \
            exit(ArrayIndexOutOFBound);                                                                         \
        }                                                                                                       \
        return self->data[index];                                                                               \
    }                                                                                                           \
    static inline                                                                                               \
    void mpanic array_set_at_##T(rutil_array_##T* self,usize index,T value){                              \
        if(index >= self->length){                                                                              \
            rutil_panic_handler(ArrayIndexOutOFBound);                                                          \
            exit(ArrayIndexOutOFBound);                                                                         \
        }                                                                                                       \
        self->data[index] = value;                                                                              \
    }                                                                                                           \
    static inline                                                                                               \
    void array_for_each_##T(rutil_array_##T* self,T (*map)(T)){   \
        for(usize i = 0; i < self->length; ++i){    \
            self->data[i] = map(self->data[i]); \
        }\
    }\
    static inline\
    rutil_array_##T* mpanic array_clone_##T(rutil_array_##T* self){    \
        rutil_array_##T* cloned = array_new_##T(self->capacity);\
        memcpy(cloned->data, self->data,self->length * sizeof (T));  \
        cloned->length = self->length;    \
        return cloned;  \
    }\


static inline
void rutil_reflect_print_i32(i32 value){
    printf("%d ",value);
}


#define rutil_vec_for_each(idx,vector) for(usize idx = 0; idx < vector->length ;++idx) 

/* Vector Type & API*/

#define rutil_monomorphized_vector(T,Type)  \
    typedef struct {        \
        T*  data;        \
        usize length;      \
        usize capacity;    \
        usize type_size;   \
    }rutil_vector_##T;         \
    \
    static inline rutil_vector_##T* newVector_##Type(){   \
        rutil_vector_##T* vector = malloc(sizeof(rutil_vector_##T));  \
        const usize defaultCapacity = 8;            \
        vector->type_size = sizeof(typeof(T));   \
        vector->data     = calloc(defaultCapacity,vector->type_size);\
        vector->length   = 0;    \
        vector->capacity = defaultCapacity;  \
        return vector;      \
    }   \
    \
    static inline rutil_vector_##T* reserveVectorCapacity_##Type(const usize capacity){   \
        rutil_vector_##T* vector = malloc(sizeof(rutil_vector_##T));    \
        \
        vector->type_size = sizeof(typeof(T));   \
        vector->data     = calloc(capacity,vector->type_size);   \
        vector->length   = 0;   \
        vector->capacity = capacity;    \
        return vector;  \
    }\
    static inline void shrinkVector_##Type(rutil_vector_##T* vector){  \
        vector->data = realloc(vector->data,vector->length * vector->type_size);   \
        vector->capacity = vector->length;   \
    }\
    static inline rutil_vector_##T* cloneVector_##Type(rutil_vector_##T* vector){    \
        rutil_vector_##T* clonedVector = reserveVectorCapacity_##Type(vector->capacity);   \
        memcpy(clonedVector->data,vector->data,vector->length * vector->type_size); \
        clonedVector->length = vector->length;  \
        return clonedVector;    \
    }   \
    static inline void pushBack_##Type(rutil_vector_##T* vector,T value){  \
        \
        while(vector->length >= vector->capacity){  \
            usize newCapacity = vector->capacity * 2;  \
            vector->data = realloc(vector->data,newCapacity * vector->type_size);   \
            vector->capacity = newCapacity; \
        }   \
        vector->data[vector->length] = value;   \
        vector->length++; \
    }\
    static inline void appendList_##Type(rutil_vector_##T* vector,T* list,usize length){ \
        usize listSize = sizeof(T) * length;   \
        while(((vector->capacity - vector->length) * sizeof(T)) < listSize){ \
            usize newCapacity = vector->capacity * 2;  \
            vector->data = realloc(vector->data,newCapacity * vector->type_size);   \
            vector->capacity = newCapacity; \
        }   \
        memcpy( \
            vector->data+vector->length,    \
            list,   \
            length * sizeof(T)  \
        );  \
        vector->length+=length; \
    }\
    static inline void push_##Type(rutil_vector_##T* vector, T value, usize index) {   \
        if(index > vector->length-1){   \
            fprintf(stderr,"[Panic]: %s","Vector Index Out of bound!\n");   \
            exit(EXIT_FAILURE); \
        }   \
        while (vector->length >= vector->capacity) {    \
            usize newCapacity = vector->capacity * 2;  \
            vector->data = realloc(vector->data, newCapacity * vector->type_size); \
            vector->capacity = newCapacity; \
        }   \
        memmove(&vector->data[index + 1], &vector->data[index],\
                (vector->length - index) * sizeof(T));  \
        vector->data[index] = value;    \
        vector->length++;\
    }\
    static inline void pushFront_##Type(rutil_vector_##T* vector,T value){   \
        push_##Type(vector,value,0);    \
    }\
    static inline T popBack_##Type(rutil_vector_##T* vector){   \
        T lastValue = vector->data[vector->length-1]; \
        vector->length--;   \
        return lastValue;   \
    }   \
    \
    static inline mpanic T getAtIndex_##Type(rutil_vector_##T* vector,usize index){     \
        if(index > vector->length-1){          \
            fprintf(stderr,"[Panic]: %s","Vector Index Out of bound!\n");   \
            exit(EXIT_FAILURE); \
        }   \
        return vector->data[index]; \
    }\
    static inline mpanic T pop_##Type(rutil_vector_##T* vector,usize index){   \
        T valueAtIndex = getAtIndex_##Type(vector,index);   \
        while(index < vector->length -1){   \
            vector->data[index] = vector->data[index+1];    \
            ++index;    \
        }   \
        vector->length--;   \
        return valueAtIndex;    \
    }   \
    \
    static inline T popFront_##Type(rutil_vector_##T* vector){   \
        return pop_##Type(vector,0);    \
    }   \
    static inline void pushBackList_##Type(rutil_vector_##T* vector,T* value,usize list_size){  \
        for(usize i = 0; i < list_size; ++i){      \
            pushBack_##Type(vector,value[i]);       \
        }       \
    }   \
    \
    static inline mpanic void setAtIndex_##Type(rutil_vector_##T* vector,T value,usize index){ \
        if(index > vector->length-1){          \
            fprintf(stderr,"[Panic]: %s","Vector Index Out of bound!\n");   \
            exit(EXIT_FAILURE); \
        }   \
        vector->data[index] = value;    \
    }\
    static inline T* vectorFront_##Type(rutil_vector_##T* vector){  \
        return vector->data;    \
    } \
    static inline T* vectorAt_##Type(rutil_vector_##T* vector,usize idx){  \
        return vector->data + idx; \
    }\
    static inline T* vectorBack_##Type(rutil_vector_##T* vector){  \
        return vector->data + (vector->length); \
    }\
    static inline T* vectorGetLast_##Type(rutil_vector_##T* vector){  \
        return vector->data + (vector->length -1); \
    }\
    static inline rutil_vector_##T* filter_##Type(rutil_vector_##T* vector,bool(*predicate)(T)){    \
        rutil_vector_##T* filteredVector = newVector_##Type(); \
        \
        T* begin = vectorFront_##Type(vector);         \
        T* end = vectorBack_##Type(vector);    \
    \
        for(T* i = begin ; i != end; ++i){  \
            if(predicate(*i))   \
                pushBack_##Type(filteredVector,*i); \
        }   \
        \
        return filteredVector;  \
    }\
    static inline rutil_vector_##T* map_##Type(rutil_vector_##T* vector, T (*transform)(T)){   \
        rutil_vector_##T* mappedVector = reserveVectorCapacity_##Type(vector->capacity); \
        \
        T* begin = vectorFront_##Type(vector);         \
        T* end = vectorBack_##Type(vector);    \
        \
        for(T* i = begin; i != end; ++i){  \
            pushBack_##Type(mappedVector,transform(*i)); \
        }   \
        return mappedVector;    \
    }   \
    \
    static inline void clearVector_##Type(rutil_vector_##T* vector){ \
        vector->length = 0; \
    }\
    static inline void deleteVector_##Type(rutil_vector_##T** vector){ \
        free((*vector)->data);  \
        free(*vector);  \
    }   \
    static inline void deleteVectorByDestructor_##Type(rutil_vector_##T** vector,void (*destructor)(T*)){ \
        rutil_vec_for_each(idx,(*vector)){    \
            destructor(&(*vector)->data[idx]);  \
        }   \
        free((*vector)->data);  \
        free(*vector);  \
    }   \
    static inline void printVector_##Type(rutil_vector_##T* vector,void (*reflector)(T)){ \
        T* begin = vectorFront_##Type(vector);         \
        T* end = vectorBack_##Type(vector);    \
        \
        printf("[ ");    \
        for(T* i = begin; i != end; ++i){  \
            reflector(*i);  \
        }   \
        printf("]\n");    \
    }   \
    \
    static inline usize getVectorLength_##Type(rutil_vector_##T* vector){ \
        return vector->length;  \
    }   \
    static inline usize getVectorCapacity_##Type(rutil_vector_##T* vector){   \
        return vector->capacity;    \
    }   \
    static inline void sortVector_##Type(rutil_vector_##T* vector,int (*comparator)(const void *, const void *)){ \
        qsort(                  \
            vector->data,       \
            vector->length,     \
            vector->type_size,  \
            comparator          \
        );                      \
    }\
    static inline T* findVector_##Type(rutil_vector_##T* vector,T* key,int (*comparator)(const void *, const void *)){   \
        return bsearch( \
            key,        \
            vector->data,   \
            vector->length, \
            vector->type_size,  \
            comparator \
        ); \
    }\
    static inline bool vectorContains_##Type(rutil_vector_##T* vector,T* key,int (*comparator)(const void *, const void *)){   \
        if (findVector_##Type(vector,key,comparator) != NULL)       \
            return true;    \
        else    \
            return false;   \
    }   \
    \
    static inline rutil_vector_##T* vectorOf_##Type##_impl(T* arr, usize count) {   \
        rutil_vector_##T* v = newVector_##Type();                      \
        appendList_##Type(v, arr, count);                           \
        return v;                                                   \
    } \
    static inline bool isEmptyVector_##Type(rutil_vector_##T* vector){    \
        if(vector->length == 0) \
            return true;    \
        else \
            return false;   \
    }   \

#define vectorOf(Type, ...) \
    vectorOf_##Type##_impl((Type[]){__VA_ARGS__}, \
                            sizeof((Type[]){__VA_ARGS__}) / sizeof(Type))


                         
                            
//rutil_monomorphized_vector(i32, i32)
/*
int main(){
    rutil_vector_i32* vector = newVector_i32();

    pushBack_i32(vector, 1);
    pushBack_i32(vector, 2);
    pushBack_i32(vector, 3);
    pushBack_i32(vector, 4);
    pushBack_i32(vector, 5);

    printVector_i32(vector,rutil_reflect_print_i32);

    deleteVector_i32(&vector);    

    return 0;
}
*/
/* rutil_monomorphized_array(i32)
i32 main(){
    rutil_array_i32* a1 = array_new_i32(10);
    array_push_i32(a1,5,1,2,3,4,5);

    array_print_i32(a1, rutil_reflect_print_i32);

    array_for_each_i32(a1,square_i32);

    array_push_i32(a1, 3,80,90,100);

    array_print_i32(a1, rutil_reflect_print_i32);

    array_push_i32(a1, 5,1,2,3,4,5);

    array_drop_i32(&a1);

    printf("\n");

    return 0;
}
    */

/*
int main(){
    int arr[5];

    int length = 0;

    for(int i = 0; i < 3 ; ++i)
        arr[length++] = i + 1;        

    for(int i = 0; i < 2; ++i)
        arr[length++] = i + 4;
    
    arr[length] = 10561;

    printf("[ ");
    
    for(int i = 0; i < 5; ++i)
        printf("%d ",arr[i]);    
    

    printf("]\n");
        
    return 0;
}*/