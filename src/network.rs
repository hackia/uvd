pub const PROTOCOL: &str = "uvd://";
pub const PORT: u16 = 7789;
pub const MAX_PACKET_SIZE: usize = 1024;
pub const MAX_RETRIES: u8 = 3;

#[doc = "Validate the port"]
pub fn validate_port(port: u16) -> bool {
    port == PORT
}
#[doc = "Validate the protocol"]
pub fn validate_protocol(protocol: &str) -> bool {
    protocol == PROTOCOL
}

#[doc = "Validate the maximum packet size"]
pub fn validate_packet_size(size: usize) -> bool {
    size <= MAX_PACKET_SIZE
}

#[doc = "Validate the maximum number of retries"]
pub fn validate_retries(retries: u8) -> bool {
    retries <= MAX_RETRIES
}
