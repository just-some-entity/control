use anyhow::anyhow;
use control_core::modbus::modbus_crc16;



#[derive(Debug, Clone)]
pub enum Payload
{
    /// Read one or more holding registers
    ReadHoldingRegister
    {
        slave_id:      u8,
        start_address: u16,
        quantity:      u16,
    },

    /// Read input registers
    ReadInputRegister 
    {
        slave_id:      u8,
        start_address: u16,
        quantity:      u16,
    },

    /// Write one register
    PresetHoldingRegister 
    {
        slave_id: u8,
        address:  u16,
        value:    u16,
    },

    /// Diagnostics function (echo request)
    Diagnostic
    {
        slave_id:     u8,
        sub_function: u16,
        data:         Vec<u8>,
    },
}
    
enum FunctionCode
{
    ReadHoldingRegister   = 0x03,
    ReadInputRegister     = 0x04,
    PresetHoldingRegister = 0x06,
    Diagnostic            = 0x08,
}
   
impl FunctionCode
{
    pub fn from_int(value: u8) -> Option<Self>
    {
        match value 
        {
            3 => Some(FunctionCode::ReadHoldingRegister),
            4 => Some(FunctionCode::ReadInputRegister),
            6 => Some(FunctionCode::PresetHoldingRegister),
            8 => Some(FunctionCode::Diagnostic),
            _ => None,
        }
    }
}
    
impl Payload
{
    pub fn decode_frame(frame_bytes: &[u8]) -> Result<Self, anyhow::Error> 
    {    
        if frame_bytes.len() <= 4 { return Err(anyhow!("Not enough bytes!")); }
        
        // discard crc bytes
        let payload_bytes = &frame_bytes[..frame_bytes.len() - 2];
        
        let slave_id = payload_bytes[0];

        if let Some(function_code) = FunctionCode::from_int(payload_bytes[1])
        {
            match function_code
            {
                FunctionCode::ReadHoldingRegister =>
                {
                    debug_assert!(payload_bytes.len() == 6);
                    let start_address = u16::from_be_bytes([payload_bytes[2], payload_bytes[3]]);
                    let quantity      = u16::from_be_bytes([payload_bytes[4], payload_bytes[5]]);
                    Ok(Payload::ReadHoldingRegister { slave_id, start_address, quantity })
                }
                FunctionCode::ReadInputRegister =>
                {
                    debug_assert!(payload_bytes.len() == 6);
                    let start_address = u16::from_be_bytes([payload_bytes[2], payload_bytes[3]]);
                    let quantity      = u16::from_be_bytes([payload_bytes[4], payload_bytes[5]]);
                    Ok(Payload::ReadInputRegister { slave_id, start_address, quantity })
                }
                FunctionCode::PresetHoldingRegister =>
                {
                    debug_assert!(payload_bytes.len() == 6);
                    let address = u16::from_be_bytes([payload_bytes[2], payload_bytes[3]]);
                    let value   = u16::from_be_bytes([payload_bytes[4], payload_bytes[5]]);
                    Ok(Payload::PresetHoldingRegister { slave_id, address, value })
                }
                FunctionCode::Diagnostic =>
                {
                    let sub_function = u16::from_be_bytes([payload_bytes[2], payload_bytes[3]]);
                    let data = payload_bytes[4..].to_vec();
                    Ok(Payload::Diagnostic { slave_id, sub_function, data })
                }
            }
        }
        
        else { Err(anyhow!("Unsupported function code: {}", payload_bytes[1])) }
    }
    
    /// Convert to a Modbus RTU frame (without CRC)
    pub fn encode_frame<'b>(&self, buf: &'b mut [u8; 256]) -> &'b [u8] 
    {
        let mut i = 0;
        match self 
        {
            Payload::ReadHoldingRegister { slave_id, start_address, quantity } => 
            {
                buf[i] = *slave_id; i += 1;
                buf[i] = 0x03; i += 1;
                buf[i..i+2].copy_from_slice(&start_address.to_be_bytes()); i += 2;
                buf[i..i+2].copy_from_slice(&quantity.to_be_bytes()); i += 2;
            }
            
            Payload::ReadInputRegister { slave_id, start_address, quantity } => 
            {
                buf[i] = *slave_id; i += 1;
                buf[i] = 0x04; i += 1;
                buf[i..i+2].copy_from_slice(&start_address.to_be_bytes()); i += 2;
                buf[i..i+2].copy_from_slice(&quantity.to_be_bytes()); i += 2;
            }
            
            Payload::PresetHoldingRegister { slave_id, address, value } => 
            {
                buf[i] = *slave_id; 
                i += 1;
                buf[i] = 0x06; 
                i += 1;
                buf[i..i+2].copy_from_slice(&address.to_be_bytes()); 
                i += 2;
                buf[i..i+2].copy_from_slice(&value.to_be_bytes()); 
                i += 2;
            }
            
            Payload::Diagnostic { slave_id, sub_function, data } => 
            {
                buf[i] = *slave_id; 
                i += 1;
                buf[i] = 0x08; 
                i += 1;
                buf[i..i+2].copy_from_slice(&sub_function.to_be_bytes()); i += 2;
                buf[i..i+data.len()].copy_from_slice(data); 
                i += data.len();
            }
        }

        let crc = modbus_crc16(&buf[..i]).to_le_bytes();
        
        buf[i] = crc[0];
        i += 1;
        buf[i] = crc[1];
        i += 1;

        &buf[..i]
    }
}