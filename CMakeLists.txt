cmake_minimum_required(VERSION 3.10)
project(rmaptest)

link_directories("rmap/target/release/")
add_executable(rmaptest
    src/rustymap.c
    src/rmap.h)

target_link_libraries(rmaptest rmap)