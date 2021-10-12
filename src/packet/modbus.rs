use pnet_macros::packet;
use pnet_macros_support::types::*;

#[packet]
pub struct Modbus {
    // add fields
    #[payload]
    payload: Vec<u8>,
}
