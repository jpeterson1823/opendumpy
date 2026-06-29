cmake_minimum_required(VERSION 3.13)

# C Settings
set(CMAKE_C_STANDARD 23)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Pico Settings
set(PICO_PLATFORM "rp2350")
set(PICO_BOARD "none")
include(libs/pico-sdk/pico_sdk_init.cmake)

project(opendumpy C CXX ASM)

# init Pico SDK
pico_sdk_init()

# Project Settings
add_executable(opendumpy)
target_sources(opendumpy PRIVATE
    opendumpy.c
)
target_include_directories(opendumpy PRIVATE
    ./include
)
target_link_libraries(opendumpy PRIVATE
    pico_stdlib
    hardware_gpio
)

# More Pico Settings
pico_set_program_name(opendumpy "OpenDumpy")
pico_set_program_version(opendumpy "1.0")
pico_enable_stdio_uart(opendumpy 0)
pico_enable_stdio_usb(opendumpy 1)
pico_add_extra_outputs(opendumpy)
