

#[derive(Debug, Clone, Copy)]
enum MitsubishiCS80Register 
{
    InverterReset,
    InverterStatusAndControl,
    RunningFrequencyRAM,
    MotorStatus,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum MitsubishiCS80Request
{
    None,
    ResetInverter,
    ClearAllParameters,
    ClearNonCommunicationParameter,
    ClearNonCommunicationParameters,
    ReadInverterStatus,
    StopMotor,
    StartForwardRotation,
    StartReverseRotation,
    ReadRunningFrequency,
    WriteRunningFrequency,
    ReadMotorStatus,
    WriteParameter,
}

#[derive(Debug, Clone, Copy)]
pub enum RequestType {
    None,
    OperationCommand,
    ReadWrite,
    ParamClear,
    Reset,
}

#[derive(Debug)]
pub struct MitsubishiCS80<'a>
{
    interface: &'a mut ModbusSerialDeviceInterface,
    
    // Communication
    pub status: MitsubishiCS80Status,
    pub motor_status: MotorStatus,
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct MitsubishiCS80Status 
{
    pub running: bool,
    pub forward_running: bool,
    pub reverse_running: bool,
    pub su: bool,
    pub ol: bool,
    pub no_function: bool,
    pub fu: bool,
    pub abc_: bool,
    pub fault_occurence: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MotorStatus 
{
    pub rpm: AngularVelocity,
    pub frequency: Frequency,
    pub current: ElectricCurrent,
    pub voltage: ElectricPotential,
}

impl MitsubishiCS80 
{
    pub fn new(device_id: u32) -> Self 
    {
        Self {
            device_id,
            motor_status: MotorStatus::default(),
            status:       MitsubishiCS80Status::default(),
        }
    }
    
    fn handle_motor_status(&mut self, resp: &ModbusResponse) 
    {
        if resp.data.len() >= 7 
        {
            let freq_bytes = &resp.data[1..3]; // bytes 1 and 2 are needed
            let raw_frequency = u16::from_be_bytes([freq_bytes[0], freq_bytes[1]]) as f64;
            self.motor_status.frequency = Frequency::new::<centihertz>(raw_frequency);

            let electric_current_bytes = &resp.data[3..5];
            let raw_current =
                u16::from_be_bytes([electric_current_bytes[0], electric_current_bytes[1]]) as f64;
            self.motor_status.current = ElectricCurrent::new::<centiampere>(raw_current);

            let voltage_current_bytes = &resp.data[5..7];
            let raw_voltage =
                u16::from_be_bytes([voltage_current_bytes[0], voltage_current_bytes[1]]) as f64;
            self.motor_status.voltage = ElectricPotential::new::<centivolt>(raw_voltage);
        }
    }
    
    fn handle_read_inverter_status(&mut self, resp: &ModbusResponse) 
    {
        if resp.data.len() < 3 { return; }

        let status_bytes: [u8; 2] = match resp.data[1..3].try_into() 
        {
            Ok(bytes) => bytes,
            Err(_) => return,
        };

        let bits: &BitSlice<u8, Lsb0> = BitSlice::<_, Lsb0>::from_slice(&status_bytes);
        
        if bits.len() >= 16 
        {
            self.status = MitsubishiCS80Status {
                fault_occurence: bits[7],
                running: bits[8],
                forward_running: bits[9],
                reverse_running: bits[10],
                su: bits[11],
                ol: bits[12],
                no_function: bits[13],
                fu: bits[14],
                abc_: bits[15],
            };
        }
    }
    
    fn handle_response(&mut self, control_request_type: u32) 
    {
        let response_type = match MitsubishiCS80Requests::try_from(control_request_type) 
        {
            Ok(request_type) => request_type,
            Err(_) => return,
        };

        let Some(response) = self.modbus_serial_interface.get_response().cloned() else { return; };

        match response_type 
        {
            MitsubishiCS80Requests::ReadInverterStatus => 
            {
                self.handle_read_inverter_status(&response);
            }
            MitsubishiCS80Requests::ReadMotorStatus => 
            {
                self.handle_motor_status(&response);
            }
            // Other request types don't need response handling
            _ => {}
        }
    }
    
    fn convert_frequency_to_word(&self, frequency: Frequency) -> u16 
    {
        let scaled = frequency.get::<centihertz>(); // Convert Hz to 0.01 Hz units
        scaled.round() as u16
    }
    
    pub fn stop_motor(&mut self, interface: &mut ModbusSerialDeviceInterface) 
    {
        self.add_request(MitsubishiCS80Requests::StopMotor.into());
    }
}

impl ModbusSerialDevice for MitsubishiCS80 
{
    fn update(&mut self, interface: &mut ModbusSerialDeviceInterface)
    {
        
    }
}