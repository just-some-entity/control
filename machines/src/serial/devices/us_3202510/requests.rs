use control_core::modbus::ModbusFunctionCode;

use crate::serial::devices::us_3202510::modbus_rtu_ex;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Request
{
    WriteRunningFrequency,
    ReadInverterStatus,
    StopMotor,
    StartForwardRotation,
    StartReverseRotation,
    ReadRunningFrequency,
    ReadMotorStatus,
    ResetInverter,
    WriteParameter,
}

impl Request 
{
    pub fn id(&self) -> usize { *self as usize }
}

pub const REQUEST_REGISTRY: [modbus_rtu_ex::RequestEntry; 9] = [
    modbus_rtu_ex::RequestEntry { // WriteRunningFrequency
        priority: u16::MAX - 1,
        no_response_expected: false,
        payload: modbus_rtu_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::PresetHoldingRegister,
            data: &[0x00, 0x00, 0x0, 0x0], 
        },
    },
    modbus_rtu_ex::RequestEntry { // ReadInverterStatus
        priority: u16::MAX - 6,
        no_response_expected: false,
        payload: modbus_rtu_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::ReadHoldingRegister,
            data: &[0x00, 0x00, 0x00, 0x01], 
        },
    },
    modbus_rtu_ex::RequestEntry { // StopMotor
        priority: u16::MAX,
        no_response_expected: false,
        payload: modbus_rtu_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::PresetHoldingRegister,
            data: &[0x00, 0x00, 0x00, 0x01],
        },
    },
    modbus_rtu_ex::RequestEntry { // StartForwardRotation
        priority: u16::MAX - 1,
        no_response_expected: false,
        payload: modbus_rtu_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::PresetHoldingRegister,
            data: &[0x00, 0x00, 0x00, 0x02],
        },
    },
    modbus_rtu_ex::RequestEntry { // StartReverseRotation
        priority: u16::MAX - 1,
        no_response_expected: false,
        payload: modbus_rtu_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::PresetHoldingRegister,
            data: &[0x00, 0x00, 0x00, 0x04],
        },
    },
    modbus_rtu_ex::RequestEntry { // ReadRunningFrequency
        priority: u16::MAX - 4,
        no_response_expected: false,
        payload: modbus_rtu_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::ReadHoldingRegister,
            data: &[0x00, 0x00, 0x00, 0x01],
        },
    },
    modbus_rtu_ex::RequestEntry { // ReadMotorStatus
        priority: u16::MAX - 2,
        no_response_expected: false,
        payload: modbus_rtu_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::ReadHoldingRegister,
            data: &[0x00, 0x00, 0x00, 0x03],
        },
    },
    modbus_rtu_ex::RequestEntry { // ResetInverter
        priority: u16::MAX,
        no_response_expected: false,
        payload: modbus_rtu_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::PresetHoldingRegister,
            data: &[0x00, 0x00, 0x00, 0x01],
        },
    },
    modbus_rtu_ex::RequestEntry { // WriteParameter
        priority: u16::MAX,
        no_response_expected: false,
        payload: modbus_rtu_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::PresetHoldingRegister,
            data: &[0x00, 0x00, 0x00, 0x00],
        },
    },
];