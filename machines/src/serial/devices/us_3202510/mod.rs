use control_core::modbus::{ModbusFunctionCode, ModbusResponse};
use serde::{Deserialize, Serialize};

//Note: The communication grid test is modbus rtu with 8 data bits, 1 stop bit, no verification
// and a silent wave holding rate of 9600bps

mod modbus_ex;

mod register;
mod requests;
mod serial_device;

#[derive(Debug)]
pub struct US3202510 
{
    pub path: String,
    pub config: Config,
    pub status: Option<Status>,
    
    interface: modbus_ex::Interface<1>,
}

pub const REQUESTS: [modbus_ex::RequestEntry; 1] = [
    modbus_ex::RequestEntry {
        priority: 10,
        no_response_expected: false,
        payload: modbus_ex::Request { 
            slave_id: 1,
            function_code: ModbusFunctionCode::ReadInputRegister,
            data: &[0x0, 0x0, 0x0, 0x0],
        },
    },
];

#[derive(Debug, Default, Clone, Copy)]
pub struct Config 
{
    pub rotation_state: RotationState,
    pub frequency: units::Frequency, // 1 - 99hz
    pub acceleration_level: u16, // 1 - 15
    pub deceleration_level: u16, // 1 - 15
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Status
{
    pub voltage: units::ElectricPotential,
    pub current: units::ElectricCurrent,
    pub temperature: units::ThermodynamicTemperature,
    pub operation_status: OperationStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RotationState
{
    #[default]
    Stopped,
    Forward,
    Reverse,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperationStatus
{
    #[default]
    Idle,
    Running,
    Fault,
}

impl US3202510 
{
    pub fn set_rotation_state(&mut self, rotation_state: RotationState)
    {
        self.config.rotation_state = rotation_state;
        self.interface.queue_request(0);
    }
    
    pub fn set_frequency_target(&mut self, frequency: units::Frequency)
    {
        self.config.frequency = frequency;
        self.interface.queue_request(1);
    }
    
    pub fn set_acceleration_level(&mut self, acceleration_level: u16)
    {
        self.config.acceleration_level = acceleration_level;
        self.interface.queue_request(2);
    }
    
    pub fn set_deceleration_level(&mut self, deceleration_level: u16)
    {
        self.config.deceleration_level = deceleration_level;
        self.interface.queue_request(3);
    }
    
    pub fn update(&mut self)
    {
        if let Some(response) = self.interface.poll_response()
        {
            self.handle_response(response);
        }
        
        _ = self.interface.send_request();
    }
    
    fn handle_response(&mut self, response: ModbusResponse)
    {
        //TODO: process response
    }
}