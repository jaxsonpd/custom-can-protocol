/**
 * @file packet_processing.c
 * @author Jack Duignan (JackpDuignan@gmail.com)
 * @date 2025-01-25
 * @brief This file contains the implementation of the packet processing module
 * which allows automated processing of packets after they are received.
 */


#include <stdint.h>
#include <stdbool.h>

#include "../custom_can_protocol/packet.h"
#include "../custom_can_protocol/packet_handler.h"

#include "../custom_can_protocol/packet_processing.h"

struct PacketProcessor packetProcessors[MAX_NUM_IDENTIFIERS] = {0};

int packet_processing_add_callback(struct PacketProcessor packetProcessor) {
    packetProcessors[packetProcessor.identifier] = packetProcessor;

    return 0;
}

int packet_processing_process(uint8_t *buffer, uint16_t bufferLength) {
    packetStatus_t validateResult = packet_validate(buffer, bufferLength);
    if (validateResult != PACKET_VALID) {
        return validateResult;
    }

    struct PacketProcessor currentProcessor = packetProcessors[buffer[PACKET_IDENTIFIER_LOC]];

    packetProcessingResult_t processResult = PROCESS_COMPLETE;
    processResult = currentProcessor.packet_processing_cb(
                        &buffer[PACKET_PAYLOAD_START_LOC], 
                        bufferLength-PACKET_HEADER_SIZE-PACKET_FOOTER_SIZE);

    return processResult; 
}