use crate::HTTPMethods;

#[derive(Debug)]
pub struct HTTPRequest {
    method: HTTPMethods,
    path: String,
    version: String,
    headers: Option<Vec<HTTPHeaders>>,
    request_body: Option<String>
}

impl HTTPRequest {
    pub fn new() -> Self {
        Self {
            method: HTTPMethods::GET,
            path: String::from("/"),
            version: String::from("1.1"),
            headers: None,
            request_body: None
        }
    }

    pub fn with(method: String, path: String, version: String, headers: Option<Vec<HTTPHeaders>>, body:Option<String>) -> Self {
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
            version,
            headers,
            request_body: body
        }
    }
}

#[derive(Debug)]
pub struct HTTPHeaders {
    key: String,
    value: String
}

impl HTTPHeaders {
    pub fn with(key: String, value: String) -> Self {
        Self {
            key,
            value
        }
    }
}
