// external deps
use std::{result::Result::Ok};

use core::mem::MaybeUninit;

use anyhow::{anyhow};

use smol::channel::{Sender, unbounded};
use smol::channel::Receiver;

use crate::serial::interfaces::modbus_rtu::frame::FrameParseError;
use crate::serial::interfaces::modbus_rtu::worker;

use super::structs::{
    RequestResult,
    
    ReadRequestResult,
    WriteRequestResult,
    ExceptionRequestResult,
};

use super::{config::Config};

// internal deps
use super::structs::{RequestPayload};

use super::frame::Frame;

// [structs]
#[derive(Debug)]
pub struct Interface<const N: usize>
{
    config: Config,
    
    request_registry: &'static [RequestRegistryEntry; N],
    payloads_buf: [MaybeUninit<RequestPayload>; N],
    metadata_buf: [Metadata; N],
    
    req_tx:  Sender<Frame>,
    resp_rx: Receiver<Result<Option<Frame>, InterfaceIOError>>,
    
    queued_id: Option<usize>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RequestRegistryEntry
{
    pub priority:    u32,
    pub extra_delay: u32,
}

#[derive(Debug)]
pub struct Request
{
    pub type_id: usize,
    pub payload: RequestPayload,
}

#[derive(Debug, Clone, Copy, Default)]
struct Metadata
{
    enabled:       bool,
    ignored_times: u16,
}

#[derive(Debug)]
pub enum InterfaceIOError 
{
    ChannelClosed,
    Parse(FrameParseError),
    Write(std::io::Error),
    Read(std::io::Error),
}

// ======================== functions ========================

impl Metadata
{
    fn new() -> Self { Self { enabled: true, ignored_times: 0 } }
}

impl<const N: usize> Interface<N>
{
    pub fn new(
        config: Config, 
        request_registry: &'static [RequestRegistryEntry; N]
    ) -> Result<Self, anyhow::Error>
    {
        let (req_tx, req_rx) = unbounded::<Frame>();
        let (rsp_tx, rsp_rx) = unbounded::<Result<Option<Frame>, InterfaceIOError>>();
        
        worker::start(config.clone(), req_rx, rsp_tx)?;
        
        Ok(Self {
            config,
            request_registry,
            payloads_buf: unsafe { MaybeUninit::uninit().assume_init() },
            metadata_buf: [Metadata::default(); N],
            req_tx,
            resp_rx: rsp_rx,
            queued_id: None,
        })
    }
    
    pub fn queue_request(&mut self, request: Request)
    {
        assert!(request.type_id < N);

        self.payloads_buf[request.type_id].write(request.payload);
        
        if !self.metadata_buf[request.type_id].enabled 
        { 
            self.metadata_buf[request.type_id] = Metadata::new(); 
        };
    }
    
    fn find_next_request_id(&self) -> Option<usize>
    {
        let mut result: Option<usize> = None;
        let mut i: usize = 0;
        
        for metadata in &self.metadata_buf 
        {
            if !metadata.enabled { continue; }

            if let Some(idx) = result
            {
                let lhs_priority = self.request_registry[i].priority;
                let lhs_ignored_times = self.metadata_buf[i].ignored_times;
                let rhs_priority = self.request_registry[idx].priority;
                let rhs_ignored_times = self.metadata_buf[idx].ignored_times;
                
                let is_higher_priority = higher_priority_than(
                    lhs_priority, 
                    lhs_ignored_times as u32, 
                    rhs_priority, 
                    rhs_ignored_times as u32
                );
                
                if is_higher_priority { result = Some(i); }
            }
            
            else { result = Some(i); }

            i += 1;
        }
        
        result
    }
    
