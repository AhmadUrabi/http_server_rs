use crate::HTTPMethods;

#[derive(Debug)]
pub struct HTTPRequest {
    method: HTTPMethods,
    path: String,
    version: String,
    headers: Option<Vec<HTTPHeader>>,
    request_body: Option<String>,
}

impl HTTPRequest {
    pub fn new() -> Self {
        Self {
            method: HTTPMethods::GET,
            path: String::from("/"),
            version: String::from("1.1"),
            headers: None,
            request_body: None,
        }
    }

    pub fn with(
        method: String,
        path: String,
        version: String,
        headers: Option<Vec<HTTPHeader>>,
        body: Option<String>,
    ) -> Self {
        let method_str = method.as_str();
        let method = match method_str {
            "GET" => HTTPMethods::GET,
            "POST" => HTTPMethods::POST,
            "PUT" => HTTPMethods::PUT,
            "PATCH" => HTTPMethods::PATCH,
            "DELETE" => HTTPMethods::DELETE,
            "OPTIONS" => HTTPMethods::OPTIONS,
            _ => HTTPMethods::GET,
        };
        Self {
            method,
            path,
            version,
            headers,
            request_body: body,
        }
    }

    pub fn set_headers(&mut self, headers: Vec<String>) {
        let mut headers_obj: Vec<HTTPHeader> = Vec::new();
        for header in headers {
            let header_parts: Vec<&str> = header.split(":").collect();
            headers_obj.push(HTTPHeader::with(
                header_parts[0].to_string(),
                header_parts[1].to_string().trim().to_string(),
            ));
        }
        self.headers = Some(headers_obj);
    }

    pub fn set_body(&mut self, body: &String) {
        self.request_body = Some(body.clone());
    }

    pub fn is_empty(&self) -> bool {
        self.method == HTTPMethods::GET
            && self.path == "/"
            && self.version == "1.1"
            && self.headers.is_none()
            && self.request_body.is_none()
    }
}

#[derive(Debug)]
pub struct HTTPHeader {
    key: String,
    value: String,
}

impl HTTPHeader {
    pub fn with(key: String, value: String) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
