use pnet_macros::packet;
use pnet_macros_support::types::*;

#[packet]
pub struct Modbus {
    // add fields
    // tid: XXX,
    // pid: XXX,
    // length: XXX,
    // uid:  XXX,
    // function_code: XXX,
    #[payload]
    payload: Vec<u8>,
}
