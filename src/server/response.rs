use std::collections::HashMap;
use std::io::Error;
use std::io::prelude::*;
use std::net::TcpStream;


pub struct Response {
    pub route: String,
    pub action: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub raw: String,
}

impl Response {
    pub fn new(stream: &mut TcpStream) -> Result<Response, Error> {
        let mut buf = Vec::new();

        stream.read(&mut buf)?;

        let buf_clone = buf.clone();
        let buf_as_string = String::from_utf8_lossy(&buf_clone);


        let ok = "HTTP/1.1 200 OK\r\n";
        let h = "<html> <head></head> <body> hi </body> </html>";
        let res = format!("{}{}", ok, h);

        stream.write(res.as_bytes());

        Ok(Response {
            route: String::from("hello"),
            action: String::from("hello"),
            headers: HashMap::new(),
            query: HashMap::new(),
            raw: buf_as_string.to_string(),
        })
    }

    fn get_route(body: &str) -> String {
        "/get".to_string()
    }
}
