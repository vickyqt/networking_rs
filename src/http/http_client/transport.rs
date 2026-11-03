use super::{Transport, errors};
use std::{
    io::{BufReader, BufWriter, Read, Write},
    net::{Shutdown, TcpStream},
};

#[derive(Debug)]
pub struct TcpTransport {
    tcp_steam: Option<TcpStream>,
}

impl TcpTransport {
    pub fn new() -> Self {
        Self { tcp_steam: None }
    }
}

impl Transport for TcpTransport {
    fn connect(&mut self, host: &str, port: u16) -> Result<(), errors::Errors> {
        let address = format!("{}:{}", host, port);

        let socket_steam = TcpStream::connect(address)?;

        socket_steam
            .set_nodelay(true)
            .expect("how set_no_delay not ture?");

        self.tcp_steam = Some(socket_steam);
        Ok(())
    }
    fn read(&self, buff: &mut [u8]) -> Result<usize, errors::Errors> {
        if let Some(steam) = &self.tcp_steam {
            let mut bufreader = BufReader::new(steam);
            let total_read = bufreader.read(buff)?;
            Ok(total_read)
        } else {
            Err(errors::Errors::Transport(
                errors::TransportError::SocketReadFailuer,
            ))
        }
    }
    fn write(&self, buff: &[u8]) -> Result<usize, errors::Errors> {
        if let Some(steam) = &self.tcp_steam {
            let mut bufwriter = BufWriter::new(steam);
            let total_write = bufwriter.write(buff)?;
            Ok(total_write)
        } else {
            Err(errors::Errors::Transport(
                errors::TransportError::SocketWriteFailuer,
            ))
        }
    }
    fn close(&mut self) -> Result<(), errors::Errors> {
        if let Some(steam) = self.tcp_steam.take() {
            steam.shutdown(Shutdown::Both)?;
            Ok(())
        } else {
            Err(errors::Errors::Transport(
                errors::TransportError::SocketCloseFailuer,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        net::{SocketAddr, TcpListener},
        sync::mpsc,
        thread::{self, JoinHandle},
    };

    use super::*;

    fn server_setup<F>(server_logic: F) -> (SocketAddr, JoinHandle<()>)
    where
        F: FnOnce(TcpStream) + Send + 'static,
    {
        let tcp_stream = TcpListener::bind("127.0.0.1:0").unwrap();
        let local_addr = tcp_stream.local_addr().unwrap();

        let server_thread = thread::spawn(move || {
            if let Ok((server_steam, _)) = tcp_stream.accept() {
                server_logic(server_steam)
            }
        });

        (local_addr, server_thread)
    }

    #[test]
    fn is_connecting_to_server() {
        let (addr, server_thread) = server_setup(|_steam| {
            // no server logic, cuz nothing to do;
        });
        let mut tcp = TcpTransport::new();
        let result = tcp.connect(&addr.ip().to_string(), addr.port());

        assert!(result.is_ok());
        server_thread.join().unwrap();
    }

    #[test]
    fn writting_bytes_to_server() {
        let (sx, rx) = mpsc::channel();
        let message_to_send = "Hello World";

        let (addr, server_thread) = server_setup(move |mut server_steam| {
            let mut buffer = String::new();
            server_steam.read_to_string(&mut buffer).unwrap();
            sx.send(buffer).unwrap();
        });

        let mut tcp_connect = TcpTransport::new();
        tcp_connect
            .connect(&addr.ip().to_string(), addr.port())
            .unwrap();

        let write_size = tcp_connect.write(message_to_send.as_bytes()).unwrap();
        assert_eq!(message_to_send.len(), write_size);

        tcp_connect.close().unwrap();

        let from_server = rx.recv().unwrap();
        assert_eq!(message_to_send, from_server);
        server_thread.join().unwrap();
    }

    #[test]
    fn reading_bytes_from_server() {
        let reading_message = "Hello world";
        let (addr, server_tread) = server_setup(move |mut server_stream| {
            server_stream.write_all(reading_message.as_bytes()).unwrap();
        });

        let mut tcp_connect = TcpTransport::new();
        tcp_connect
            .connect(&addr.ip().to_string(), addr.port())
            .unwrap();

        let mut buffer = [0u8; 2048];
        let read_size = tcp_connect.read(&mut buffer).unwrap();
        assert_eq!(reading_message.len(), read_size);

        let form_server_message = std::str::from_utf8(&buffer[..read_size]).unwrap();
        assert_eq!(form_server_message, reading_message);

        server_tread.join().unwrap();
    }
}
