pub mod structs;

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use structs::{
    http_request::{HTTPHeader, HTTPRequest},
    http_response::{HTTPResponse, HTTPStatus},
};

fn create_socket(config: String) -> TcpListener {
    return TcpListener::bind(config).unwrap();
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_headers: Vec<String> = Vec::new();
    let mut content_length = 0;
    let mut request = HTTPRequest::new();
    // Read headers
    for line in buf_reader.by_ref().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        // Read request line
        if request.is_empty() {
            let request_line = line.split(" ").collect::<Vec<&str>>();
            request = HTTPRequest::with(
                request_line[0].to_string(),
                request_line[1].to_string(),
                request_line[2].to_string(),
                None,
                None,
            );
            continue;
        }

        if line.starts_with("Content-Length:") {
            // content_length = usize::parse(line.split(':').nth(1).unwrap().trim()).unwrap();
            content_length = line.split(':').nth(1).unwrap().trim().parse().unwrap();
        }
        http_headers.push(line);
    }

    request.set_headers(http_headers);

    // Read body if Content-Length is set
    let mut body = Vec::new();
    if content_length > 0 {
        let mut handle = buf_reader.take(content_length as u64);
        handle.read_to_end(&mut body).unwrap();
    }

    let body_string = String::from_utf8(body).unwrap_or_else(|_| String::from("[binary data]"));
    request.set_body(&body_string);

    println!("{:?}", request);

    let my_response = HTTPResponse::new(
        request,
        HTTPStatus::OK,
        Some(vec![
            HTTPHeader::with("Content Length".to_string(), body_string.len().to_string()),
            HTTPHeader::with("Content-Type".to_string(), "application/json".to_string()),
        ]),
        Some(body_string),
    );
    stream.write_all(my_response.as_buf().as_bytes()).unwrap();
}
fn main() {
    let socket = create_socket("localhost:1024".to_string());
    for stream in socket.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}

#[derive(Debug, PartialEq)]
enum HTTPMethods {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
}

fn handle_request(stream: &TcpStream, request: HTTPRequest) {}
