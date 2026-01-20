
use smol::lock::RwLock;

use control_core::{
    helpers::hashing::{byte_folding_u16, hash_djb2}, 
};
use units::{ConstZero, Frequency};

use anyhow::anyhow;

use super::US3202510;

use crate::{
    MACHINE_PELLET, 
    SerialDevice, 
    SerialDeviceNew, 
    SerialDeviceNewParams, 
    VENDOR_QITECH, 
    machine_identification::{
        DeviceHardwareIdentification, DeviceHardwareIdentificationSerial, DeviceIdentification, DeviceMachineIdentification, MachineIdentification, MachineIdentificationUnique
    }, serial::devices::us_3202510::{Config, RotationState, modbus_rtu_ex}
};

use serialport::ClearBuffer;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};


use std::{sync::Arc, time::Duration};

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
                        machine: MACHINE_PELLET,
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
        
        let port = create_port(&params.path)?;
        
        let interface = modbus_rtu_ex::Interface::<9>::new(port)?;

        let _self = Arc::new(RwLock::new(Self {
            path: params.path.clone(),
            config: Config {
                rotation_state: RotationState::Stopped,
                frequency: Frequency::ZERO,
                acceleration_level: 7,
                deceleration_level: 7,
            },
            status: None,
            failed_attempts: 0,
            interface,
        }));
        
        Ok((device_identification, _self))
    }
}

fn create_port(path: &String) -> Result<Box<dyn SerialPort>, anyhow::Error>
{
    let mut port: Box<dyn SerialPort> = serialport::new(path, 38_400)
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