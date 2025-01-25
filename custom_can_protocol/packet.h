/** 
 * @file packet.h
 * @author Jack Duignan (JackpDuignan@gmail.com)
 * @date 2024-12-27
 * @brief Definitions for the packet schema
 */


#ifndef PACKET_H
#define PACKET_H


#include <stdint.h>
#include <stdbool.h>

#define PACKET_START_BYTE 0x7E
#define PACKET_END_BYTE 0x7E

#define MIN_PACKET_LENGTH 6
#define PACKET_HEADER_SIZE 3
#define PACKET_FOOTER_SIZE 1

#define CRC_LENGTH 0x02

#define PACKET_CRC_POLYNOMIAL 0x1021

#define PROTOCOL_PACKET_IDENTIFIER 0xFF // Identifies a protocol packet not a data packet

#define MAX_NUM_IDENTIFIERS 0xFF // The maximum number of identifiers should be the same as the highest possible identifier. 

enum packetByteLocations_e {
    PACKET_IDENTIFIER_LOC = 0x01,
    PACKET_LENGTH_LOC = 0x02,
    PACKET_PAYLOAD_START_LOC = 0x03,
};


#endif // PACKET_H