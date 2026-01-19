use std::{collections::HashMap, process::id};
use core::mem::MaybeUninit;



const MAX_REQUESTS: usize = 8192;

pub struct ModbusSerialRequest
{
    
}

pub struct ModbusSerialResponse
{
    
}

pub struct ModbusSerialHandler 
{
    id_pool:     IdPool,   
    devices:     Vec<ModbusSerialDeviceInterface>, 
    devices_lto: HashMap<u32, usize>,
}

impl ModbusSerialHandler
{
    pub fn update(&mut self)
    {
        
    }
    
    pub fn register_device(&mut self, kind: ModbusSerialDeviceKind) -> Result<u32, anyhow::Error>
    {
        if let Some(id) = self.id_pool.alloc()
        {
            self.devices.push(ModbusSerialDeviceInterface::new(kind));
            self.devices_lto.insert(id, self.devices.len() - 1);
            return Ok(id);
        }

        Err(anyhow::anyhow!("no free device id"))
    }
    
    pub fn unregister_device(&mut self, device_id: u32)
    {
        if let Some(device_index) = self.devices_lto.get(&device_id)
        {
            
        }
        
    }
    
    pub fn submit_request(&mut self, device_id: u32, request: ModbusSerialRequest)
    {
        
    }
    
    pub fn poll_request(&mut self, device_id: u32) -> Option<ModbusSerialResponse>
    {
        
    }
}

pub struct SerialHandler 
{
    modbus_handler: ModbusSerialHandler
}

pub struct ModbusSerialDeviceInterface
{
    kind: ModbusSerialDeviceKind,
    rx_buf: [u8; 512],
    rx_buf_len: usize,
}

impl ModbusSerialDeviceInterface
{
    pub fn new(kind: ModbusSerialDeviceKind) -> Self
    {
        Self {
            kind,
            rx_buf: [0; 512],
            rx_buf_len: 0,
        }
    }
    
    pub fn submit_request(&mut self, request: ModbusSerialRequest)
    {
        
    }
    
    pub fn poll_request(&mut self) -> Option<ModbusSerialResponse>
    {
        
    }
    
    // ... submit and poll_response
}

pub trait ModbusSerialDevice
{
    fn update(&mut self, interface: &mut ModbusSerialDeviceInterface);
}

pub enum ModbusSerialDeviceKind
{
    Laser,
    US3202510,
    MitsubishiCS80
}

impl SerialHandler 
{
    pub fn update(&mut self)
    {
        self.modbus_handler.update();
    }
    
    pub fn submit_request()
    {
        
    }
    
    pub fn poll_request()
    {
        
    }
}








//TODO: MOVE BELOW TO OTHER FILE

pub struct IdPool
{
    free: Vec<u32>,
    next: u32,
    max: u32,
}

impl IdPool
{
    pub fn new(max: u32) -> Self
    {
        Self {
            free: Vec::new(),
            next: 0,
            max,
        }
    }

    pub fn alloc(&mut self) -> Option<u32>
    {
        if let Some(id) = self.free.pop() {
            Some(id)
        } else if self.next < self.max {
            let id = self.next;
            self.next += 1;
            Some(id)
        } else {
            None
        }
    }

    pub fn free(&mut self, id: u32)
    {
        self.free.push(id);
    }
}