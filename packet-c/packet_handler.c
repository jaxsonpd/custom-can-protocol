/**
 * @file packet_handler.c
 * @author Jack Duignan (JackpDuignan@gmail.com)
 * @date 2024-12-27
 * @brief Implementation for the packet handling functionality
 */


#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

#include "../custom_can_protocol/packet.h"

#include "../custom_can_protocol/packet_handler.h"

typedef enum packetStates_e {
    START_BYTE,
    CMD_BYTE,
    PACKET_LENGTH_BYTE,
    PACKET_DATA_BYTES,
    CRC_BYTES,
    END_BYTE,
    PACKET_COMPLETE
} packetState_t;

static uint16_t calculate_crc16(const uint8_t *data, uint16_t length) {
    uint16_t crc = 0xFFFF;
    const uint16_t polynomial = PACKET_CRC_POLYNOMIAL;

    for (uint16_t i = 0; i < length; i++) {
        crc ^= (data[i] << 8);

        for (uint8_t bit = 0; bit < 8; bit++) {
            if (crc & 0x8000) {
                crc = (crc << 1) ^ polynomial;
            } else {
                crc <<= 1;
            }
        }
    }

    return crc;
}

packetStatus_t packet_validate(uint8_t* packetBuffer, uint16_t bufferLength) {
    if (bufferLength < MIN_PACKET_LENGTH) { // Ensure minimum length for a valid packet
        return PACKET_SCHEMA_ERROR;
    }

    uint16_t currentByte = 0;
    uint16_t payloadLength = 0;
    packetState_t state = START_BYTE;

    while (currentByte < bufferLength && state != PACKET_COMPLETE) {
        uint8_t byte = packetBuffer[currentByte];
        
        switch (state) {
            case START_BYTE:
                if (byte != PACKET_START_BYTE || currentByte != 0) {
                    return PACKET_SCHEMA_ERROR;
                }
                state = CMD_BYTE;
                currentByte++;
                break;

            case CMD_BYTE:
                if (byte == PACKET_START_BYTE || byte == PACKET_END_BYTE || currentByte != 1) {
                    return PACKET_SCHEMA_ERROR;
                }
                state = PACKET_LENGTH_BYTE;
                currentByte++;
                break;

            case PACKET_LENGTH_BYTE:
                if (currentByte != 2) {
                    return PACKET_SCHEMA_ERROR;
                }
                payloadLength = byte;
                if (payloadLength + PACKET_HEADER_SIZE + CRC_LENGTH + PACKET_FOOTER_SIZE > bufferLength) {
                    return PACKET_LENGTH_ERROR;
                }
                state = PACKET_DATA_BYTES;
                currentByte++;
                break;

            case PACKET_DATA_BYTES:
                for (uint8_t i = 0; i < payloadLength; i++) {
                    if (currentByte + i >= bufferLength || 
                        packetBuffer[currentByte + i] == PACKET_START_BYTE || 
                        packetBuffer[currentByte + i] == PACKET_END_BYTE) {
                        return PACKET_LENGTH_ERROR;
                    }
                }
                currentByte += payloadLength;
                state = CRC_BYTES;
                break;

            case CRC_BYTES: {
                if (currentByte + 2 > bufferLength) {
                    return PACKET_SCHEMA_ERROR;
                }
                uint16_t crc16 = calculate_crc16(&packetBuffer[PACKET_PAYLOAD_START_LOC], payloadLength);
                uint16_t receivedCrc16 = (packetBuffer[currentByte] << 8) | packetBuffer[currentByte + 1];
                if (crc16 != receivedCrc16) {
                    return PACKET_CRC_ERROR;
                }
                currentByte += 2;
                state = END_BYTE;
                break;
            }

            case END_BYTE:
                if (byte != PACKET_END_BYTE) {
                    return PACKET_SCHEMA_ERROR;
                }
                state = PACKET_COMPLETE;
                currentByte++;
                break;

            default:
                return PACKET_SCHEMA_ERROR;
        }
    }

    return (state == PACKET_COMPLETE) ? PACKET_VALID : PACKET_SCHEMA_ERROR;
}

uint16_t packet_compile(uint8_t* packetBuf, uint8_t* payloadBuf, uint16_t payloadLength, uint8_t packetIdent) {
    if (packetBuf == NULL) {
        return 0;
    } else if (payloadBuf == NULL && payloadLength > 0) {
        return 0;
    }

    packetBuf[0] = PACKET_START_BYTE;
    packetBuf[PACKET_IDENTIFIER_LOC] = packetIdent;
    packetBuf[PACKET_LENGTH_LOC] = payloadLength;
    for (uint16_t i = 0; i < payloadLength; i++)
    {
        packetBuf[PACKET_PAYLOAD_START_LOC+i] = payloadBuf[i];
    }

    uint16_t crc16 = calculate_crc16(payloadBuf, payloadLength);

    packetBuf[PACKET_PAYLOAD_START_LOC+payloadLength] = crc16 >> 8 & 0xFF;
    packetBuf[PACKET_PAYLOAD_START_LOC+payloadLength+1] = crc16 & 0xFF;

    packetBuf[PACKET_PAYLOAD_START_LOC+payloadLength+2] = PACKET_END_BYTE;

    return PACKET_HEADER_SIZE+PACKET_FOOTER_SIZE+payloadLength+2;
}

int packet_send(int (sendByte)(int), uint8_t* payloadBuf, uint16_t payloadLength, uint8_t packetIdent) {
    uint8_t sendBuffer[MAX_PACKET_LENGTH] = {0};
    uint16_t packetLength = packet_compile(sendBuffer, payloadBuf, payloadLength, packetIdent);
    for (uint16_t i = 0; i < packetLength; i++) {
        sendByte(sendBuffer[i]);
    }

    return 0;
}

uint16_t packet_receive(int (readByte)(void), uint8_t *buffer) {
    uint16_t idx = 0;
    
    int byte;
    while ((byte = readByte()) != EOF) {
        if (byte == PACKET_START_BYTE) {
            buffer[idx++] = (uint8_t)PACKET_START_BYTE;
            break;
        }
    }

    if (byte == EOF) {
        return 0;
    }

    while (idx < MAX_PACKET_LENGTH) {
        byte = readByte();

        if (byte == EOF) {
            continue;
        }
        buffer[idx++] = (uint8_t)byte;
        if (byte == PACKET_END_BYTE) {
            break;
        }
    }

    return idx;
}