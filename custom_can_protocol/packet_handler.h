/**
 * @file packet_decoder.h
 * @author Jack Duignan (JackpDuignan@gmail.com)
 * @date 2024-12-27
 * @brief Declarations for the packet handling functionality including validation
 * extraction and compilation.
 */


#ifndef PACKET_DECODER_H
#define PACKET_DECODER_H


#include <stdint.h>
#include <stdbool.h>

#include "packet.h"

typedef enum packetValidationStatus_e {
    PACKET_VALID,
    PACKET_LENGTH_ERROR,
    PACKET_CMD_ERROR,
    PACKET_CRC_ERROR,
    PACKET_SCHEMA_ERROR,
    PACKET_UNKOWN_ERROR,
} packetStatus_t;

/**
 * @brief Check if the packet is valid
 * @param packetBuffer the packet string
 * @param bufferLength the length of the packet
 *
 * @return the packet status
 */
packetStatus_t packet_validate(uint8_t* packetBuffer, uint16_t bufferLength);

/** 
 * @brief Compile a packet to send
 * @param packetBuf a pointer of the location to store the packet
 * @param payloadBuf the payload to send
 * @param payloadLength the length of the payload
 * @param commandByte the command byte to send
 * 
 * @return the status of the packet should mainly be schema errors
 * due to using reserved bytes for command or length
 */
packetStatus_t packet_compile(uint8_t* packetBuf, uint8_t* payloadBuf, uint8_t payloadLength, packetIdentifier_t packetIdent);


#endif // PACKET_DECODER_H