#ifndef ERR_H
#define ERR_H

#include "util.h"
#include "log.h"
#include "toaster.h"

#ifdef TOASTER
#define TOASTER_INJECT_FAILURE(err, expr) \
    if(0 != toaster_check()) {\
      if(!err) {\
        err = -1;\
      }\
      LOG(DEBUG, "inject:%s", #expr); \
      goto CHECK(err); \
    } else
#else 
#define TOASTER_INJECT_FAILURE(err, expr)
#endif

#define TEST(err, expr) \
  do {\
    LOG(TRACE, "call:%s", #expr); \
    TOASTER_INJECT_FAILURE(err, expr) \
    if(!(expr)) {\
      if(!err) {\
        err = -1;\
      }\
      LOG(DEBUG, "fail:%s", #expr); \
      goto CHECK(err); \
    } else {\
        LOG(TRACE, "pass:%s", #expr); \
    }\
  } while(0)

#define TEST_FAIL 0
#define FAIL(err) TEST(err, TEST_FAIL)

#define CHECK(err) __ ## err ## _test_check

#endif //ERR_H
