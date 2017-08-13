use std::net::TcpStream;
use std::io::Error;
use std::io::prelude::*;
use std::collections::HashMap;

fn status_to_named(status: u16) -> String {
    let res = match status {
        100 => "Continue",
        200 => "OK",
        301 => "Moved Permanently",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        414 => "URI Too Long",
        418 => "I'm a teapot",
        500 => "Internal Server Error",
        _ => panic!("Code not implemented! (sorry!)"),
    };
    res.to_string()
}

pub struct Response {
    stream: TcpStream,
    pub headers: HashMap<String, String>,
    status: u16,
}

impl Response {
    /// Creates a new response object for interacting with the user.
    pub fn new(stream: TcpStream) -> Response {
        Response {
            stream: stream,
            headers: HashMap::new(),
            status: 200,
        }
    }

    /// sets the status code for the request, allows for chaining
    /// `response.status(200).send("Hello!")`
    pub fn status(&mut self, code: u16) -> &mut Response {
        self.status = code;
        self
    }

    pub fn send<T: ToString>(&mut self, body: T) -> Result<&mut Response, Error> {
        let payload = self.create_response(body.to_string());

        self.stream.write_all(payload.as_bytes())?;

        Ok(self)
    }

    fn create_response(&self, body: String) -> String {
        let headers = self.headers
            .iter()
            .map(|(key, value)| format!("{}: {}\r\n", key, value))
            .collect::<String>();

        format!(
            "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
            self.status,
            status_to_named(self.status),
            headers,
            body
        )
    }
}
