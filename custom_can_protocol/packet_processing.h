/** 
 * @file packet_processing.h
 * @author Jack Duignan (JackpDuignan@gmail.com)
 * @date 2025-01-25
 * @brief A module to process packets data using custom callbacks to handle
 * data automatically when a packet is received.
 */


#ifndef PACKET_PROCESSING_H
#define PACKET_PROCESSING_H


#include <stdint.h>
#include <stdbool.h>

typedef enum packetProcessingResults_e {
    PROCESS_COMPLETE,
    PROCESS_RESEND_PACKET,
} packetProcessingResult_t;

/**
 * @struct PacketProcessor
 * 
 * @brief Holds the information for a custom callback to process one packet
 * identifier
 * 
 */
struct PacketProcessor {
    uint8_t identifier; // The packet identifier associated with this callback
    packetProcessingResult_t (*packet_processing_cb)(uint8_t*, uint16_t); // Callback to process packet, takes: data, data length
};

/** 
 * @brief Add a new identifier processor/callback to the processor list
 * @param packetProcessor the packet processor to add
 * 
 * @return 0 if successful
 */
int packet_processing_add_callback(struct PacketProcessor packetProcessor);

/** 
 * @brief Take a packet and validate it then call the correct process callback to consume the data
 * @param packet the packet to process
 * @param length the length of the packet to process
 * 
 * @return the result of the processing
 */
int packet_processing_process(uint8_t* buffer, uint16_t bufferLength);



#endif // PACKET_PROCESSING_H