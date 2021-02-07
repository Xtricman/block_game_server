cmake_minimum_required(VERSION 3.14)

project(BlockGameServer)

add_executable(${PROJECT_NAME} main.cpp)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

add_subdirectory(3rd/nlohmann_json)
add_subdirectory(3rd/nlohmann_json_schema_validator)

target_link_libraries(${PROJECT_NAME} nlohmann_json)
target_link_libraries(${PROJECT_NAME} nlohmann_json_schema_validator)