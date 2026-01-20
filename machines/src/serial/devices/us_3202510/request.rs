use crate::serial::devices::us_3202510::{Register, modbus_rtu_ex};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Request
{
    WriteRunningFrequency(u16),
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
    pub fn to_interface_request(&self) -> modbus_rtu_ex::InterfaceRequest
    {
        match self
        {
            Request::WriteRunningFrequency(frequency) => 
            {
                let payload = modbus_rtu_ex::Payload::PresetHoldingRegister {
                    slave_id: 0,
                    address: Register::SetFrequency.address(),
                    value: *frequency,
                };
                
                modbus_rtu_ex::InterfaceRequest {
                    type_id:  0,
                    priority: 999,
                    payload,
                    delay:    None,
                }
            },
            Request::ReadInverterStatus => todo!(),
            Request::StopMotor => todo!(),
            Request::StartForwardRotation => todo!(),
            Request::StartReverseRotation => todo!(),
            Request::ReadRunningFrequency => todo!(),
            Request::ReadMotorStatus => todo!(),
            Request::ResetInverter => todo!(),
            Request::WriteParameter => todo!(),
        }
    }
}