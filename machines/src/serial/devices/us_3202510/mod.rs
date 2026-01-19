
use control_core::{helpers::hashing::hash_djb2, modbus::{
    modbus_serial_interface::ModbusSerialInterface,
}};
use serde::Serialize;
use std::{sync::Arc, time::{Instant}};

use smol::lock::RwLock;

use crate::{SerialDevice, SerialDeviceNewParams, machine_identification::DeviceIdentification};

//Note: The communication grid test is modbus rtu with 8 data bits, 1 stop bit, no verification
// and a silent wave holding rate of 9600bps

use register::US3202510Register;
use requests::Request;

mod register;
mod requests;

#[derive(Debug)]
pub struct US3202510 
{
    pub last_ts: Instant,
    pub modbus_serial_interface: ModbusSerialInterface,

    pub config: Config,
}

// Serialize is needed so we can hash it
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct Config 
{
    pub rotation_state: u16, // 1 = forward, 2 = stop, 3 = reverse
    pub frequency: units::Frequency,

    pub acceleration_time: u16, // 1 - 15
    pub deceleration_time: u16, // 1 - 15
}

impl SerialDevice for US3202510 {}

impl SerialDeviceNew for US3202510 
{
    fn new_serial(
        params: &SerialDeviceNewParams,
    ) -> Result<(DeviceIdentification, Arc<RwLock<Self>>), anyhow::Error> 
    { 
        let hash = hash_djb2(params.path.as_bytes());
    }
}

impl US3202510 
{
    pub fn set_run_state() 
    {

    }

    pub fn stop_motor(&mut self) 
    {
        self.add_request(MitsubishiCS80Requests::StopMotor.into());
    }
}