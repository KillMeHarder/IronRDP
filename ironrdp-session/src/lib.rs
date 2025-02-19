#[macro_use]
extern crate log;

mod codecs;
mod errors;
mod utils;

pub mod active_session;
pub mod connection_sequence;
pub mod image;
pub mod transport;

use ironrdp::{gcc, nego};

pub use crate::active_session::{ActiveStageOutput, ActiveStageProcessor};
pub use crate::codecs::{ErasedWriter, FramedReader};
pub use crate::connection_sequence::{process_connection_sequence, ConnectionSequenceResult, UpgradedStream};
pub use crate::errors::RdpError;

pub struct GraphicsConfig {
    pub avc444: bool,
    pub h264: bool,
    pub thin_client: bool,
    pub small_cache: bool,
    pub capabilities: u32,
}

pub struct InputConfig {
    pub credentials: sspi::AuthIdentity,
    pub security_protocol: nego::SecurityProtocol,
    pub keyboard_type: gcc::KeyboardType,
    pub keyboard_subtype: u32,
    pub keyboard_functional_keys_count: u32,
    pub ime_file_name: String,
    pub dig_product_id: String,
    pub width: u16,
    pub height: u16,
    pub global_channel_name: String,
    pub user_channel_name: String,
    pub graphics_config: Option<GraphicsConfig>,
}
