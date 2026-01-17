use anyhow::anyhow;
use control_core::helpers::hashing::{byte_folding_u16, hash_djb2};
use control_core::helpers::retry::retry_n_times;
use control_core::modbus::ModbusResponse;
use control_core::modbus::modbus_serial_interface::ModbusSerialInterface;
use control_core::modbus::{self, ModbusRequest};
use serialport::SerialPort;
use serialport::{ClearBuffer, DataBits, FlowControl, Parity, StopBits};
use smol::lock::RwLock;
use units::frequency::hertz;
use std::collections::HashMap;
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

mod backend;

#[derive(Debug)]
pub struct US3202510 {
    pub path: String,
    
    pub config: Config,
    pub status: Option<Status>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    frequency: Frequency,
    run_state: RunState,
    acceleration_speed_step: u16, // 1 - 15
    deceleration_speed_step: u16, // 1 - 15
}

#[derive(Debug, Clone, Copy)]
pub struct Status {
    pub frequency: Frequency,
    pub current: ElectricCurrent,
    pub voltage: ElectricPotential,
    pub temperature: ThermodynamicTemperature,
    pub system_status: SystemStatus,
    pub error_code: ErrorCode
}

#[derive(Debug, Clone, Copy)]
pub enum RunState {
    Stopped,
    Foward,
    Reverse,
}

#[derive(Debug, Clone, Copy)]
pub enum SystemStatus {
    Idle,
    Running,
    Fault,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode
{
    None,
    PulseOvercurrent,
    IgbtOvercurrentProtection,
    DcBusOvervoltageProtection,
    TemperatureNearIgbtLimit,
    InverterThermalProtection,
    InverterOverload100Percent,
    InverterPowerCut,
}

#[derive(Debug, Clone)]
pub struct US3202510RequestData {
    request: ModbusRequest,
    control_request_type: MitsubishiCS80Request,
    request_type: RequestType,
    priority: u16,
}

enum US3202510Request {
    None,
    StopMotor,
    StartForwardRotation,
    StartReverseRotation,
    
    ReadRunningFrequency,
    WriteRunningFrequency,
    
    ReadMotorStatus,
}

enum US3202510Register {
    
    // Holding Registers
    
    /// Register 0x0002
    SetFrequency,

    /// Register 0x0003
    RunCommand,

    /// Register 0x0004
    AccelerationTime,

    /// Register 0x0005
    DeacelerationTime,


    // Input Registers

    /// Register 0x0008
    BusVoltage,

    /// Register 0x0009
    LineCurrent,

    /// Register 0x000A
    DriveTemperature,

    /// Register 0x000B
    SystemStatus,

    /// Register 0x000C
    ErrorCode,

    /// Register 0x000D
    CurrentOperatingFrequency,
}

impl US3202510Register {
    const fn address(self) -> u16
    {
        match self {
            // Holding Registers
            Self::SetFrequency => 0x0002,
            Self::RunCommand => 0x0003,
            Self::AccelerationTime => 0x0004,
            Self::DeacelerationTime => 0x0005,
            
            // Input Registers
            Self::BusVoltage => 0x0008,
            Self::LineCurrent => 0x0009,
            Self::DriveTemperature => 0x000A,
            Self::SystemStatus => 0x000B,
            Self::ErrorCode => 0x000C,
            Self::CurrentOperatingFrequency => 0x000D,
        }
    }

    const fn address_be_bytes(self) -> [u8; 2] {
        self.address().to_be_bytes()
    }
}

impl From<US3202510Request> for modbus::ModbusRequest {
    fn from(request: US3202510Request) -> Self {
        
        match request {
            US3202510Request::StopMotor => Self {
                slave_id: 1,
                function_code: modbus::ModbusFunctionCode::ReadMotorStatus,
                data: vec![
                    0x00, 0x0E, // Start register = 0x000E
                    0x00, 0x03, // Read 3 registers (AvgDiameter, X, Y)
                ],
            },
        }
    }
}

impl US3202510 { 
    pub async fn get_config(&self) -> Config {
        self.config.clone()
    }
    
    pub async fn get_status(&self) -> Option<Status> {
        self.status.clone()
    }
}

impl SerialDevice for US3202510 {}

impl SerialDeviceNew for US3202510 {
    fn new_serial(
        params: &SerialDeviceNewParams,
    ) -> Result<(DeviceIdentification, Arc<RwLock<Self>>), anyhow::Error> {
        
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
        
        let _self = Arc::new(RwLock::new(Self {
            path: params.path.clone(),
            config: Config { 
                frequency: Frequency::new::<hertz>(0.0), 
                run_state: RunState::Stopped, 
                acceleration_speed_step: 7, 
                deceleration_speed_step: 7,
            },
            status: None,
            
            
        }));
        
        // let _self_clone = _self.clone();
        
        // let _ = thread::Builder::new()
        //     .name("vfd".to_owned())
        //     .spawn(move || {
        //         smol::block_on(async {
        //             let _ = Self::process(_self_clone).await;
        //         });
        //     })?;

        Ok((device_identification, _self))
    }
}

// impl Drop for US3202510 {
//     fn drop(&mut self) {
//         // Signal shutdown
//         self.shutdown_flag.store(true, Ordering::SeqCst);
//         println!("Laser struct dropped, thread stopped");
//     }
// }