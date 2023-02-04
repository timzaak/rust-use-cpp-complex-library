# On Ubuntu 18.04, get the latest CMake from https://apt.kitware.com/.
cmake_minimum_required(VERSION 3.18)

set(target_lib "open3d_wrapper")
project(${target_lib} LANGUAGES C CXX)

find_package(Open3D REQUIRED)


add_library(${target_lib} SHARED src/open3d_wrapper.cc include/open3d_wrapper.h)

target_link_libraries(${target_lib} PRIVATE Open3D::Open3D)
target_include_directories(${target_lib}
        PUBLIC
        ${PROJECT_SOURCE_DIR}/include
        )

set(CMAKE_INSTALL_PREFIX "${PROJECT_SOURCE_DIR}")
install(TARGETS ${target_lib} LIBRARY DESTINATION)




# execute
#add_executable(open_rs_t src/main.cpp)
#target_link_libraries(open_rs_t PRIVATE ${target_lib})
