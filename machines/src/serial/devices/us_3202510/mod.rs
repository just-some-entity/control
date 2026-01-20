
use ringbuf::{traits::*, HeapRb};

use control_core::{helpers::hashing::hash_djb2, modbus::{
    ModbusRequest, modbus_serial_interface::ModbusSerialInterface
}};
use serde::{Deserialize, Serialize};
use std::{alloc::Global, collections::BinaryHeap, sync::{Arc, atomic::AtomicBool}, time::Instant};

use smol::lock::RwLock;

use crate::{SerialDevice, SerialDeviceNewParams, extruder1::api::RotationState, machine_identification::DeviceIdentification};

//Note: The communication grid test is modbus rtu with 8 data bits, 1 stop bit, no verification
// and a silent wave holding rate of 9600bps

use register::US3202510Register;
use requests::Request;

mod register;
mod requests;
mod serial_device;
mod process;
mod sync;

#[derive(Debug)]
pub struct US3202510 
{
    pub path: String,
    pub config: Config,
    pub status: Option<Status>,
    
    shutdown_flag: Arc<AtomicBool>,
    
    // will be send to io thread
    requests_queue: BinaryHeap<ModbusRequest>,
    
    
}

// Serialize is needed so we can hash it
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct Config 
{
    pub rotation_state: RotationState,
    pub frequency: units::Frequency, // 1 - 99hz
    pub acceleration_level: u16, // 1 - 15
    pub deceleration_level: u16, // 1 - 15
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct Status
{
    pub voltage: units::ElectricPotential,
    pub current: units::ElectricCurrent,
    pub temperature: units::ThermodynamicTemperature,
    pub operation_status: OperationStatus,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationState
{
    Stopped,
    Forward,
    Reverse,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationStatus
{
    Idle,
    Running,
    Fault,
}


impl SerialDevice for US3202510 {}

impl SerialDeviceNew for US3202510 
{
    fn new_serial(
        params: &SerialDeviceNewParams,
    ) -> Result<(DeviceIdentification, Arc<RwLock<Self>>), anyhow::Error> 
    { 
        let hash = hash_djb2(params.path.as_bytes());
        
        let serial = byte_folding_u16(&hash.to_le_bytes());
        
        let device_identification = DeviceIdentification {
            device_machine_identification: Some(DeviceMachineIdentification {
                machine_identification_unique: MachineIdentificationUnique {
                    machine_identification: MachineIdentification {
                        vendor: VENDOR_QITECH,
                        machine: MACHINE_LASER_V1,
                    },
                    serial,
                },
                role: 0,
            }),
            device_hardware_identification: DeviceHardwareIdentification::Serial(
                DeviceHardwareIdentificationSerial {
                    path: params.path.clone(),
                },
            ),
        };
        
        let rb = HeapRb::<u16>::new(1024);
        let (mut prod, mut cons) = rb.split();
        
        
    }
}

impl US3202510 
{
    pub fn set_rotation_state(&mut self, rotation_state: RotationState)
    {
        self.config.rotation_state = rotation_state;
        
        //TODO: submit request!
    }
    
    pub fn set_frequency_target(&mut self, frequency: units::Frequency)
    {
        self.config.frequency = frequency;
        
        //TODO: submit request!
    }
    
    pub fn set_acceleration_level(&mut self, acceleration_level: u16)
    {
        self.config.acceleration_level = acceleration_level;
        
        //TODO: submit request!
    }
    
    pub fn set_deceleration_level(&mut self, deceleration_level: u16)
    {
        self.config.deceleration_level = deceleration_level;
        
        //TODO: submit request!
    }
}