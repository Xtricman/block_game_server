cmake_minimum_required(VERSION 3.14)

project(BlockGameServer)

add_executable(${PROJECT_NAME} main.cpp)
set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD 20)

add_subdirectory(3rd/nlohmann_json)

target_link_libraries(${PROJECT_NAME} nlohmann_json)
