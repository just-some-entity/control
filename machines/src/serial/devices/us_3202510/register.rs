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

impl US3202510Register 
{
    const fn address(self) -> u16 
    {
        match self {
            Self::HoldRegisterBank => 0x2,
            Self::InputRegisterBank => 0x8,
        }
    }

    const fn address_be_bytes(self) -> [u8; 2]
    {
        self.address().to_be_bytes()
    }
}