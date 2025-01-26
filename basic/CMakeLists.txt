cmake_minimum_required(VERSION 3.10)
project(basic)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GSTREAMER REQUIRED gstreamer-1.0)
pkg_check_modules(GSTREAMER_BASE REQUIRED gstreamer-base-1.0)

add_executable(${PROJECT_NAME} main.c)

# 添加包含目录
target_include_directories(${PROJECT_NAME} PUBLIC ${GSTREAMER_INCLUDE_DIRS})
# 添加链接库
target_link_libraries(${PROJECT_NAME} ${GSTREAMER_LIBRARIES} ${GSTREAMER_BASE_LIBRARIES})
# 添加编译选项
target_compile_options(${PROJECT_NAME} PUBLIC ${GSTREAMER_CFLAGS_OTHER})