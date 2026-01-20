use std::{thread, time::Duration};

use control_core::modbus::{self, ModbusFunctionCode, ModbusResponse};

use serialport::SerialPort;

use smol::channel::{Sender, unbounded};
use smol::channel::Receiver;

use std::result::Result::Ok;

#[derive(Debug)]
pub struct RequestEntry
{
    pub priority: u16,
    pub no_response_expected: bool,
    pub payload: Request,
}

#[derive(Debug, Clone)]
pub struct Request
{
    pub slave_id: u8,
    pub function_code: ModbusFunctionCode,
    pub data: &'static [u8],
}

impl Request
{
    pub fn write_to_buf<'a>(&self, buf: &'a mut [u8; 128]) -> &'a [u8]
    {
        assert!(self.data.len() < 96, "Modbus request data too long");
        
        buf[0] = self.slave_id;
        buf[1] = self.function_code.clone().into();
        
        let mut i: usize = 2;
        for b in self.data
        {
            buf[i] = *b;
            i += 1;
        }
        
        let length = i;
        
        let crc16_modbus: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_MODBUS);
        let result_crc = crc16_modbus.checksum(&buf[..length]);
        
        // Little-endian: low byte first, high byte second
        buf[length]     = (result_crc & 0xFF) as u8; // CRC low byte
        buf[length + 1] = (result_crc >> 8) as u8;   // CRC high byte
        
        // Return slice including CRC
        &buf[..length + 2]
    }
}

#[derive(Debug)]
pub enum State
{
    Idle,
    Waiting,
}

#[derive(Debug)]
pub struct Interface<const N: usize>
{
    request_table:  &'static [RequestEntry; N],
    priority_table: [i32; N],
    
    state: State,
    
    req_tx:  Sender<Request>,
    resp_rx: Receiver<ModbusResponse>,
}

impl<const N: usize> Interface<N>
{
    pub fn new(
        request_table: &'static [RequestEntry; N], 
        port: Box<dyn SerialPort>
    ) -> Result<Self, anyhow::Error>
    {
        let (req_tx, req_rx) = unbounded::<Request>();
        let (resp_tx, resp_rx) = unbounded::<ModbusResponse>();
        
        let _ = thread::Builder::new()
            .name("modbus_rtu_interface".to_owned())
            .spawn(move || 
            {
                smol::block_on(Self::process(port, req_rx, resp_tx))
            })?;
        
        Ok(Self {
            request_table,
            priority_table: [-1; N],
            state: State::Idle,
            req_tx,
            resp_rx,
        })
    }
    
    async fn process(
        mut port: Box<dyn SerialPort>,
        rx: Receiver<Request>,
        tx: Sender<ModbusResponse>,
    ) -> Result<(), anyhow::Error>
    {
        let mut payload_buf: [u8; 128] = [0; 128];
        
        while let Ok(request) = rx.recv().await 
        {
            let frame = request.write_to_buf(&mut payload_buf);
            
            port.write_all(&frame);
            
            std::thread::sleep(modbus::calculate_modbus_rtu_timeout(
                8,
                Duration::from_millis(10),
                38400,
                8,
            ));
            
            let maybe_response = 
                modbus::receive_data_modbus(&mut *port)?
                .map(ModbusResponse::try_from);
                
            match maybe_response 
            {
                Some(response) => 
                {
                    let _ = tx.send(response?).await;
                },
                None => {}
            }
        }
        
        Ok(())
    }
    
    pub fn queue_request(&mut self, request_type_id: usize) -> Result<(), ()>
    {
        if request_type_id >= N { return Err(()); }
        
        if self.priority_table[request_type_id] != -1 { return Ok(()); };
        
        self.priority_table[request_type_id] = 
            self.request_table[request_type_id].priority.into();
            
        Ok(())
    }
    
    pub fn send_request(&mut self) -> Result<(), ()>
    {
        let mut highest_prio_idx: usize = 0;
        let mut i: usize = 0;
        for prio in self.priority_table 
        {
            if prio >= self.priority_table[highest_prio_idx]
            {
                highest_prio_idx = i;
            }
            
            i += 1;
        }
        
        if self.priority_table[highest_prio_idx] <= -1
        {
            return Ok(());
        }
        
        let index: usize = i.into();
        let request = self.request_table[index].payload.clone();
        
        self.priority_table[index] = -1;
        for prio in &mut self.priority_table 
        {
            if *prio == -1 { continue; }
            *prio += 1;
        }
        
        smol::block_on(self.req_tx.send(request)).map_err(|_| ())?;

        Ok(())
    }
       
    pub fn discard_requests(&mut self)
    {
        for prio in self.priority_table.iter_mut() { *prio = -1; }
    }
    
    pub fn poll_response(&mut self) -> Option<ModbusResponse>
    {
        match self.resp_rx.try_recv() 
        {
            Ok(frame) => ModbusResponse::try_from(frame).ok(),
            Err(_) => None, // No response yet    
        }
    }
}