#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>

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

#define mpanic

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


#define rutil_monomorphized_array(T,Type,Size) \
    typedef struct{              \
        T data[Size];                 \
        usize length;            \
        usize capacity;          \
    }rutil_array_##Type;            \
                                 \
    static inline                                                                                               \
    rutil_array_##Type array_new_##Type(){                                   \
        rutil_array_##Type array = {};                                         \
        array.length = 0;                                                     \
        array.capacity = Size;                                                \
        return array;                                                         \
    }                                                                                                           \
    static inline                                                                                               \
    void mpanic array_push_##Type(rutil_array_##Type* self,usize element_count,...){                            \
        va_list ap;                                                                                             \
        if(self->length == self->capacity || ((self->capacity - self->length) < element_count)){                \
            rutil_panic_handler(ArraySpaceLimitExceeded);                                                        \
        }                                                                                                       \
        va_start(ap, element_count);                                                                            \
        for(usize i = 0; i < element_count; ++i){                                                               \
            self->data[self->length] =  va_arg(ap,T);                                                           \
            self->length++;                                                                                     \
        }                                                                                                       \
        va_end(ap);                                                                                             \
    }                                                                                                           \
    static inline                                                                                               \
    void array_print_##Type(rutil_array_##Type* self,void (*reflector)(T)){                                     \
            printf("[ ");                                                                                       \
        for(usize i = 0; i < self->length; ++i)                                                                 \
            reflector(self->data[i]);                                                                           \
        printf("]\n");                                                                                          \
    }                                                                                                           \
                                                                                                     \
    static inline                                                                                               \
    void array_reflection_##Type(rutil_array_##Type* self,void (*reflector)(T)){                                \
        printf("[ \n    data: ");                                                                               \
        array_print_##Type(self,reflector);                                                                  \
        printf("    length: %zu,\n",self->length);                                                              \
        printf("    capacity: %zu,\n",self->capacity);                                                          \
        printf("]\n");                                                                                          \
    }                                                                                                           \
    static inline                                                                                               \
    void array_breflection_##Type(rutil_array_##Type* self){                                                    \
        printf("[ \n    data: %p,\n",self->data);                                                               \
        printf("    length: %zu,\n",self->length);                                                              \
        printf("    capacity: %zu,\n",self->capacity);                                                          \
        printf("]\n");                                                                                          \
    }                                                                                                           \
    static inline                                                                                               \
    T mpanic array_get_at_##Type(rutil_array_##Type* self,usize index){                                         \
        if(index >= self->length){                                                                              \
            rutil_panic_handler(ArrayIndexOutOFBound);                                                          \
            exit(ArrayIndexOutOFBound);                                                                         \
        }                                                                                                       \
        return self->data[index];                                                                               \
    }                                                                                                           \
    static inline                                                                                               \
    void mpanic array_set_at_##Type(rutil_array_##Type* self,usize index,T value){                              \
        if(index >= self->length){                                                                              \
            rutil_panic_handler(ArrayIndexOutOFBound);                                                          \
            exit(ArrayIndexOutOFBound);                                                                         \
        }                                                                                                       \
        self->data[index] = value;                                                                              \
    }                                                                                                           \
    static inline                                                                                               \
    void array_for_each_##Type(rutil_array_##Type* self,T (*map)(T)){   \
        for(usize i = 0; i < self->length; ++i){    \
            self->data[i] = map(self->data[i]); \
        }\
    }\
    static inline\
    rutil_array_##Type mpanic array_clone_##Type(rutil_array_##Type* self){    \
        rutil_array_##Type cloned = array_new_##Type();\
        memcpy(cloned.data, self->data,self->length * sizeof (T));  \
        cloned.length = self->length;    \
        return cloned;  \
    }\

rutil_monomorphized_array(int, int5,5)

int main(){

    rutil_array_int5 arr = array_new_int5();
    array_push_int5(&arr, 4, 1,2,3,4);


    return 0;
}