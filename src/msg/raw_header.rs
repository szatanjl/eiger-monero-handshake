#[derive(Debug)]
pub struct RawHeader {
    pub signature: u64,
    pub length: usize,
    pub expect_response: u8,
    pub command: u32,
    pub return_code: u32,
    pub flags: u32,
    pub version: u32,
}

pub const LENGTH: usize = 33;
pub const SIGNATURE: u64 = 0x0101010101012101;
pub const VERSION: u32 = 1;

impl From<[u8; LENGTH]> for RawHeader {
    fn from(bytes: [u8; LENGTH]) -> Self {
        Self {
            signature: u64::from_le_bytes(into_array(&bytes[0..8])),
            length: usize::from_le_bytes(into_array(&bytes[8..16])),
            expect_response: bytes[16],
            command: u32::from_le_bytes(into_array(&bytes[17..21])),
            return_code: u32::from_le_bytes(into_array(&bytes[21..25])),
            flags: u32::from_le_bytes(into_array(&bytes[25..29])),
            version: u32::from_le_bytes(into_array(&bytes[29..33])),
        }
    }
}

impl From<RawHeader> for [u8; LENGTH] {
    fn from(header: RawHeader) -> Self {
        let mut bytes = [0; LENGTH];

        bytes[0..8].copy_from_slice(&header.signature.to_le_bytes());
        bytes[8..16].copy_from_slice(&header.length.to_le_bytes());
        bytes[16] = header.expect_response;
        bytes[17..21].copy_from_slice(&header.command.to_le_bytes());
        bytes[21..25].copy_from_slice(&header.return_code.to_le_bytes());
        bytes[25..29].copy_from_slice(&header.flags.to_le_bytes());
        bytes[29..33].copy_from_slice(&header.version.to_le_bytes());

        bytes
    }
}

fn into_array<const N: usize>(slice: &[u8]) -> [u8; N] {
    slice.try_into().unwrap()
}
