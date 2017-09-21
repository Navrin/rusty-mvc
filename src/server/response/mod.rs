use std::net::TcpStream;
use std::fs::File;
use std::path::Path;
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

pub trait Pathable {
    fn parse(&self) -> String;
}

impl<T: ToString> Pathable for T {
    fn parse(&self) -> String {
        self.to_string()
    }
}

impl Pathable for Path {
    fn parse(&self) -> String {
        self.to_string_lossy().to_string()
    }
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

    // sets the content type header. allows for easier method chaining like
    /// `res.content_type("application/json").send(json!{ "hello": "world" })`
    pub fn content_type<T: ToString>(&mut self, setting: T) -> &mut Response {
        self.headers.insert("Content-Type".to_string(), setting.to_string());
        self
    }

    /// Sends the body payload to the stream response
    pub fn send<T: ToString>(&mut self, body: T) -> Result<&mut Response, Error> {
        let payload = self.create_response(body.to_string());

        self.stream.write_all(payload.as_bytes())?;

        Ok(self)
    }

    /// Sends a file as the response. chain with `.content_type` to set the proper header
    /// By default, `.` is the project root. eg `src/ Cargo.lock Cargo.toml`. Use a static path for more clarity.
    pub fn send_file<T: Pathable>(&mut self, path: T) -> Result<&mut Response, Error> {
        let mut contents = String::new();

        let path = path.parse();
        let mut file = File::open(path)?;
        file.read_to_string(&mut contents)?;

        self.send(contents)
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
