mod interface;
mod payload;

pub use payload::Payload;
pub use interface::Interface;
pub use interface::Request as InterfaceRequest;

#[allow(dead_code)]
pub const FRAME_SIZE_MAX: usize = 256;