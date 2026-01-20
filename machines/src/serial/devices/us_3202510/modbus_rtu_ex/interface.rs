// external deps
use std::{thread, time::Duration};
use std::result::Result::Ok;

use core::mem::MaybeUninit;

use anyhow::{anyhow};

use serialport::SerialPort;

use smol::channel::{Sender, unbounded};
use smol::channel::Receiver;

// internal deps
use super::payload::{Payload};

use control_core::modbus;

#[derive(Debug)]
pub struct Request
{
    pub type_id:  usize,
    pub priority: u16,
    pub payload:  Payload,
    pub delay:    Option<u32>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Metadata
{
    enabled:       bool,
    priority:      u16,
    ignored_times: u16,
    extra_delay:   u32,
}

impl Metadata
{
    fn higher_priority_than(&self, other: &Metadata) -> bool
    {
        if self.effective_priority() == other.effective_priority() 
        {
            return self.priority > other.priority;
        }
        
        self.effective_priority() > other.effective_priority()
    }
    
    fn effective_priority(&self) -> u16 { self.priority + self.ignored_times }
}

#[derive(Debug)]
pub struct Interface<const REQUEST_TYPE_COUNT: usize>
{
    payload_buffer:  [MaybeUninit<Payload>; REQUEST_TYPE_COUNT],
    metadata_buffer: [Metadata;             REQUEST_TYPE_COUNT],
    
    is_ready: bool,
    
    req_tx:  Sender<Payload>,
    resp_rx: Receiver<Payload>,
}

impl<const REGISTRY_SIZE: usize> Interface<REGISTRY_SIZE>
{
    pub fn new(port: Box<dyn SerialPort>) -> Result<Self, anyhow::Error>
    {
        let (req_tx, req_rx) = unbounded::<Payload>();
        let (resp_tx, resp_rx) = unbounded::<Payload>();
        
        let _ = thread::Builder::new()
            .name("modbus_rtu_interface".to_owned())
            .spawn(move || 
            {
                smol::block_on(Self::process(port, req_rx, resp_tx))
            })?;
        
        Ok(Self {
            payload_buffer:  unsafe { MaybeUninit::uninit().assume_init() },
            metadata_buffer: [Metadata::default(); REGISTRY_SIZE],
            is_ready: true,
            req_tx,
            resp_rx,
        })
    }
    
    async fn process(
        mut port: Box<dyn SerialPort>,
        rx: Receiver<Payload>,
        tx: Sender<Payload>,
    ) -> Result<(), anyhow::Error>
    {
        let mut payload_buf: [u8; 256] = [0; 256];
        
        while let Ok(payload) = rx.recv().await
        {
            let frame = payload.encode_frame(&mut payload_buf);
            
            port.write_all(frame)?;
            
            //TODO: compute proper timing
            std::thread::sleep(modbus::calculate_modbus_rtu_timeout(
                8,
                Duration::from_millis(10),
                38400,
                8,
            ));
            
            if let Some(response_data) = modbus::receive_data_modbus(&mut *port)?
            {
                // Payload was validated in receive_data_modbus
                // so assume all bytes are set correctly
                let payload = Payload::decode_frame(response_data.as_slice())?;
                
                let _ = tx.send(payload).await;
            }
        }
        
        Ok(())
    }
    
    pub fn queue_request(&mut self, request: Request)
    {
        // Fail loud in debug
        debug_assert!(
            request.type_id < REGISTRY_SIZE && 
            request.type_id <= i16::MAX as usize
        );
        
        // Fail silently in release
        if request.type_id >= REGISTRY_SIZE || 
           request.type_id > i16::MAX as usize 
           { return; }
        
        self.payload_buffer[request.type_id].write(request.payload);
        
        if self.metadata_buffer[request.type_id].enabled { return; };
        
        self.metadata_buffer[request.type_id] = 
            Metadata { 
                enabled: true, 
                priority:request.priority, 
                ignored_times: 0, 
                extra_delay: request.delay.unwrap_or(0)
            };
    }
    
    pub fn send_next_request(&mut self) -> Result<(), anyhow::Error>
    {
        debug_assert!(self.is_ready);
        
        let mut found_item: bool = false;
        let mut highest_prio_idx: usize = 0;
        let mut i: usize = 0;
        for metadata in &self.metadata_buffer 
        {
            if !metadata.enabled { continue; }
            if metadata.higher_priority_than(&self.metadata_buffer[highest_prio_idx])
            {
                highest_prio_idx = i;
                found_item = true;
            }
            
            i += 1;
        }
        
        if !found_item { return Err(anyhow!("No Request in queue!")); }
        
        let request = unsafe
        {
            self.payload_buffer[i].assume_init_ref().clone()
        };
        
        self.metadata_buffer[i].enabled = false;
        
        for metadata in &mut self.metadata_buffer 
        {
            // ignore if statement, to avoid branch, since disabled ones 
            // get set to 0 when enabled anyways
            metadata.ignored_times += 1;
        }
        
        smol::block_on(self.req_tx.send(request)).map_err(|_| anyhow!("Failed to send!"))?;

        self.is_ready = false;

        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn discard_all_requests(&mut self)
    {
        for metadata in &mut self.metadata_buffer { metadata.enabled = false; }
    }
    
    pub fn check_response(&mut self) -> Option<Payload>
    {
        match self.resp_rx.try_recv()
        {
            Ok(response) => 
            {
                self.is_ready = true;
                Some(response)
            }
            Err(_) => None
        }
    }
    
    #[allow(dead_code)]
    pub fn await_response(&mut self) -> Option<Payload>
    {
        match self.resp_rx.recv_blocking()
        {
            Ok(response) => 
            {
                self.is_ready = true;
                Some(response)
            }
            Err(_) => None
        }
    }
    
    pub fn is_ready_to_send(&self) -> bool
    {
        self.is_ready
    }
}