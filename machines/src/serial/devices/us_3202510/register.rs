#[derive(Debug, Clone, Copy)]
pub enum US3202510Register 
{
    /// Register 0x0002
    SetFrequency,

    /// Register 0x0003
    RunCommand,

    /// Register 0x0004
    AccelerationTime,

    /// Register 0x0005
    DeacelerationTime,

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

const HOLD_REGISTER:  u16 = 0x2;
const INPUT_REGISTER: u16 = 0x8;


impl US3202510Register 
{
    pub const fn address(self) -> u16 
    {
        match self {
            US3202510Register::SetFrequency => HOLD_REGISTER + 0,            // 0x0002
            US3202510Register::RunCommand => HOLD_REGISTER + 1,              // 0x0003
            US3202510Register::AccelerationTime => HOLD_REGISTER + 2,        // 0x0004
            US3202510Register::DeacelerationTime => HOLD_REGISTER + 3,       // 0x0005

            US3202510Register::BusVoltage => INPUT_REGISTER + 0,             // 0x0008
            US3202510Register::LineCurrent => INPUT_REGISTER + 1,            // 0x0009
            US3202510Register::DriveTemperature => INPUT_REGISTER + 2,       // 0x000A
            US3202510Register::SystemStatus => INPUT_REGISTER + 3,           // 0x000B
            US3202510Register::ErrorCode => INPUT_REGISTER + 4,              // 0x000C
            US3202510Register::CurrentOperatingFrequency => INPUT_REGISTER + 5, // 0x000D
        }
    }

    const fn address_be_bytes(self) -> [u8; 2]
    {
        self.address().to_be_bytes()
    }
}