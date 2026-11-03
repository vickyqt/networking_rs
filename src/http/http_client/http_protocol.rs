use crate::http::http_client::{
    HttpHeaderView, HttpMethod, HttpProtocol, HttpRequest, HttpResponce, Transport, errors,
};
use std::io::Write;

struct Http1Protocol<T: Transport> {
    trasnport: T,
    buffer: Vec<u8>, // this vec buffer will contain the http request coming or going..
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
        }
            write!(&mut self.buffer, "{}: {}\r\n", header.key, header.value).unwrap();

        if !request.body.is_empty() && request.method == HttpMethod::Post {
            self.buffer.extend_from_slice(request.body);
        }
    }
    fn read_full_responce(&mut self) -> Result<(), errors::Errors> {
        loop {
            let size = match self.trasnport.read(&mut self.buffer) {
                Err(errors::Errors::Transport(errors::TransportError::ConnectionFailuer)) => {
                    println!("Connection Failer");
                    break;
                },
                Err(e) => return Err(e),
                Ok(value) => value,
            };
            for i in 0..size {

            }
        }
        Ok(())
    }
    fn parse_full_responce<'a>(&mut self) -> Result<HttpResponce<'a>, errors::Errors> {}
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
