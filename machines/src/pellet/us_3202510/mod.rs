use bitvec::{order::Lsb0, slice::BitSlice};
use control_core::modbus::{
    ModbusFunctionCode, ModbusRequest, ModbusResponse,
    modbus_serial_interface::ModbusSerialInterface,
};
use ethercat_hal::io::serial_interface::SerialInterface;
use serde::Serialize;
use std::time::{Duration, Instant};
use units::electric_current::centiampere;
use units::electric_potential::centivolt;
use units::f64::*;
use units::frequency::centihertz;

//Note: The communication grid test is modbus rtu with 8 data bits, 1 stop bit, no verification
// and a silent wave holding rate of 9600bps

/// Specifies all System environment Variables
#[derive(Debug, Clone, Copy)]
enum US3202510Register {

    /// Register 0x0002
    SetFrequency,

    /// Register 0x0003
    RunCommand,

    /// Register 0x0004
    AccelerationTime,

    /// Register 0x0005
    DeacelerationTime,

    /// Register 0x0008
    BusVoltage,

    /// Register 0x0009
    LineCurrent,

    /// Register 0x000A
    DriveTemperature,

    /// Register 0x000B
    SystemStatus,

    /// Register 0x000C
    ErrorCode,

    /// Register 0x000D
    CurrentOperatingFrequency,
}

impl US3202510Register {
    const fn address(self) -> u16 {
        match self {
            Self::HoldRegisterBank => 0x2,
            Self::InputRegisterBank => 0x8,
        }
    }

    const fn address_be_bytes(self) -> [u8; 2] {
        self.address().to_be_bytes()
    }
}

/// These Requests Serve as Templates for controlling the inverter
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum US3202510Requests {
    None,
}

impl From<US3202510Requests> for u16 {
    fn from(request: US3202510Requests) -> Self {
        request as Self
    }
}

impl TryFrom<u16> for US3202510Requests {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            _ => Err(()),
        }
    }
}

/*
    MitsubishiModbusRequest get executed by their priority
    Start and StopMotor are highest priority while writeRunningFrequency and readMotorFrequency are one lower
    lets say we had StartMotor and readMotorFrequency the order of execution is:
    1. StartMotor
    2. readMotorFrequency

    this is because StartMotor is higher priority
    Since the events do not need to be pushed into a queue this makes the inverter operation more stable

    Lets say one request "A" with priority 1 and one with 2 "B" are queued up, assume that request B is frequently used
    1. Request "B" is executed due to higher priority
    2. When B is added again request A has the same priority because it was ignored. B is executed once again
    3. B is added again, now A has an effective priority of 3, which is higher then B
*/
impl From<US3202510Requests> for US3202510Request {
    fn from(request: US3202510Requests) -> Self {
        match request {
            US3202510Requests::WriteRunningFrequency => {
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

// Serialize is needed so we can hash it
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct US3202510Status {

    pub rotation_state: u16, // 1 = forward, 2 = stop, 3 = reverse
    pub frequency: u16,

    pub acceleration_time: u16, // 1 - 15
    pub deceleration_time: u16, // 1 - 15
}

#[derive(Debug, Clone, Copy)]
pub enum RequestType {
    None,
    /// Monitoring, Operation (start,stop etc) command, frequency setting (RAM), less than 12 milliseconds timeout for Response
    OperationCommand,
    /// Parameter Read/Write and Frequency (EEPROM), Less than 30 milliseconds timeout for Response
    ReadWrite,
    /// Less than 5 seconds timeout for Response
    ParamClear,
    /// Supposedly no waiting time, however inverter takes a while to start ~300ms should be more than enough
    Reset,
}

impl RequestType {
    const fn timeout_duration(self) -> Duration {
        match self {
            Self::OperationCommand => Duration::from_millis(12),
            Self::ReadWrite => Duration::from_millis(30),
            Self::ParamClear => Duration::from_millis(5000),
            Self::Reset => Duration::from_millis(900),
            Self::None => Duration::from_millis(12),
        }
    }
}

// We need to know from the request queue which events are of what operation type, so that the correct timeout can be used
#[derive(Debug, Clone)]
pub struct US3202510Request {
    request: ModbusRequest,
    control_request_type: US3202510Requests,
    request_type: RequestType,
    priority: u16,
}

impl US3202510Request {
    const fn new(
        request: ModbusRequest,
        control_request_type: US3202510Requests,
        request_type: RequestType,
        priority: u16,
    ) -> Self {
        Self {
            request,
            control_request_type,
            request_type,
            priority,
        }
    }
}

#[derive(Debug)]
pub struct US3202510 {
    pub last_ts: Instant,
    pub modbus_serial_interface: ModbusSerialInterface,

    pub status: US3202510Status,
}

impl US3202510 {
    pub fn stop_motor(&mut self) {
        self.add_request(MitsubishiCS80Requests::StopMotor.into());
    }
}