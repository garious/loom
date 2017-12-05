#ifndef VISIBILITY_H
#define VISIBILITY_H

#ifdef TOASTER
#define PUBLIC
#define PRIVATE 
#define LOCAL 
#else
#define PUBLIC  __attribute__ ((visibility ("default")))
#define PRIVATE __attribute__ ((visibility ("hidden")))
#define LOCAL static
#endif

#endif // VISIBILITY_H
