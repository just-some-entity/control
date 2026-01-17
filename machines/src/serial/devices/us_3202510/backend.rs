use anyhow::{Ok, anyhow};
use control_core::helpers::hashing::{byte_folding_u16, hash_djb2};
use control_core::helpers::retry::retry_n_times;
use control_core::modbus::ModbusResponse;
use control_core::modbus::{self, ModbusRequest};
use serialport::SerialPort;
use serialport::{ClearBuffer, DataBits, FlowControl, Parity, StopBits};
use smol::lock::RwLock;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::{
    io::Write,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use units::f64::Frequency;
use units::f64::ElectricCurrent;
use units::f64::ElectricPotential;
use units::f64::ThermodynamicTemperature;

use crate::machine_identification::{
    DeviceHardwareIdentification, DeviceHardwareIdentificationSerial, DeviceIdentification,
    DeviceMachineIdentification, MachineIdentification, MachineIdentificationUnique,
};
use crate::{
    MACHINE_LASER_V1, SerialDevice, SerialDeviceNew, SerialDeviceNewParams, VENDOR_QITECH,
};

#[derive(Debug)]
struct ModbusSerialBridge
{
    port:  Box<dyn SerialPort>,
    queue: HashMap<u32, Request>,
}

struct Backend 
{
    instance: Arc<RwLock<super::US3202510>>,
    keep_alive: Arc<AtomicBool>
}

impl Backend
{
    pub fn run(instance: Arc<RwLock<super::US3202510>>) -> Result<(), anyhow::Error>
    {
        let _ = thread::Builder::new()
            .name("backend_ID".to_owned())
            .spawn(move || {
                smol::block_on(async {
                    let _ = Self::process(instance).await;
                });
            })?;
        
        Ok(())
    }
    
    async fn process(instance: Arc<RwLock<super::US3202510>>) -> Result<(), anyhow::Error>
    {
        const BUF_SIZE: usize = size_of::<ModbusRequest>();
        const BAUD_RATE: u32 = 9_600;
        const TIMEOUT_MILLIS: u64 = 500;
        
        let path = {
            let read_guard = instance.read().await;
            read_guard.path.clone()
        };
        
        let mut request_buffer: [u8; BUF_SIZE] = [0; BUF_SIZE];
        
        let port = Backend::port_create(path);
        
        
        
        Ok(())
    }
    
    fn port_create(path: String) -> Result<Box<dyn SerialPort>, anyhow::Error>
    {
        let mut port: Box<dyn SerialPort> = serialport::new(&path, 38_400)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .timeout(Duration::from_millis(500)) // start with something forgiving
            .open()
            .map_err(|e| anyhow!("Failed to open port {}: {}", path, e))?;
            
        port.write_data_terminal_ready(true).ok();
        port.write_request_to_send(true).ok();

        port.clear(ClearBuffer::All).ok();
        
        Ok(port)
    }
}

#[derive(Debug)]
pub struct Request
{
    id: u32,
    priority: u32,
    response_expected: bool,
    payload: ModbusRequest,
    delay: Option<u32>,
}

impl ModbusSerialBridge
{
    pub fn new(port: Box<dyn SerialPort>) -> Self
    {
        Self {
            port,
            queue: HashMap::new(),
        }
    }
    
    fn submit_request(&mut self, request: Request) 
    {
        
    }
    
    fn port_new(path: String) -> Result<Box<dyn SerialPort>, anyhow::Error>
    {
        let mut port: Box<dyn SerialPort> = serialport::new(&path, 38_400)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .timeout(Duration::from_millis(500)) // start with something forgiving
            .open()
            .map_err(|e| anyhow!("Failed to open port {}: {}", path, e))?;
            
        port.write_data_terminal_ready(true).ok();
        port.write_request_to_send(true).ok();

        port.clear(ClearBuffer::All).ok();
        
        Ok(port)
    }
}