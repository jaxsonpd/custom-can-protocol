use std::{error::Error, io::{Read, Write}};

use std::fmt;

pub const PACKET_START_BYTE: u8 = 0x7E;
pub const PACKET_END_BYTE: u8 = 0x7F;

pub const MIN_PACKET_LENGTH: usize = 6;
pub const MAX_PACKET_LENGTH: usize = 1024;
pub const PACKET_HEADER_SIZE: usize = 3;
pub const PACKET_FOOTER_SIZE: usize = 1;

pub const CRC_LENGTH: usize = 2;

pub const PACKET_CRC_POLYNOMIAL: u16 = 0x1021;

pub const PROTOCOL_PACKET_IDENTIFIER: u8 = 0xFF; // Identifies a protocol packet not a data packet

pub const MAX_NUM_IDENTIFIERS: u8 = 0xFF; // The maximum number of identifiers should be the same as the highest possible identifier.

#[repr(u8)]
pub enum PacketByteLocations {
    PacketIdentifierLoc = 0x01,
    PacketLengthLoc = 0x02,
    PacketPayloadStartLoc = 0x03,
}

#[derive(Debug)]
pub enum PacketValidationError {
    SchemaError,
    LengthError,
    CrcError,
}

#[derive(Debug, PartialEq)]
pub enum PacketState {
    StartByte,
    CmdByte,
    PacketLengthByte,
    PacketDataBytes,
    CrcBytes,
    EndByte,
    PacketComplete,
}

pub struct Packet {
    pub buffer: Vec<u8>,
    pub packet_ident: u8,
    pub payload_length: u8,
    pub payload: Vec<u8>,
    pub packet_buffer: Vec<u8>,
}

pub trait PacketHandler {
    fn handle_packet(&mut self, packet: &Packet) -> Result<(), Box<dyn Error>>;
    fn get_packet_id(&self) -> u8;
}

impl Packet {
    fn calculate_crc16(payload: &[u8]) -> u16 {
        let mut crc: u16 = 0xFFFF;
        let polynomial: u16 = PACKET_CRC_POLYNOMIAL;

        for &byte in payload {
            crc ^= (byte as u16) << 8;

            for _ in 0..8 {
                if crc & 0x8000 != 0 {
                    crc = (crc << 1) ^ polynomial;
                } else {
                    crc <<= 1;
                }
            }
        }

        crc
    }

    pub fn validate_packet(buffer: &[u8]) -> Result<(), PacketValidationError> {
        if buffer.len() < MIN_PACKET_LENGTH {
            return Err(PacketValidationError::SchemaError);
        }

        let mut current_byte = 0;
        let mut state = PacketState::StartByte;

        while current_byte < buffer.len() && state != PacketState::PacketComplete {
            let byte = buffer[current_byte];

            match state {
                PacketState::StartByte => {
                    if byte != PACKET_START_BYTE || current_byte != 0 {
                        return Err(PacketValidationError::SchemaError);
                    }
                    state = PacketState::CmdByte;
                    current_byte += 1;
                }
                PacketState::CmdByte => {
                    if byte == PACKET_START_BYTE || byte == PACKET_END_BYTE || current_byte != 1 {
                        return Err(PacketValidationError::SchemaError);
                    }
                    state = PacketState::PacketLengthByte;
                    current_byte += 1;
                }
                PacketState::PacketLengthByte => {
                    if current_byte != 2 {
                        return Err(PacketValidationError::SchemaError);
                    }
                    let payload_length = byte as usize;
                    if payload_length + PACKET_HEADER_SIZE + CRC_LENGTH + PACKET_FOOTER_SIZE
                        > buffer.len()
                    {
                        return Err(PacketValidationError::LengthError);
                    }
                    state = PacketState::PacketDataBytes;
                    current_byte += 1;
                }
                PacketState::PacketDataBytes => {
                    for i in 0..(buffer[PacketByteLocations::PacketLengthLoc as usize] as usize) {
                        if current_byte + i >= buffer.len()
                            || buffer[current_byte + i] == PACKET_START_BYTE
                            || buffer[current_byte + i] == PACKET_END_BYTE
                        {
                            return Err(PacketValidationError::LengthError);
                        }
                    }
                    current_byte += buffer[PacketByteLocations::PacketLengthLoc as usize] as usize;
                    state = PacketState::CrcBytes;
                }
                PacketState::CrcBytes => {
                    if current_byte + 2 > buffer.len() {
                        return Err(PacketValidationError::SchemaError);
                    }
                    let crc16 = Packet::calculate_crc16(&buffer[(PacketByteLocations::PacketPayloadStartLoc as usize)
                                                                            ..(PacketByteLocations::PacketPayloadStartLoc as usize 
                                                                            + buffer[PacketByteLocations::PacketLengthLoc as usize] as usize)]);
                    let received_crc16 = ((buffer[current_byte] as u16) << 8)
                        | (buffer[current_byte + 1] as u16);
                    if crc16 != received_crc16 {
                        return Err(PacketValidationError::CrcError);
                    }
                    current_byte += 2;
                    state = PacketState::EndByte;
                }
                PacketState::EndByte => {
                    if byte != PACKET_END_BYTE {
                        return Err(PacketValidationError::SchemaError);
                    }
                    state = PacketState::PacketComplete;
                    current_byte += 1;
                }
                PacketState::PacketComplete => {}
            }
        }

        if state == PacketState::PacketComplete {
            Ok(())
        } else {
            Err(PacketValidationError::SchemaError)
        }
    }

