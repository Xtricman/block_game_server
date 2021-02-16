cmake_minimum_required(VERSION 3.14)

project(BlockGameServer)

add_executable(${PROJECT_NAME} main.cpp)
set_target_properties(${PROJECT_NAME} PROPERTIES CXX_STANDARD 20)
target_link_options(${PROJECT_NAME} PRIVATE -static-libstdc++ -static-libgcc)
