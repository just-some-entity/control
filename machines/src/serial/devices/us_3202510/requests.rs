#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Request
{
    None,
}

impl From<Request> for u16 
{
    fn from(request: Request) -> Self 
    {
        request as Self
    }
}

impl TryFrom<u16> for Request 
{
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> 
    {
        match value {
            0 => Ok(Self::None),
            _ => Err(()),
        }
    }
}

/*
impl From<Request> for Request 
{
    fn from(request: Request) -> Self 
    {
        match request 
        {
            US3202510Requests::WriteRunningFrequency => 
            {
                let reg_bytes = US3202510Register::RunningFrequencyRAM.address_be_bytes();
                Self::new(
                    ModbusRequest {
                        slave_id: 1,
                        function_code: ModbusFunctionCode::PresetHoldingRegister,
                        data: vec![reg_bytes[0], reg_bytes[1], 0x0, 0x0],
                    },
                    request,
                    RequestType::OperationCommand,
                    u16::MAX - 1,
                )
            }

            US3202510Requests::ReadInverterStatus => {
                let reg_bytes = US3202510Register::InverterStatusAndControl.address_be_bytes();
                Self::new(
                    ModbusRequest {
                        slave_id: 1,
                        function_code: ModbusFunctionCode::ReadHoldingRegister,
                        data: vec![reg_bytes[0], reg_bytes[1], 0x00, 0x01], // Read 1 register
                    },
                    request,
                    RequestType::OperationCommand,
                    u16::MAX - 6, // Priority is -6 because we do not want to know the status as frequently as the frequency
                )
            }
            US3202510Requests::StopMotor => {
                let reg_bytes = US3202510Register::InverterStatusAndControl.address_be_bytes();
                Self::new(
                    ModbusRequest {
                        slave_id: 1,
                        function_code: ModbusFunctionCode::PresetHoldingRegister,
                        data: vec![reg_bytes[0], reg_bytes[1], 0x00, 0x01], // Value 1 to stop
                    },
                    request,
                    RequestType::OperationCommand,
                    u16::MAX, // StopMotor should have highest priority
                )
            }
            US3202510Requests::StartForwardRotation => {
                let reg_bytes = US3202510Register::InverterStatusAndControl.address_be_bytes();
                Self::new(
                    ModbusRequest {
                        slave_id: 1,
                        function_code: ModbusFunctionCode::PresetHoldingRegister,
                        data: vec![reg_bytes[0], reg_bytes[1], 0, 0b00000010], // Value 2 for forward rotation
                    },
                    request,
                    RequestType::OperationCommand,
                    u16::MAX - 1,
                )
            }
            US3202510Requests::StartReverseRotation => {
                let reg_bytes = US3202510Register::InverterStatusAndControl.address_be_bytes();
                Self::new(
                    ModbusRequest {
                        slave_id: 1,
                        function_code: ModbusFunctionCode::PresetHoldingRegister,
                        data: vec![reg_bytes[0], reg_bytes[1], 0, 0b00000100], // Value 4 for reverse rotation
                    },
                    request,
                    RequestType::OperationCommand,
                    u16::MAX - 1,
                )
            }
            US3202510Requests::ReadRunningFrequency => {
                let reg_bytes = US3202510Register::RunningFrequencyRAM.address_be_bytes();
                Self::new(
                    ModbusRequest {
                        slave_id: 1,
                        function_code: ModbusFunctionCode::ReadHoldingRegister,
                        data: vec![reg_bytes[0], reg_bytes[1], 0x00, 0x01], // Read 1 register
                    },
                    request,
                    RequestType::OperationCommand,
                    u16::MAX - 4,
                )
            }
            US3202510Requests::ReadMotorStatus => {
                let reg_bytes = US3202510Register::MotorStatus.address_be_bytes();
                Self::new(
                    ModbusRequest {
                        slave_id: 1,
                        function_code: ModbusFunctionCode::ReadHoldingRegister,
                        data: vec![reg_bytes[0], reg_bytes[1], 0x0, 0x3], // read 3 registers: 0x00C8 = frequency , 0x00C9 = current ,0x00C10 = voltage
                    },
                    request,
                    RequestType::OperationCommand,
                    u16::MAX - 2,
                )
            }
            US3202510Requests::ResetInverter => {
                let reg_bytes = US3202510Register::InverterReset.address_be_bytes();
                Self::new(
                    ModbusRequest {
                        slave_id: 1,
                        function_code: ModbusFunctionCode::PresetHoldingRegister,
                        data: vec![reg_bytes[0], reg_bytes[1], 0x0, 0b00000001],
                    },
                    request,
                    RequestType::Reset,
                    u16::MAX,
                )
            }
            US3202510Requests::WriteParameter => Self::new(
                ModbusRequest {
                    slave_id: 1,
                    function_code: ModbusFunctionCode::PresetHoldingRegister,
                    data: vec![0x0, 0x0, 0x0, 0x0],
                },
                request,
                RequestType::ReadWrite,
                u16::MAX,
            ),

            // For unimplemented variants, return a default request
            _ => Self::new(
                ModbusRequest {
                    slave_id: 1,
                    function_code: ModbusFunctionCode::ReadHoldingRegister,
                    data: vec![0x0, 0x0, 0x0, 0x1],
                },
                request,
                RequestType::None,
                0,
            ),
        }
    }
}
*/