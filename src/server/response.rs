use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Response {
    pub route: String,
    pub action: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub raw: String,
}

impl Response {
    pub fn new(stream: &mut TcpStream) -> Result<Response, Error> {
        let mut buf = String::new();

        loop {
            let mut buffer = vec![0; 256];
            stream.read(&mut buffer).unwrap();

            let buf_as_string = String::from_utf8_lossy(&mut buffer);
            let polished_buffer = buf_as_string.trim_matches('\u{0}');
            buf.push_str(polished_buffer);

            if buf_as_string.contains('\u{0}') {
                break;
            }
        }

        let buf_clone = buf.clone();
        let mut lines = buf_clone.lines();

        let first = match lines.next() {
            Some(v) => v,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Malformed Input")),
        };

        // let (action, route) = Response::parse_route(first)?;

        Ok(Response {
            route: "hi".to_string(),
            action: "hi".to_string(),
            headers: HashMap::new(),
            query: HashMap::new(),
            raw: buf,
        })
    }

    // fn parse_route(query: &str) -> Result<(String, String), Error> {
    //     let mut req = query.split_whitespace();
    //     let iters = req.take(2).collect();
        
    //     Ok((
    //         iters
    //     ))
    // }
}
