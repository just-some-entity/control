


pub mod request
{
    #[derive(Debug, Clone)]
    pub struct ReadRegisters
    {
        pub start_address: u16,
        pub quantity:      u16,
    }
    
    #[derive(Debug, Clone)]
    pub struct WriteRegister 
    {
        pub address: u16,
        pub value:   u16,
    }
    
    impl ReadRegisters
    {
        pub fn to_be_bytes(&self) -> [u8; 4]
        {
            let mut result: [u8; 4] = [0; 4];
            
            let address_bytes = self.start_address.to_be_bytes();
            result[0] = address_bytes[0];
            result[1] = address_bytes[1];
            
            let quantity_bytes = self.quantity.to_be_bytes();
            result[2] = quantity_bytes[0];
            result[3] = quantity_bytes[1];
            
            result
        }
    }
    
    impl WriteRegister
    {
        pub fn to_be_bytes(&self) -> [u8; 4]
        {
            let mut result: [u8; 4] = [0; 4];
            
            let address_bytes = self.address.to_be_bytes();
            result[0] = address_bytes[0];
            result[1] = address_bytes[1];
            
            let value_bytes = self.value.to_be_bytes();
            result[2] = value_bytes[0];
            result[3] = value_bytes[1];
            
            result
        }
    }
}

pub mod response
{
    #[derive(Debug, Clone)]
    pub struct Exception
    {
        pub function: u8,
        pub code:     u8,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionCode
{
    /// Read one or more holding registers
    ReadHoldingRegisters = 0x03,
    
    /// Read input registers
    ReadInputRegisters = 0x04,
    
    /// Write one register
    PresetHoldingRegister = 0x06,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExceptionCode 
{
    IllegalFunction                    = 0x01,
    IllegalDataAddress                 = 0x02,
    IllegalDataValue                   = 0x03,
    SlaveDeviceFailure                 = 0x04,
    Acknowledge                        = 0x05,
    SlaveDeviceBusy                    = 0x06,
    NegativeAcknowledge                = 0x07,
    MemoryParityError                  = 0x08,
    GatewayPathUnavailable             = 0x0A,
    GatewayTargetDeviceFailedToRespond = 0x0B,
    Unknown
}

impl ExceptionCode 
{
    pub fn from_int(value: u8) -> ExceptionCode 
    {
        match value 
        {
            0x01 => ExceptionCode::IllegalFunction,
            0x02 => ExceptionCode::IllegalDataAddress,
            0x03 => ExceptionCode::IllegalDataValue,
            0x04 => ExceptionCode::SlaveDeviceFailure,
            0x05 => ExceptionCode::Acknowledge,
            0x06 => ExceptionCode::SlaveDeviceBusy,
            0x07 => ExceptionCode::NegativeAcknowledge,
            0x08 => ExceptionCode::MemoryParityError,
            0x0A => ExceptionCode::GatewayPathUnavailable,
            0x0B => ExceptionCode::GatewayTargetDeviceFailedToRespond,
            _    => ExceptionCode::Unknown,
        }
    }
}



#[derive(Debug, Clone)]
pub enum RequestPayload
{
    // Read one or more holding registers
    ReadHoldingRegisters(request::ReadRegisters),

    // Read input registers
    ReadInputRegisters(request::ReadRegisters),

    // Write one register
    PresetHoldingRegister(request::WriteRegister),
}


#[derive(Debug, Clone)]
pub struct RequestResultData<Request, Result>
{
    pub request: Request,
    pub result:  Result,
}

pub type ReadRequestResult = RequestResultData<request::ReadRegisters, Vec<u16>>;
pub type WriteRequestResult = RequestResultData<request::WriteRegister, ()>;
pub type ExceptionRequestResult = RequestResultData<RequestPayload, ExceptionCode>;

#[derive(Debug, Clone)]
pub enum RequestResult
{
    ReadHoldingRegisters(ReadRequestResult),
    ReadInputRegisters(ReadRequestResult),
    PresetHoldingRegister(WriteRequestResult),
    Exception(ExceptionRequestResult),
}

// [functions]
 
impl FunctionCode
{
    pub const fn from_int(value: u8) -> Option<Self>
    {
        match value 
        {
            0x3 => Some(FunctionCode::ReadHoldingRegisters),
            0x4 => Some(FunctionCode::ReadInputRegisters),
            0x6 => Some(FunctionCode::PresetHoldingRegister),
            _ => None,
        }
    }
}
    
impl RequestPayload
{
    pub const fn function_code(&self) -> FunctionCode
    {
        match self 
        {
            Self::ReadHoldingRegisters(_)  => FunctionCode::ReadHoldingRegisters,
            Self::ReadInputRegisters(_)    => FunctionCode::ReadInputRegisters,
            Self::PresetHoldingRegister(_) => FunctionCode::PresetHoldingRegister,
        }
    }
}