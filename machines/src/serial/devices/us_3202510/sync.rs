use std::{collections::BinaryHeap, sync::{Arc, atomic::AtomicBool}};

use control_core::modbus::{ModbusRequest, ModbusResponse};

use ringbuf::{traits::*, HeapRb};
use serialport::SerialPort;

pub struct ModbusRequestEx
{
    base: ModbusRequest,
    priority: 
}

pub enum ModbusRTUIOChannelState
{
    Idle,
    Processing,
}

pub struct ModbusRTUIOChannel
{
    state: ModbusRTUIOChannelState,
    
    requests:  BinaryHeap<ModbusRequest>,
    responses: Vec<ModbusResponse>,
}

impl ModbusRTUIOChannel
{
    pub fn record_request(&mut self, request: ModbusRequest) -> Result<(), anyhow::Error>
    {
        if(sel)
    }
        
    pub fn discard_requests(&mut self)
    {
        self.requests.clear();
    }
    
    pub fn submit_requests(&mut self, port: dyn SerialPort)
    {
        
    }
}