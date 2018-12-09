pub mod packer;

/*
use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};
use super::util::packer::{pack_u32_to_u8};

const CONNECT_PREAMBLE: u32 = 0x6060B017;
const SUPPORTED_VERSIONS: [u32; 1] = [0x1];

pub struct BoltConnection {
}


impl BoltConnection {
    pub fn connect<Addr: ToSocketAddrs>(server_address: Addr) -> Result<BoltConnection, String> {

        let mut stream = match TcpStream::connect(server_address) {
            Ok(stream) => stream,
            Err(err) => return Err(err.to_string()),
        };

        let to_pack: [u32; 5] = [CONNECT_PREAMBLE, SUPPORTED_VERSIONS[0], 0x0, 0x0, 0x0];
        let handshake_bytes = &pack_u32_to_u8(&to_pack)[..];
        let sent = match stream.write(handshake_bytes) {
            Ok(size) => size,
            Err(_) => return Err("Writing to the TCP socket caused an error".to_string())
        };

        if sent < 20 {
            return Err("Sent the wrong number of bytes".to_string());
        }
        
        let mut handshake_response: [u8; 4] = [0; 4];
        match stream.read_exact(&mut handshake_response) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string())
        };

        Ok(BoltConnection {})
    }
}*/
