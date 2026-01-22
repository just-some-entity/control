use std::time::Duration;

use serialport;

#[derive(Debug, Clone)]
pub struct Config
{
    pub slave_id:     u8,
    pub path:         String,
    pub data_bits:    serialport::DataBits,
    pub parity:       serialport::Parity,
    pub stop_bits:    serialport::StopBits,
    pub flow_control: serialport::FlowControl,
    pub timeout:      Duration,
    pub baudrate:     u32,
    
    pub machine_operation_delay: Duration,
}

impl Config
{
    pub fn total_bits_per_frame(&self) -> u8
    {
        let data_bits = match self.data_bits 
        {
            serialport::DataBits::Five  => 5,
            serialport::DataBits::Six   => 6,
            serialport::DataBits::Seven => 7,
            serialport::DataBits::Eight => 8,
        };

        let parity_bits = match self.parity 
        {
            serialport::Parity::None => 0,
            _ => 1,
        };

        let stop_bits = match self.stop_bits 
        {
            serialport::StopBits::One => 1,
            serialport::StopBits::Two => 2,
        };

        data_bits + parity_bits + stop_bits
    }
}