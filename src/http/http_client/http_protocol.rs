use crate::http::http_client::{
    HttpHeaderView, HttpMethod, HttpProtocol, HttpRequest, HttpResponce, Transport, errors,
};
use std::io::Write;

struct Http1Protocol<T: Transport> {
    trasnport: T,
    buffer: Vec<u8>,
    header_size: usize,
    content_length: Option<usize>,
}

impl<T: Transport + Default> Default for Http1Protocol<T> {
    fn default() -> Self {
        Self {
            trasnport: T::default(),
            buffer: Vec::new(),
            header_size: 0,
            content_length: None,
        }
    }
}

impl<T: Transport> Http1Protocol<T> {
    pub fn new(trasnport: T) -> Self {
        Self {
            trasnport,
            buffer: Vec::with_capacity(1024),
            content_length: None,
            header_size: 0,
        }
    }

    fn build_request_str(&mut self, request: &HttpRequest) {
        self.buffer.clear();
        let method_str = match request.method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
        };

        write!(
            &mut self.buffer,
            "{} {} HTTP/1.1\r\n",
            method_str, request.path
        )
        .unwrap();

        for header in &request.header {
            write!(&mut self.buffer, "{}: {}\r\n", header.key, header.value).unwrap();
        }

        if !request.body.is_empty() && request.method == HttpMethod::Post {
            self.buffer.extend_from_slice(request.body);
        }
    }
    fn read_full_responce(&mut self) -> Result<(), errors::Errors> {
        // give me clearead buffer and reset content_length and header_size,
        self.buffer.clear(); // the buffer len() is 0 at the starting of this function, so it does not have any writtable area
        self.content_length = None;
        self.header_size = 0;
        // as we don't know how many bytes arrived into the kernal buffer, we need to loop forever
        loop {
            let available_capacity = self.buffer.capacity() - self.buffer.len();
            let old_len = self.buffer.len();
            let read_ammout = std::cmp::max(available_capacity, 1024);
            self.buffer.resize(read_ammout + old_len, 0);

            let size = match self.trasnport.read(&mut self.buffer[old_len..]) {
                Err(errors::Errors::Transport(errors::TransportError::ConnectionFailuer)) => {
                    self.buffer.truncate(old_len); // to truncate old errors
                    if self.content_length.is_some()
                        && self.buffer.len() < self.content_length.unwrap() + self.header_size
                    {
                        return Err(errors::Errors::Protocol(
                            errors::HttpProtocolError::HttpParseError,
                        ));
                    }
                    break;
                }
                Err(e) => {
                    self.buffer.truncate(old_len);
                    return Err(e);
                }
                Ok(value) => value,
            };
            // now i have to parse for content length
            if self.header_size == 0 {
                if let Some(position) = self
                    .buffer
                    .windows(4)
                    .position(|window| window == b"\r\n\r\n")
                {
                    self.header_size = position + 4;
                    let header_view = &self.buffer[..self.header_size];
                    for line in header_view.split(|&x| x == b'\n').skip(1) {

                    }
                };
            }
        }
        Ok(())
    }

    fn parse_full_responce<'a>(&mut self) -> Result<HttpResponce<'a>, errors::Errors> {
        unimplemented!()
    }
}

impl<T: Transport> HttpProtocol for Http1Protocol<T> {
    fn connect(&mut self, host: &str, port: u16) -> Result<(), errors::Errors> {
        self.trasnport.connect(host, port)?;
        Ok(())
    }
    fn dissconnect(&mut self) -> Result<(), errors::Errors> {
        self.trasnport.close()?;
        Ok(())
    }

    fn perfrom_request<'a>(
        &mut self,
        request: &'a HttpRequest,
    ) -> Result<HttpResponce<'a>, errors::Errors> {
        self.build_request_str(&request);
        self.trasnport.write(&self.buffer)?;
        self.read_full_responce()?;
        self.parse_full_responce()
        // read the responce from the server,
        // parse the responce from the server,
        // return the httpresponce
    }
}
