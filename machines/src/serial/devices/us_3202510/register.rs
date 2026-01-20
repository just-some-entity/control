#[derive(Debug, Clone, Copy)]
pub enum Register 
{
    // Holding Registers (RW): 0x2 offset
    
    /// Register 0x0002
    SetFrequency,

    /// Register 0x0003
    RunCommand,

    /// Register 0x0004
    AccelerationTime,

    /// Register 0x0005
    DecelerationTime,

    // Input Registers (RO): 0x8 offset

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
    CurrentFrequency,
}

impl Register 
{
    pub const fn address(self) -> u16 
    {
        const HOLD_REGISTER_OFFSET:  u16 = 0x2;
        const INPUT_REGISTER_OFFSET: u16 = 0x8;

        match self 
        {
            // Holding Registers (RW): 0x2 offset
            Register::SetFrequency     => HOLD_REGISTER_OFFSET,     // 0x0002
            Register::RunCommand       => HOLD_REGISTER_OFFSET + 1, // 0x0003
            Register::AccelerationTime => HOLD_REGISTER_OFFSET + 2, // 0x0004
            Register::DecelerationTime => HOLD_REGISTER_OFFSET + 3, // 0x0005

            // Input Registers (RO): 0x8 offset
            Register::BusVoltage       => INPUT_REGISTER_OFFSET,     // 0x0008
            Register::LineCurrent      => INPUT_REGISTER_OFFSET + 1, // 0x0009
            Register::DriveTemperature => INPUT_REGISTER_OFFSET + 2, // 0x000A
            Register::SystemStatus     => INPUT_REGISTER_OFFSET + 3, // 0x000B
            Register::ErrorCode        => INPUT_REGISTER_OFFSET + 4, // 0x000C
            Register::CurrentFrequency => INPUT_REGISTER_OFFSET + 5, // 0x000D
        }
    }
}