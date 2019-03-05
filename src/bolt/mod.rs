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

        let mut server_agreed_version: [u8; 4] = [0; 4];
        match stream.read_exact(&mut server_agreed_version) {
            Ok(_) => (),
            Err(err) => return Err(ConnectionError::new(&err.to_string())),
        };

        debug!("Recieved response from server");

        Ok(BoltConnection{ stream: stream })
    }
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_connect() {
        
    }

}
