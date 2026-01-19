

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
type AsyncFn<T> = Box<dyn Fn() -> BoxFuture<T> + Send + Sync>;
type AsyncFnWithArg<A, T> = Box<dyn Fn(A) -> BoxFuture<T> + Send + Sync>;

struct SerialInterface
{
    pub has_message:         AsyncFn<bool>,
    pub write_message:       AsyncFnWithArg<Vec<u8>, Result<bool, Error>>,
    pub read_message:        AsyncFn<Option<Vec<u8>>>,
    pub get_baudrate:        AsyncFn<Option<u32>>,
    pub get_serial_encoding: AsyncFn<Option<SerialEncoding>>,
    pub initialize:          AsyncFn<bool>,
}

struct ModbusSerialInterface
{
    serial_interface: SerialInterface,
    baudrate: Option<u32>,
    encoding: Option<SerialEncoding>,
    response: Option<ModbusResponse>,
    state: State,
    last_message_size: usize,
    last_message_delay: u32,
    pub last_message_id: u32,
    no_response_expected: bool,
    request_map: HashMap<u32, ModbusRequest>,
    request_metadata_map: HashMap<u32, RequestMetaData>,
    last_ts: Instant,
}

#[derive(Debug, Clone)]
struct RequestMetaData {
    priority: u32,
    ignored_times: u32,
    /// This is used when a machine for example takes 4ms to process the request
    /// and is added ontop of the standard waiting time for a serial transfer
    extra_delay: Option<u32>,
    no_response_expected: bool,
}

#[derive(Debug)]
pub enum State {
    /// WaitingForResponse is set after sending a request through the serial_interface
    WaitingForResponse,
    /// After Sending a Request we need to wait atleast one ethercat cycle
    /// After one Cycle we check if el6021 status has transmit accepted toggled
    /// Then we can set state = ReadyToSend
    WaitingForRequestAccept,
    /// After Receiving a Response we need to wait atleast one ethercat cycle
    /// After one Cycle we check if el6021 status has received accepted toggled
    WaitingForReceiveAccept,
    /// ReadyToSend is set after receiving the response from the serial_interface
    ReadyToSend,
    /// Initial State
    Uninitialized,
}

impl SerialInterface
{
    pub fn from_ethercat_device<PORT>(
        device: Arc<RwLock<dyn SerialInterfaceDevice<PORT>>>, 
        port: PORT
    ) -> Self
    {
        let mut port2 = port.clone();
        let mut device2 = device.clone();
        
        let read_message = Box::new(
            move || -> Pin<Box<dyn Future<Output = Option<Vec<u8>>> + Send>> {
                let device2 = device2.clone();
                let port_clone = port2.clone();

                Box::pin(async move {
                    let mut device = device2.write().await;
                    device.serial_interface_read_message(port_clone)
                })2
            },
        );
    }
    
    pub fn from_direct() -> Self
    {
        
    }
}