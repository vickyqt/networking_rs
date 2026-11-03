pub mod client;
pub mod errors;
pub mod http_protocol;
pub mod transport;

pub trait Transport {
    fn connect(&mut self, host: &str, port: u16) -> Result<(), errors::Errors>;
    fn read(&self, buff: &mut [u8]) -> Result<usize, errors::Errors>;
    fn write(&self, buff: &[u8]) -> Result<usize, errors::Errors>;
    fn close(&mut self) -> Result<(), errors::Errors>;
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
}

pub struct HttpHeaderView<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

pub struct HttpRequest<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub body: &'a [u8],
    pub header: Vec<HttpHeaderView<'a>>,
}

pub struct HttpResponce<'a> {
    pub status_code: u16,
    pub status_message: &'a str,

    pub headers: Vec<HttpHeaderView<'a>>,
    pub body: &'a [u8],

    pub content_length: Option<usize>,
}

pub trait HttpProtocol {
    // what the fuck this function doing?
    fn connect(&mut self, host: &str, port: u16) -> Result<(), errors::Errors>;
    fn dissconnect(&mut self) -> Result<(), errors::Errors>;
    fn perfrom_request<'a>(&mut self, request: &'a HttpRequest) -> Result<HttpResponce<'a>, errors::Errors>;
}
