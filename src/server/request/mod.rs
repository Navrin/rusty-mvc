use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::net::TcpStream;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub struct Request {
    pub route: String,
    pub action: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub raw: String,
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let headers = self.headers
            .iter()
            .map(|(k, v)| format!("{}: {}\n", k, v))
            .collect::<String>();
        let query = self.query
            .iter()
            .map(|(k, v)| format!("{}: {}\n", k, v))
            .collect::<String>();

        write!(
            f,
            "action: {}\nheaders: \"\"\"\n{}\"\"\"\nquery: {}\nraw: \"\"\"{}\"\"\"\n route: {}",
            self.action,
            headers,
            query,
            self.raw,
            self.route
        )
    }
}

impl Request {
    /// Creates a new request object
    /// This bundles the HTTP Request into one object for ease.
    pub fn new(mut stream: &mut TcpStream) -> Result<Request, Error> {
        let buf = Request::read_stream(&mut stream);

        let buf_clone = buf.clone();
        let mut lines = buf_clone.lines();

        let first = match lines.next() {
            Some(v) => v,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Malformed Input")),
        };

        let (action, route) = Request::parse_route(first)?;
        let headers = Request::parse_headers(&buf)?;

        Ok(Request {
            route,
            action,
            headers,
            query: HashMap::new(),
            raw: buf,
        })
    }

    fn read_stream(stream: &mut TcpStream) -> String {
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

        buf
    }

    fn parse_route(query: &str) -> Result<(String, String), Error> {
        let mut req = query.split_whitespace();

        let (method, path) = match (req.next(), req.next()) {
            (Some(m), Some(p)) => (m.to_string(), p.to_string()),
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Malformed Input")),
        };

        Ok((method, path))
    }

    fn parse_headers(query: &str) -> Result<HashMap<String, String>, Error> {
        let mut lines = query.lines();
        lines.next();
        let mut headers = HashMap::new();

        for line in lines {
            if line.len() < 3 {
                break;
            }

            let mut sep = line.split(":");
            let (key, value) = match (sep.next(), sep.next()) {
                (Some(m), Some(p)) => (m.to_string(), p.to_string()),
                _ => return Err(Error::new(ErrorKind::InvalidInput, "Malformed Input")),
            };
            headers.insert(key, value);
        }

        Ok(headers)
    }
}