    pub fn send_next_request(&mut self) -> Result<(), anyhow::Error>
    {
        
        
        if self.queued_id.is_some() 
        {
            return Err(anyhow!("Can't send request, open request"));
        }
        
        let request_index = match self.find_next_request_id() 
        {
            Some(idx) => { idx }
            None => { return Err(anyhow!("Nothing in queue")) }
        };

        let payload = unsafe
        {
            self.payloads_buf[request_index].assume_init_ref()
        };
        
        let frame = Frame::from_request(self.config.slave_id, payload);
        
        smol::block_on(self.req_tx.send(frame)).map_err(|_| anyhow!("Failed to send!"))?;

        self.metadata_buf[request_index].enabled = false;
        for metadata in &mut self.metadata_buf 
        {
            metadata.ignored_times += 1;
        }

        self.queued_id = Some(request_index);

        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn discard_all_requests(&mut self)
    {
        for metadata in &mut self.metadata_buf { metadata.enabled = false; }
    }
    
    pub fn check_result(&mut self) -> Result<Option<RequestResult>, InterfaceIOError>
    {
        if let Some(index) = self.queued_id
        {
            let recv_data = match self.resp_rx.try_recv()
            {
                Ok(data) => data ,
                Err(e) => 
                {
                    //tracing::warn!("Nothing hererecv error?"); 
                    
                    match e
                    {
                        smol::channel::TryRecvError::Empty  => return Ok(None),
                        smol::channel::TryRecvError::Closed => return Err(InterfaceIOError::ChannelClosed),
                    }
                },
            };
            
            self.queued_id = None;
            
            match recv_data?
            {
                Some(frame) => 
                {
                    let payload = unsafe { self.payloads_buf[index].assume_init_ref() };
                    
                    let maybe_result = create_response(frame, payload.clone());
                    
                    //tracing::warn!("submitting: {:?}", maybe_result);
                    
                    Ok(maybe_result)
                }
                None => Ok(None)
            }   
        }
        
        else { Ok(None) }
    }
    
    // #[allow(dead_code)]
    // pub fn await_response(&mut self) -> Result<Option<RequestResult>, anyhow::Error>
    // {
    //     let index = self.queued_id.unwrap();
    //     
    //     match self.resp_rx.recv_blocking()??
    //     {
    //         Some(frame) => 
    //         {
    //             let payload = unsafe { self.payloads_buf[index].assume_init_ref() };
    //             let result= frame.try_to_response(payload, &self.response_data_buf)?;
    //             Ok(Some(result))
    //         }
    //         None => Ok(None)
    //     }
    // }
    
    pub fn is_ready_to_send(&self) -> bool
    {
        self.queued_id.is_none()
    }
}


fn higher_priority_than(
    lhs_priority:      u32,
    lhs_ignored_times: u32,
    rhs_priority:      u32,
    rhs_ignored_times: u32,
) -> bool
{
    if lhs_priority + lhs_ignored_times == rhs_priority + rhs_ignored_times
    {
        return rhs_priority > lhs_priority;
    }
    
    lhs_priority + lhs_ignored_times > rhs_priority + rhs_ignored_times
}

fn create_response(frame: Frame, request_payload: RequestPayload) -> Option<RequestResult>
{
    if frame.function_code() != request_payload.function_code() 
    {
        tracing::warn!("Cant match: {:?} to {:?}", frame.function_code(), request_payload.function_code());
        
        // not matching function codes, malformed data
        return None;
    }
    
    if let Some(exception_code) = frame.exception_code() 
    { 
        return Some(RequestResult::Exception(ExceptionRequestResult {
            request: request_payload.clone(),
            result:  exception_code,
        }));
    }
    
    match request_payload 
    {
        RequestPayload::ReadHoldingRegisters(read_registers) => 
        {
            let data = frame.data();
            let len: usize = (data[0]) as usize;
            
            assert!(len < data.len());
            let registers = &data[1..len + 1];
            
            let result = bytes_to_u16_vec(registers)?;
            
            
            Some(RequestResult::ReadHoldingRegisters(ReadRequestResult {
                request: read_registers.clone(),
                result,
            }))
        },
        RequestPayload::ReadInputRegisters(read_registers) =>
        {
            let data = frame.data();
            let len: usize = (data[0]) as usize;
            
            assert!(len < data.len());
            let registers = &data[1..len + 1];
            
            let result = bytes_to_u16_vec(registers)?;
            
            Some(RequestResult::ReadInputRegisters(ReadRequestResult {
                request: read_registers.clone(),
                result,
            }))
        },
        RequestPayload::PresetHoldingRegister(write_register) =>
        {
            Some(RequestResult::PresetHoldingRegister(WriteRequestResult {
                request: write_register.clone(),
                result:  (),
            }))
        },
    }
}

fn bytes_to_u16_vec(buf: &[u8]) -> Option<Vec<u16>> 
{
    // len not a factor of 2
    if !buf.len().is_multiple_of(2) { return None; }

    Some(
        buf.chunks(2)
            .map(|chunk| ((chunk[0] as u16) << 8) | (chunk[1] as u16))
            .collect()
    )
}
