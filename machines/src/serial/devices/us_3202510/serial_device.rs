
use smol::lock::RwLock;

use control_core::{
    helpers::hashing::{byte_folding_u16, hash_djb2}, 
};
use units::{ConstZero, Frequency};

use super::US3202510;

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

use std::{sync::Arc};

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
            
        let _self = Arc::new(RwLock::new(Self {
            path: params.path.clone(),
            config: Config {
                rotation_state: super::RotationState::Stopped,
                frequency: Frequency::ZERO,
                acceleration_level: 7,
                deceleration_level: 7,
            },
            status: None,
        }));
        
        Ok((device_identification, _self))
    }
}