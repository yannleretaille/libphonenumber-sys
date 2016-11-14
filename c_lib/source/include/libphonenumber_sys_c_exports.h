#ifndef LIBPHONENUMBER_SYS_C_EXPORTS_H
#define LIBPHONENUMBER_SYS_C_EXPORTS_H

#ifdef _WIN32
    #ifdef LIBPHONENUMBER_SYS_C_LIBRARY
        #define LIBPHONENUMBER_SYS_C_EXPORT __declspec(dllexport)
    #else
        #define LIBPHONENUMBER_SYS_C_EXPORT __declspec(dllimport)
    #endif
#else
    #define LIBPHONENUMBER_SYS_C_EXPORT
#endif

#endif // LIBPHONENUMBER_SYS_C_EXPORTS_H

