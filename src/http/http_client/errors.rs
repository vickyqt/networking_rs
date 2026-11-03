use std::fmt::Display;

#[derive(Debug)]
pub enum TransportError {
    ConnectionFailuer,
    ConnectionRefusedFromServer,
    SocketReadFailuer,
    SocketWriteFailuer,
    SocketCloseFailuer,
    DnsFailuer,
}

#[derive(Debug)]
pub enum HttpProtocolError {
    // this needs implementation too.
    // what errors can protocal give to back
}

#[derive(Debug)]
pub enum Errors {
    Transport(TransportError),
    Protocol(HttpProtocolError),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(e) => write!(f, "Transport Error: {:?}", e),
            Self::Protocol(e) => write!(f, "HTTP Error: {:?}", e),
        }
    }
}

impl std::error::Error for Errors {}

impl From<std::io::Error> for Errors {
    fn from(value: std::io::Error) -> Self {
        match value.kind() {
            std::io::ErrorKind::ConnectionAborted => {
                Self::Transport(TransportError::ConnectionFailuer)
            }
            std::io::ErrorKind::ConnectionRefused => {
                Self::Transport(TransportError::ConnectionRefusedFromServer)
            }
            _ if value.to_string().contains("Name or service not known") => {
                Self::Transport(TransportError::DnsFailuer)
            }
            _ => Self::Transport(TransportError::SocketReadFailuer),
        }
    }
}
