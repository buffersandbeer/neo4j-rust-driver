use std::result::Result;

/// Take an array of u32 integers and pack into a vector of u8 integers.
///
/// Primarially for converting u32 strings into values that can be sent
/// through a network socket.
///
/// ```
/// use neo4j::util::packer::{pack_u32_to_u8};
///
/// let payload: [u32; 5] = [0x0, 0x0, 0x0, 0x0, 0x0];
/// let packed: Vec<u8> = pack_u32_to_u8(&payload);
/// ```
pub fn pack_u32_to_u8(payload: &[u32]) -> Vec<u8> {

    let payload_len = payload.len();
    let mut packed: Vec<u8> = Vec::with_capacity(payload_len);
    packed = vec![0; payload_len * 4];

    for idx in 0..payload_len {
        packed[0 + (4 * idx)] = ((payload[idx] >> 24) & 0xff) as u8; 
        packed[1 + (4 * idx)] = ((payload[idx] >> 16) & 0xff) as u8; 
        packed[2 + (4 * idx)] = ((payload[idx] >> 8) & 0xff) as u8; 
        packed[3 + (4 * idx)] = (payload[idx] & 0xff) as u8; 
    }

    return packed;
}


/// Take a slice of 4 u8's and  return a u32.
///
/// This uses big-endian conversion to merge the values.
///
/// ```
/// use neo4j::util::packer::{unpack_u8_to_u32};
///
/// let packed: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
/// let unpacked: u32 = unpack_u8_to_u32(&packed).unwrap();
/// ```
pub fn unpack_u8_to_u32(packed: &[u8]) -> Result<u32, String> {
    
    let packed_len = packed.len();

    if packed_len != 4 {
        return Err("Invalid u8 size. Not enough bytes to unpack to u32".to_string());
    }

    let unpacked: u32 = ((packed[0] as u32) << 24) +
                        ((packed[1] as u32) << 16) +
                        ((packed[2] as u32) << 8) +
                        (packed[3] as u32);
    return Ok(unpacked);

}

#[cfg(test)]
mod tests {
    use super::{pack_u32_to_u8, unpack_u8_to_u32};

    #[test]
    fn test_pack_u32_to_u8() {
        let expected: Vec<u8> = vec![0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
        let test_payload: [u32; 2] = [0x00000001, 0x00000002];
        let result = pack_u32_to_u8(&test_payload);
        assert_eq!(result, expected, "{:?} != {:?}", result, expected);
    }

    #[test]
    fn test_unpack_u8_to_u32() {
        let expected: u32 = 0x01020304;
        let packed: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let result = match unpack_u8_to_u32(&packed) {
            Ok(result) => result,
            Err(err) => assert!(false, err.to_string())
        };
    }
}
