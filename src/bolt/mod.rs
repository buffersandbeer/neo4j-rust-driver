use std::io::prelude::*;
use std::net::{TcpStream, ToSocketAddrs};
use super::errors::{ConnectionError};

const CONNECT_PREAMBLE: u32 = 0x6060B017;
const SUPPORTED_VERSIONS: [u32; 1] = [1];

pub mod packer;

pub struct BoltConnection {
    pub stream: TcpStream,
}

/// Create and manage a bolt connection
///
/// ```
/// use neo4j::bolt::{BoltConnection};
/// # use neo4j::errors::{ConnectionError};
/// # fn test_bolt_connection() -> Result<BoltConnection, ConnectionError> {
///
/// let connection = match BoltConnection::connect("127.0.0.1:7687") {
///     Ok(con) => con,
///     Err(err) => return Err(err),
/// };
/// # Ok(connection)
/// # }
impl BoltConnection {
    
    /// Connect to the bolt server and return a BoltConnection struct
    ///
    /// ```
    /// use neo4j::bolt::{BoltConnection};
    /// # use neo4j::errors::{ConnectionError};
    /// # fn test_connect() -> Result<BoltConnection, ConnectionError> {
    ///
    /// let connection = match BoltConnection::connect("127.0.0.1:7687") {
    ///     Ok(con) => con,
    ///     Err(err) => return Err(err),
    /// };
    /// #
    /// # Ok(connection)
    /// # }
    /// ```
    pub fn connect<Addr: ToSocketAddrs>(server_address: Addr) -> Result<BoltConnection, ConnectionError> {
        
        let mut stream = match TcpStream::connect(server_address) {
            Ok(stream) => stream,
            Err(err) => return Err(ConnectionError::new(&err.to_string())),
        };

        let versions: [u32; 4] = [SUPPORTED_VERSIONS[0], 0, 0, 0];
        let handshake_payload = packer::pack_handshake(CONNECT_PREAMBLE, versions);

        debug!("Sending handshake");

        let sent = match stream.write(&handshake_payload) {
            Ok(size) => size,
            Err(err) => return Err(ConnectionError::new(&err.to_string())),
        };

        if sent < 20 {
            return Err(ConnectionError::new("Incorrect sent handshake bytes"));
        }

        let mut server_agreed_version_packed: [u8; 4] = [0; 4];
        match stream.read_exact(&mut server_agreed_version_packed) {
            Ok(_) => (),
            Err(err) => return Err(ConnectionError::new(&err.to_string())),
        };
        
        debug!("Recieved response from server");
        let server_agreed_version: u32 = packer::u8_to_u32_be(server_agreed_version_packed);
        if server_agreed_version < 1 {
            return Err(ConnectionError::new("Server does not support requested protocol version(s)"));
        }
        Ok(BoltConnection{ stream: stream })
    }

}

#[cfg(test)]
mod tests {
    use std::{thread, time};
    use std::net::{TcpListener};
    use std::io::{Write, Read};
    use super::{BoltConnection};
    use super::packer::{u8_to_u32_be};

    #[test]
    fn test_connect() {

        let test_server = thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:17687").unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {

                        let mut request_buffer: [u8; 20] = [0; 20];
                        stream.read(&mut request_buffer).unwrap();
                        let mut unpacked_request: [u32; 5] = [0; 5];
                        for iter in 0..5 {
                            unpacked_request[iter] = u8_to_u32_be([request_buffer[iter + 0], 
                                                                  request_buffer[iter + 1], 
                                                                  request_buffer[iter + 2], 
                                                                  request_buffer[iter + 3]]);
                        }
                        if unpacked_request[0] != 0x6060B017 {
                            stream.write(&[0u8; 4]).unwrap(); 
                        }
                        if unpacked_request[1] != 1 && 
                           unpacked_request[2] != 1 &&
                           unpacked_request[3] != 1 &&
                           unpacked_request[4] != 1 {
                            stream.write(&[0u8; 4]).unwrap(); 
                        }


                        let version: [u8; 4]  = [0,0,0,1];
                        stream.write(&version).unwrap();
                        break;
                    }
                    Err(e) => {
                        panic!("Error: {}", e);
                    }
                }
            }
        });
        
        let ten_mil = time::Duration::from_millis(10);
        thread::sleep(ten_mil);
        let _connection = match BoltConnection::connect("127.0.0.1:17687") {
            Ok(con) => con,
            Err(err) => panic!("Error: {}", err),
        };

        test_server.join().unwrap();
    }

    #[test]
    fn test_connect_unsupported_ver_response() {
        let test_server = thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:17688").unwrap();
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        stream.write(&[0u8; 4]).unwrap(); 
                        break;
                    }
                    Err(e) => {
                        panic!("Error: {}", e);
                    }
                }
            }
        });
        
        let ten_mil = time::Duration::from_millis(10);
        thread::sleep(ten_mil);
        let _connection = match BoltConnection::connect("127.0.0.1:17688") {
            Ok(_con) => assert!(false),
            Err(_err) => (),
        };
        test_server.join().unwrap();
    }

}
