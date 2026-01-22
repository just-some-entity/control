mod interface;
mod structs;
mod config;
mod frame;
mod worker;

// mod _enum_ex;

pub use interface::RequestRegistryEntry;

pub use structs::RequestResult;

pub use config::Config;

pub use interface::Interface;
pub use interface::Request as InterfaceRequest;

pub use structs::FunctionCode;
pub use structs::RequestPayload;

pub use structs::request;
pub use structs::response;

const FRAME_SIZE_MAX: usize = 256;