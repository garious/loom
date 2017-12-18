#ifndef UTIL_H
#define UTIL_H

//string version of the label
#define __STR(x) #x
#define STR(x) __STR(x)

//string version of file line
#define __FILE_LINE__ __FILE__ ":" STR(__LINE__)

//compile time assert
#define C_ASSERT(test) \
    switch(0) {\
      case 0:\
      case test:;\
    }

#define FREEIF(var) \
    do {\
        void *p = var;\
        if(p) {\
            free(p);\
            var = 0;\
        }\
    } while(0);

#define MAX(a, b) (a > b) ? a : b
#define MIN(a, b) (a < b) ? a : b

#endif // UTIL_H
