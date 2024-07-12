pub mod structs;

use std::{io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};
use structs::http_request::{self, HTTPRequest};
use structs::http_request::HTTPHeaders;

fn create_socket(config: String) -> TcpListener {
    return TcpListener::bind(config).unwrap();
}


fn handle_stream(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_request: Vec<String> = Vec::new();
    let mut content_length = 0;

    // Read headers
    for line in buf_reader.by_ref().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        if line.starts_with("Content-Length:") {
            // content_length = usize::parse(line.split(':').nth(1).unwrap().trim()).unwrap();
            content_length = line.split(':').nth(1).unwrap().trim().parse().unwrap();
        }
        http_request.push(line);
    }

    // Read body if Content-Length is set
    let mut body = Vec::new();
    if content_length > 0 {
        let mut handle = buf_reader.take(content_length as u64);
        handle.read_to_end(&mut body).unwrap();
    }

    // Convert body to string if it's textual data
    let body_string = String::from_utf8(body).unwrap_or_else(|_| String::from("[binary data]"));
    println!("Request: {:?}\nBody: {:?}", http_request, body_string);

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


fn parse_request(data: &Vec<String>) {
    let request = data[0].clone();
    let split: Vec<&str> = request.split(" ").collect();

    let mut headers_obj: Vec<HTTPHeaders> = Vec::new();

    let headers: Vec<&str> = data[1..].iter().map(|x| x.as_str()).collect();
    for header in headers {
        let header_split: Vec<&str> = header.split(":").collect();
        let header_obj = HTTPHeaders::with(header_split[0].to_string(), header_split[1].trim_start().to_string());
        headers_obj.push(header_obj);
    }

    let req_obj = HTTPRequest::with(split[0].to_string(),split[1].to_string(),split[2].to_string(),Some(headers_obj), None);
    // println!("{:?}", req_obj);
}
