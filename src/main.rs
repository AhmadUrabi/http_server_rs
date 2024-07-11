use std::{io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};

fn create_socket(config: String) -> TcpListener {
    return TcpListener::bind(config).unwrap();
}

fn handle_stream(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        parse_request(http_request);
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let socket = create_socket("localhost:1024".to_string());
    for stream in socket.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}

#[derive(Debug)]
enum HTTPMethods {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS
}

#[derive(Debug)]
struct HTTPRequest {
    method: HTTPMethods,
    path: String,
    version: String,
}

impl HTTPRequest {
    pub fn new() -> Self {
        Self {
            method: HTTPMethods::GET,
            path: String::from("/"),
            version: String::from("1.1")
        }
    }

    pub fn with(method: String, path: String, version: String) -> Self {
        let method_str = method.as_str();
        let method = match method_str {
            "GET" => HTTPMethods::GET,
            "POST" => HTTPMethods::POST,
            "PUT" => HTTPMethods::PUT,
            "PATCH" => HTTPMethods::PATCH,
            "DELETE" => HTTPMethods::DELETE,
            "OPTIONS" => HTTPMethods::OPTIONS,
            _ => HTTPMethods::GET
        };
        Self {
            method,
            path,
            version
        }
    }
}

fn parse_request(data: Vec<String>) {
    let request = data[0].clone();
    let split: Vec<&str> = request.split(" ").collect();
    let req_obj = HTTPRequest::with(split[0].to_string(),split[1].to_string(),split[2].to_string());
    println!("{:?}", req_obj);
}
