cmake_minimum_required(VERSION 3.0.0 FATAL_ERROR)
project (libphonenumber_sys_c)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include ${CMAKE_CURRENT_BINARY_DIR})

file(GLOB sources src/*.cpp)
file(GLOB headers include/*.h)
add_library(libphonenumber_sys_c STATIC ${sources})

include_directories("/usr/include" "/usr/lib/gcc/x86_64-linux-gnu/6/include" "/usr/lib/gcc/x86_64-linux-gnu/6/include-fixed")
add_definitions(-DLIBPHONENUMBER_SYS_C_LIBRARY)

install(TARGETS libphonenumber_sys_c
  LIBRARY DESTINATION lib
  ARCHIVE DESTINATION lib
  RUNTIME DESTINATION lib
)

install(FILES ${headers} DESTINATION include)

if(${CMAKE_COMPILER_IS_GNUCXX})
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Werror=return-type")
endif()
set (CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ")
