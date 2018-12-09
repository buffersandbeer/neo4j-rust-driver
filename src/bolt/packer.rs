/// Big-endian conversion of a u32 value to an array of u8s
///
/// ```
/// use neo4j::bolt::packer::{u32_to_u8_be};
/// 
/// let result = u32_to_u8_be(0x01020304);
/// ```
pub fn u32_to_u8_be(value: u32) -> [u8; 4] {
    [((value >> 24) & 0xff) as u8,
     ((value >> 16) & 0xff) as u8,
     ((value >> 8) & 0xff) as u8,
     (value & 0xff) as u8]
}

/// Big-endian conversion of an array of u8s to a u32 value
///
/// ```
/// use neo4j::bolt::packer::{u8_to_u32_be};
///
/// let result = u8_to_u32_be([0x01, 0x02, 0x03, 0x04]);
/// ```
pub fn u8_to_u32_be(value: [u8; 4]) -> u32 {
    ((value[0] as u32) << 24) +
    ((value[1] as u32) << 16) +
    ((value[2] as u32) << 8) +
    (value[3] as u32)
}

/// Create a handshake buffer that is ready for transmission.
///
/// ```
/// use neo4j::bolt::packer::{pack_handshake};
///
/// let handshake = pack_handshake(0x6060b017, [1, 0, 0, 0]);
/// ```
pub fn pack_handshake(preamble: u32, supported_protocols: [u32; 4]) -> [u8; 20] {
    let mut handshake_buffer: [u8; 20] = [0x00; 20];
    let preamble_u8: [u8; 4] = u32_to_u8_be(preamble);
    let version_0_u8: [u8; 4] = u32_to_u8_be(supported_protocols[0]);
    let version_1_u8: [u8; 4] = u32_to_u8_be(supported_protocols[1]);
    let version_2_u8: [u8; 4] = u32_to_u8_be(supported_protocols[2]);
    let version_3_u8: [u8; 4] = u32_to_u8_be(supported_protocols[3]);
    for idx in 0..4 {
        handshake_buffer[0 + idx] = preamble_u8[idx]; 
        handshake_buffer[4 + idx] = version_0_u8[idx]; 
        handshake_buffer[8 + idx] = version_1_u8[idx]; 
        handshake_buffer[12 + idx] = version_2_u8[idx]; 
        handshake_buffer[16 + idx] = version_3_u8[idx]; 
    }
    handshake_buffer
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pack_handshake() {
        let preamble: u32 = 0x6060b017;
        let versions: [u32; 4] = [1, 0, 0, 0];
        let expected: [u8; 20] = [0x60, 0x60, 0xb0, 0x17,
                                  0, 0, 0, 1,
                                  0, 0, 0, 0,
                                  0, 0, 0, 0,
                                  0, 0, 0, 0];
        let result = super::pack_handshake(preamble, versions);
        assert_eq!(result, expected, "{:?} != {:?}", result, expected);
    }

    #[test]
    fn test_u8_to_u32_be() {
        let expected: u32 = 0x01020304;
        let test: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let result = super::u8_to_u32_be(test);
        assert_eq!(expected, result, "{:?} != {:?}", expected, result);

    }

    #[test]
    fn test_u32_to_u8_be() {
        let test: u32 = 0x01020304;
        let expected: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let result = super::u32_to_u8_be(test);
        assert_eq!(expected, result, "{:?} != {:?}", expected, result);
    }

}
