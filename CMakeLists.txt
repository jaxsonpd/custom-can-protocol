add_library(custom-can-protocol packet-c/packet_handler.c packet-c/packet_processing.c)

# target_compile_options(avr-extends PRIVATE -Os -DF_CPU=16000000UL -mmcu=atmega328p)
target_include_directories(custom-can-protocol PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})