// external deps
use serde::{Deserialize, Serialize};

// internal deps
pub use request::Request;
pub use register::Register;

use crate::serial::devices::us_3202510::modbus_rtu_ex::Payload;

// modules
mod register;
mod request;
mod serial_device;

mod modbus_rtu_ex; // TODO: move out of here

#[derive(Debug)]
pub struct US3202510
{
    pub path: String,
    pub config: Config,
    pub status: Option<Status>,
    
    failed_attempts: u8,
    interface: modbus_rtu_ex::Interface<9>,
}

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
    pub frequency: units::Frequency, // 1 - 99hz
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
    pub fn update(&mut self)
    {
        if let Some(response) = self.interface.check_response()
        {

            self.handle_response(response);
        }
        
        self.refresh_status();

        if self.interface.is_ready_to_send()
        {
            _ = self.interface.send_next_request(); 
        }
    }
    
    fn queue_request(&mut self, request: Request)
    {
        self.interface.queue_request(request.to_interface_request());
    }
    
    pub fn refresh_status(&mut self)
    {
        self.queue_request(Request::RefreshStatus);
    }

    pub fn set_rotation_state(&mut self, rotation_state: RotationState)
    {
        self.config.rotation_state = rotation_state;
        self.queue_request(Request::StartForwardRotation);
    }
    
    pub fn set_frequency_target(&mut self, frequency: units::Frequency)
    {
        self.config.frequency = frequency;
        self.queue_request(Request::StartForwardRotation);
    }
    
    pub fn set_acceleration_level(&mut self, acceleration_level: u16)
    {
        self.config.acceleration_level = acceleration_level;
        self.queue_request(Request::StartForwardRotation);
    }
    
    pub fn set_deceleration_level(&mut self, deceleration_level: u16)
    {
        self.config.deceleration_level = deceleration_level;
        self.queue_request(Request::StartForwardRotation);
    }
    
    fn handle_response(&mut self, response: Payload)
    {
        tracing::error!("Got Response!");

        //TODO: process response
        
        _ = response;
    }
}

#[cfg(test)]
mod tests 
{
    #[test]
    fn test_requests() 
    {
        assert!(1 == 2);
    }
}