add_library(custom-can-module packet-c/packet_handler.c)

# target_compile_options(avr-extends PRIVATE -Os -DF_CPU=16000000UL -mmcu=atmega328p)
target_include_directories(custom-can-module PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})