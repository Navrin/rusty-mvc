use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::net::TcpStream;
use std::fmt::{Display, Formatter};
use std::fmt;

impl Display for Request {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let headers = self.headers
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}: {}\n",
                    k,
                    v,
                )
            })
            .collect::<String>();
        let query = match self.query.clone() {
            Some(query) => 
                query
                .iter()
                .map(|(k, v)| format!("{}: {}\n", k, 
                    match v.clone() {
                        Some(v) => v,
                        None => "none".to_string(),
                    }
                 ))
                .collect::<Vec<String>>()
                .join(""),
            None => "no query".to_string(),
        };

        write!(
            f,
            "method: {}\nheaders: \"\"\"\n{}\"\"\"\nquery: \n\"\"\"\n{}\"\"\"\nraw: \"\"\"{}\"\"\"\n route: {}",
            self.method,
            headers,
            query,
            self.raw_full,
            self.route
        )
    }
}

#[derive(Debug)]
pub struct Request {
    /// The route without the query parameters
    pub route: String, 
    /// What HTTP method it was requested with, GET POST PATCH etc
    pub method: String,
    /// A hashmap representation of the HTTP parmeters.
    pub headers: HashMap<String, String>,
    /// The URI query in a hashmap representation.
    pub query: Option<HashMap<String,  Option<String>>>,
    /// The URI parameters in a hashmap
    /// `/dog/:id/` -> `/dog/10` = `{ id => 10 }`
    pub params: Option<HashMap<String, String>>,
    /// the entire request in string form
    pub raw_full: String,
    /// just the headers in string form
    pub raw_headers: String,
    /// the body of the request in string form.
    pub body: Option<String>,
}


impl Request {
    /// Creates a new request object
    /// This bundles the HTTP Request into one object for ease.
    pub fn new(mut stream: &mut TcpStream) -> Result<Request, Error> {
        let buf = Request::read_stream(&mut stream);

        let buf_clone = buf.clone();
        let mut lines = buf_clone.lines();

        // gets the first line of the HTTP request
        let first = match lines.next() {
            Some(v) => v,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Malformed Input")),
        };

        let (method, route, maybe_query) = Request::parse_route(first)?;
        let headers = Request::parse_headers(&buf)?;

        let query = match maybe_query {
            Some(v) => Request::parse_query(&v),
            None => None
        };

        // sep the headers and the body
        let mut req = buf_clone.split("\r\n\r\n");

        let raw_headers = match req.next() {
            Some(v) => v.to_string(),
            None => return Err(Error::new(ErrorKind::InvalidInput, "Malformed Input")),
        };

        let body = match req.next() {
            Some(v) => Some(v.to_string()),
            None => None,
        };

        Ok(Request {
            route,
            method,
            headers,
            query,
            params: None, // params like `dog/:id` are handled by the router, added after parsing
            body,
            raw_headers,
            raw_full: buf,
        })
    }

    /// Read the entire stream into a string
    /// TODO: Impl an request limit size to prevent overflow attacks.
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


    /// Reads the first line of a HTTP response, and returns a tuple of
    /// `(METHOD, PATH, QUERY?)`
    fn parse_route(query: &str) -> Result<(String, String, Option<String>), Error> {
        let mut req = query.split_whitespace();

        let (method, path) = match (req.next(), req.next()) {
            (Some(m), Some(p)) => (m.to_string(), p.to_string()),
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Malformed Input")),
        };

        let mut path = path.split_terminator('?');
        let (path, maybe_query) = match (path.next(), path.next()) {
            (Some(p), Some(q)) => (p.to_string(), Some(q.to_string())),
            (Some(p), None) => (p.to_string(), None),
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Malformed Input")),
        };

        Ok((method, path, maybe_query))
    }

    
    /// Turns the HTTP headers into a HashMap with the formatting as
    /// `HashMap<HEADER_KEY, HEADER_VALUE>`
    /// If the headers are formed badly, return an error.
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

    /// Turns a query string like `?foo=bar;baz` into a hasmap
    /// The HashMap is formatted as 
    /// `HashMap<String, Option<String>>.`
    /// A none value signifies the query is just `?something` without a value.
    fn parse_query(query_path: &str) -> Option<HashMap<String, Option<String>>> {
        let mut queries: HashMap<String, Option<String>> = HashMap::new();
        let seperated = query_path.split(|c| c == '&' || c == ';');

        for query in seperated {
            let mut query = query.split('=');
            match (query.next(), query.next()) {
                (Some(k), Some(v)) => {
                    queries.insert(k.to_string(), Some(v.to_string()));
                }
                (Some(k), None) => {
                    queries.insert(k.to_string(), None);
                }
                _ => continue,
            }
            continue;
        }

        Some(queries)
    }
}