    pub fn read_from_stream(read_byte: &mut dyn Read) -> Result<Self, PacketValidationError> {
        let mut buffer = Vec::new();
        let mut byte = [0u8; 1];

        println!("Reading from stream");

        loop {
            match read_byte.read(&mut byte) {
                Ok(1) => {
                    if byte[0] == PACKET_START_BYTE {
                        buffer.push(PACKET_START_BYTE);
                        break;
                    }

                    println!("Read byte: {}", byte[0]);
                },
                _ => continue
            }
            
        }

        println!("Got start");

        while buffer.len() < MAX_PACKET_LENGTH {
            match read_byte.read(&mut byte) {
                Ok(1) => {
                    buffer.push(byte[0]);
                    if byte[0] == PACKET_END_BYTE {
                        break;
                    }

                    println!("Read byte: {}", byte[0]);
                },
                _ => continue
            }
        }

        match Packet::validate_packet(&buffer) {
            Ok(_) => Ok(Packet {
                buffer: buffer.to_vec(),
                packet_ident: buffer[PacketByteLocations::PacketIdentifierLoc as usize],
                payload_length: buffer[PacketByteLocations::PacketLengthLoc as usize],
                payload: buffer[(PacketByteLocations::PacketPayloadStartLoc as usize)
                                ..(PacketByteLocations::PacketPayloadStartLoc as usize 
                                + buffer[PacketByteLocations::PacketLengthLoc as usize] as usize)].to_vec(),
                packet_buffer: buffer.to_vec(),
            }),
            Err(e) => Err(e)
        }
    }

    // Constructor to initialize a new Packet
    pub fn new(packet_ident: u8, payload: Vec<u8>, packet_buffer: Vec<u8>) -> Self {
        let payload_length = payload.len() as u8;
        let buffer =
            vec![0; PACKET_HEADER_SIZE + payload_length as usize + CRC_LENGTH + PACKET_FOOTER_SIZE];

        Packet {
            buffer,
            packet_ident,
            payload_length,
            payload,
            packet_buffer,
        }
    }

    // Method to compile the packet into the buffer
    pub fn compile(&mut self) -> usize {
        self.buffer[0] = PACKET_START_BYTE;
        self.buffer[1] = self.packet_ident;
        self.buffer[2] = self.payload_length;

        self.buffer[PacketByteLocations::PacketPayloadStartLoc as usize
            ..(PacketByteLocations::PacketPayloadStartLoc as usize + self.payload_length as usize)]
            .copy_from_slice(&self.payload);

        let crc16 = Packet::calculate_crc16(&self.payload);

        self.buffer
            [PacketByteLocations::PacketPayloadStartLoc as usize + self.payload_length as usize] =
            (crc16 >> 8) as u8;
        self.buffer[PacketByteLocations::PacketPayloadStartLoc as usize
            + self.payload_length as usize
            + 1] = (crc16 & 0xFF) as u8;

        self.buffer[PacketByteLocations::PacketPayloadStartLoc as usize
            + self.payload_length as usize
            + 2] = PACKET_END_BYTE;

        PACKET_HEADER_SIZE + self.payload_length as usize + CRC_LENGTH + PACKET_FOOTER_SIZE
    }

    // Method to send the packet using the provided byte sender function
    pub fn write_to_stream(&self, send_byte: &mut dyn Write) {
        for byte in &self.buffer {
            send_byte.write_all(&[*byte]).unwrap();
        }
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Packet")
            .field("packet_ident", &self.packet_ident)
            .field("payload_length", &self.payload_length)
            .field("payload", &self.payload)
            .field("buffer", &self.buffer.get(..20)) // Display first 20 bytes of the buffer for brevity
            .finish()
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Packet Identifier: {}\nPayload Length: {}\nPayload: {:?}\nBuffer Preview: {:?}",
            self.packet_ident,
            self.payload_length,
            self.payload,
            self.buffer.get(..20) // Preview of the first 20 bytes of the buffer
        )
    }
}