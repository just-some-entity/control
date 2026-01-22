// External deps
use crc::{CRC_16_MODBUS, Crc};

use crate::serial::interfaces::modbus_rtu::structs::ExceptionCode;

// Internal deps
use super::{FRAME_SIZE_MAX, structs::RequestPayload, FunctionCode};


// [structs]

#[derive(Debug, Clone)]
pub struct Frame
{
    pub buf: [u8; FRAME_SIZE_MAX],
    pub len: usize,
}

#[derive(Debug, Clone)]
pub enum FrameParseError
{
    TooSmall,
    UndefinedFunctionCode,
    InvalidCRC,
}

// [implementations]

impl Frame
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, FrameParseError>
    {
        if bytes.len() < 5 { return Err(FrameParseError::TooSmall); }
        
        match FunctionCode::from_int(bytes[1]) 
        {
            Some(_) => {},
            None => 
            {
                tracing::error!("Failed to parse: {} | {:?}", bytes[1], &bytes);
                return Err(FrameParseError::UndefinedFunctionCode);
            }   
        };
        
        let crc_bytes = Self::compute_crc(&bytes[..bytes.len() - 2]).to_le_bytes();
        
        if bytes[bytes.len() - 2] != crc_bytes[0] { return Err(FrameParseError::InvalidCRC); }
        
        if bytes[bytes.len() - 1] != crc_bytes[1] { return Err(FrameParseError::InvalidCRC); }
        
        let mut result = Frame { buf: [0; FRAME_SIZE_MAX], len: 0 };
        
        result.buf[..bytes.len()].copy_from_slice(bytes);

        result.len = bytes.len();

        Ok(result)
    }
    
    pub fn from_request(slave_id: u8, payload: &RequestPayload) -> Self
    {
        let mut result = Frame { buf: [0; FRAME_SIZE_MAX], len: 0 };
        
        result.buf[0] = slave_id;
        result.buf[1] = payload.function_code() as u8;
        result.len   += 2;
        
        match payload 
        {
            RequestPayload::ReadHoldingRegisters(payload) => 
            {
                let bytes = payload.to_be_bytes();
                result.buf[result.len]     = bytes[0];
                result.buf[result.len + 1] = bytes[1];
                result.buf[result.len + 2] = bytes[2];
                result.buf[result.len + 3] = bytes[3];
                result.len += 4;
                
            },
            RequestPayload::ReadInputRegisters(payload) => 
            {
                let bytes = payload.to_be_bytes();
                result.buf[result.len]     = bytes[0];
                result.buf[result.len + 1] = bytes[1];
                result.buf[result.len + 2] = bytes[2];
                result.buf[result.len + 3] = bytes[3];
                result.len += 4;
            },
            RequestPayload::PresetHoldingRegister(payload) => 
            {
                let bytes = payload.to_be_bytes();
                result.buf[result.len]     = bytes[0];
                result.buf[result.len + 1] = bytes[1];
                result.buf[result.len + 2] = bytes[2];
                result.buf[result.len + 3] = bytes[3];
                result.len += 4;
            },
        }
        
        let crc_bytes = Self::compute_crc(&result.buf[0..result.len]).to_le_bytes();
        result.buf[result.len    ] = crc_bytes[0];
        result.buf[result.len + 1] = crc_bytes[1];
        result.len += 2;
        
        result
    }
    
    pub fn len(&self) -> usize
    {
        self.len
    }
    
    pub fn bytes(&self) -> &[u8]
    {
        &self.buf[..self.len]
    }
    
    pub fn data(&self) -> &[u8]
    {
        &self.buf[2..self.len - 2]
    }
    
    pub const fn slave_id(&self) -> u8
    {
        self.buf[0]
    }
    
    pub fn function_code(&self) -> FunctionCode
    {
        let code = self.buf[1];
        // Mask out MSB if it’s an exception to get original function code
        // A frame can only be created with a valid function code
        // so if this fails, the data IS corrupted
        FunctionCode::from_int(code & 0x7F).expect("Frame corrupted")
    }
    
    /// Returns true if this response is an exception
    pub fn is_exception(&self) -> bool 
    {
        self.buf[1] & 0x80 != 0
    }
    
    pub fn exception_code(&self) -> Option<ExceptionCode> 
    {
        if self.is_exception() 
        {
            self.buf.get(2).copied().map(ExceptionCode::from_int)
        } 
        
        else { None }
    }
    
    pub const fn crc(&self) -> u16
    {
        ((self.buf[1] as u16) << 8) | (self.buf[0] as u16)
    }
    
    pub fn is_valid(&self) -> bool
    {
        let actual_crc = Self::compute_crc(&self.buf[..self.len]);
        actual_crc == self.crc()
    }
    
    pub const fn compute_crc(data: &[u8]) -> u16 
    {
        let modbus: Crc<u16> = Crc::<u16>::new(&CRC_16_MODBUS);
        modbus.checksum(data)
    }
}