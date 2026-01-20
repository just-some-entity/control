use anyhow::Ok;
use anyhow::anyhow;
use serialport::ClearBuffer;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use smol::lock::RwLock;

use control_core::{
    helpers::hashing::{byte_folding_u16, hash_djb2}, 
};
use units::{ConstZero, Frequency};

use super::US3202510;

use crate::serial::devices::us_3202510::sync::Channel;
use crate::{
    MACHINE_PELLET, 
    SerialDevice, 
    SerialDeviceNew, 
    SerialDeviceNewParams, 
    VENDOR_QITECH, 
    machine_identification::{
        DeviceHardwareIdentification, DeviceHardwareIdentificationSerial, DeviceIdentification, DeviceMachineIdentification, MachineIdentification, MachineIdentificationUnique
    }, serial::devices::us_3202510::Config
};

use std::sync::atomic::Ordering;
use std::time::Instant;
use std::{sync::Arc, time::Duration};

pub async fn run(path: String, channel: Arc<RwLock<Channel>>) -> Result<(), anyhow::Error>
{
    let mut port = create_port(path)?;
    
    let target = Duration::from_secs_f64(1.0 / 120.0);
    let mut next = Instant::now();
    
    while !_self.read().await.shutdown_flag.load(Ordering::SeqCst)
    {
        
        
        next += target;
        let now = Instant::now();
        if next > now
        {
            std::thread::sleep(next - now);
        }
    }
    
    Ok(())
}

pub fn create_port(path: String) -> Result<Box<dyn SerialPort>, anyhow::Error>
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
    
    return Ok(port);
}