cmake_minimum_required(VERSION 3.10)
project(lg_xboxsys C)
add_library(lg_xboxsys STATIC sys_xbox.c)
set_target_properties(lg_xboxsys PROPERTIES PREFIX "" OUTPUT_NAME lg_xboxsys)
