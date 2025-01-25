pub const PACKET_START_BYTE: u8 = 0x7E;
pub const PACKET_END_BYTE: u8 = 0x7E;

pub const MIN_PACKET_LENGTH: usize = 6;
pub const PACKET_HEADER_SIZE: usize = 3;
pub const PACKET_FOOTER_SIZE: usize = 1;

pub const CRC_LENGTH: usize = 2;

pub const PACKET_CRC_POLYNOMIAL: u16 = 0x1021;

pub const PROTOCOL_PACKET_IDENTIFIER: u8 = 0xFF; // Identifies a protocol packet not a data packet

pub const MAX_NUM_IDENTIFIERS: u8 = 0xFF; // The maximum number of identifiers should be the same as the highest possible identifier.

#[repr(u8)]
pub enum PacketByteLocations {
    PACKET_IDENTIFIER_LOC = 0x01,
    PACKET_LENGTH_LOC = 0x02,
    PACKET_PAYLOAD_START_LOC = 0x03,
}

pub struct Packet {
    pub buffer: Vec<u8>,
    pub packet_ident: u8,
    pub payload_length: u8,
    pub payload: Vec<u8>,
}

impl Packet {
    // Constructor to initialize a new Packet
    pub fn new(packet_ident: u8, payload: Vec<u8>) -> Self {
        let payload_length = payload.len() as u8;
        let mut buffer = vec![0; PACKET_HEADER_SIZE + payload_length as usize + CRC_LENGTH + PACKET_FOOTER_SIZE];

        Packet {
            buffer,
            packet_ident,
            payload_length,
            payload,
        }
    }

    // Method to calculate the CRC16 for the packet
    fn calculate_crc16(&self) -> u16 {
        let mut crc: u16 = 0xFFFF;
        let polynomial: u16 = PACKET_CRC_POLYNOMIAL;

        for &byte in &self.payload {
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

    // Method to validate the packet according to the schema
    pub fn validate(&self) -> Result<(), PacketValidationError> {
        if self.buffer.len() < MIN_PACKET_LENGTH {
            return Err(PacketValidationError::SchemaError);
        }

        let mut current_byte = 0;
        let mut state = PacketState::StartByte;

        while current_byte < self.buffer.len() && state != PacketState::PacketComplete {
            let byte = self.buffer[current_byte];

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
                    if payload_length + PACKET_HEADER_SIZE + CRC_LENGTH + PACKET_FOOTER_SIZE > self.buffer.len() {
                        return Err(PacketValidationError::LengthError);
                    }
                    state = PacketState::PacketDataBytes;
                    current_byte += 1;
                }
                PacketState::PacketDataBytes => {
                    for i in 0..self.payload_length as usize {
                        if current_byte + i >= self.buffer.len() || self.buffer[current_byte + i] == PACKET_START_BYTE || self.buffer[current_byte + i] == PACKET_END_BYTE {
                            return Err(PacketValidationError::LengthError);
                        }
                    }
                    current_byte += self.payload_length as usize;
                    state = PacketState::CrcBytes;
                }
                PacketState::CrcBytes => {
                    if current_byte + 2 > self.buffer.len() {
                        return Err(PacketValidationError::SchemaError);
                    }
                    let crc16 = self.calculate_crc16();
                    let received_crc16 = ((self.buffer[current_byte] as u16) << 8) | (self.buffer[current_byte + 1] as u16);
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
                PacketState::PacketComplete => {
                    
                }
            }
        }

        if state == PacketState::PacketComplete {
            Ok(())
        } else {
            Err(PacketValidationError::SchemaError)
        }
    }

    // Method to compile the packet into the buffer
    pub fn compile(&mut self) -> usize {
        self.buffer[0] = PACKET_START_BYTE;
        self.buffer[1] = self.packet_ident;
        self.buffer[2] = self.payload_length;

        self.buffer[PacketByteLocations::PACKET_PAYLOAD_START_LOC as usize..(PacketByteLocations::PACKET_PAYLOAD_START_LOC as usize + self.payload_length as usize)]
            .copy_from_slice(&self.payload);

        let crc16 = self.calculate_crc16();
        self.buffer[PacketByteLocations::PACKET_PAYLOAD_START_LOC as usize + self.payload_length as usize] = (crc16 >> 8) as u8;
        self.buffer[PacketByteLocations::PACKET_PAYLOAD_START_LOC as usize + self.payload_length as usize + 1] = (crc16 & 0xFF) as u8;

        self.buffer[PacketByteLocations::PACKET_PAYLOAD_START_LOC as usize + self.payload_length as usize + 2] = PACKET_END_BYTE;

        PACKET_HEADER_SIZE + self.payload_length as usize + CRC_LENGTH + PACKET_FOOTER_SIZE
    }

    // Method to send the packet using the provided byte sender function
    pub fn send<F>(&self, send_byte: F)
    where
        F: Fn(u8),
    {
        for byte in &self.buffer {
            send_byte(*byte);
        }
    }
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
