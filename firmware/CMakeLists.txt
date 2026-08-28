cmake_minimum_required(VERSION 3.13)

# Pico Settings
set(PICO_PLATFORM "rp2350")
set(PICO_BOARD "none")
include(./include/pico-sdk/pico_sdk_init.cmake)

# C Settings
set(CMAKE_C_STANDARD 23)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON) # for coc-clangd support


# Project Settings
project(opendumpy C CXX ASM)
pico_sdk_init()
add_executable(opendumpy)

# Add source files to opendumpy executable
target_sources(opendumpy PRIVATE
    src/opendumpy.c

    src/opendumpy/gbxcart.c
    src/opendumpy/gbxlib.c
    src/opendumpy/gblib.c
    src/opendumpy/gbclib.c
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
