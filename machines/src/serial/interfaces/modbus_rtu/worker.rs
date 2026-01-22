use std::thread;
use std::time::Duration;

use anyhow::anyhow;

use serialport::ClearBuffer;
use serialport::SerialPort;
use smol::channel::Receiver;
use smol::channel::Sender;

use crate::serial::interfaces::modbus_rtu::FRAME_SIZE_MAX;
use crate::serial::interfaces::modbus_rtu::frame::Frame;
use crate::serial::interfaces::modbus_rtu::interface::InterfaceIOError;

pub fn start(
    config: super::Config, 
    rx: Receiver<Frame>,
    tx: Sender<Result<Option<Frame>, InterfaceIOError>>
) -> Result<(), anyhow::Error>
{
    let port = open_port(&config)?;
    
    let _ = thread::Builder::new()
        .name("modbus_rtu_interface".to_owned())
        .spawn(move || { smol::block_on(process(config, port, rx, tx)) })?;
    
    Ok(())
}

async fn process(
    config: super::Config,
    mut port: Box<dyn SerialPort>,
    rx: Receiver<Frame>, 
    tx: Sender<Result<Option<Frame>, InterfaceIOError>>
)
{
    let mut frame_in_buf: [u8; 256] = [0; 256];
    
    while let Ok(frame_out) = rx.recv().await
    {
        if let Err(e) = port.write_all(frame_out.bytes()) 
        {
            _ = tx.send(Err(InterfaceIOError::Write(e))).await;
            continue;
        };
        
        std::thread::sleep(compute_timeout(&config, frame_out.len()));
        
        let frame_in_len: usize = match port.read(&mut frame_in_buf)
        {
            Ok(data_len) => 
            {
                if data_len == 0
                {
                    _ = tx.send(Ok(None)).await;
                    continue;
                }
                
                data_len
            },
            Err(e) => 
            {
                _ = tx.send(Err(InterfaceIOError::Read(e))).await;
                
                continue;
            }
        };
        
        debug_assert!(frame_in_len <= FRAME_SIZE_MAX);
        
        let frame_in = match Frame::from_bytes(&frame_in_buf[..frame_in_len])
        {
            Ok(frame) => frame,
            Err(e) => 
            {
                _ = tx.send(Err(InterfaceIOError::Parse(e))).await;
                continue;
            },
        };

        //tracing::warn!("frame: {:?}:", frame_in);

        _ = tx.send(Ok(Some(frame_in))).await;
    }
}

fn open_port(config: &super::Config) -> Result<Box<dyn SerialPort>, anyhow::Error>
{
    let mut port: Box<dyn SerialPort> = serialport::new(&config.path, 9600)
        .data_bits(config.data_bits)
        .parity(config.parity)
        .stop_bits(config.stop_bits)
        .flow_control(config.flow_control)
        .timeout(config.timeout)
        .open()
        .map_err(|e| anyhow!("Failed to open port {}: {}", &config.path, e))?;

    port.write_data_terminal_ready(true).ok();
    port.write_request_to_send(true).ok();
    port.clear(ClearBuffer::All).ok();
    
    Ok(port)
}

fn compute_timeout(config: &super::Config, message_size: usize) -> Duration
{
    let nanoseconds_per_bit: u64 = (1000000 / config.baudrate) as u64;
    let nanoseconds_per_byte: u64 = config.total_bits_per_frame() as u64 * nanoseconds_per_bit;

    let transmission_timeout: u64 = nanoseconds_per_byte * message_size as u64;
    let silent_time: u64 = (nanoseconds_per_byte * (35)) / 10_u64; // silent_time is 3.5x of character length,which is 11 bit for 8E1

    let mut full_timeout: u64 = transmission_timeout;
    full_timeout += config.machine_operation_delay.as_nanos() as u64;
    full_timeout += silent_time;

    Duration::from_nanos(full_timeout)
}