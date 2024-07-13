use super::http_request::{HTTPHeader, HTTPRequest};

pub struct HTTPResponse {
    request: HTTPRequest,
    status: HTTPStatus,
    headers: Option<Vec<HTTPHeader>>,
    body: Option<String>,
}

impl HTTPResponse {
    pub fn new(
        request: HTTPRequest,
        status: HTTPStatus,
        headers: Option<Vec<HTTPHeader>>,
        body: Option<String>,
    ) -> HTTPResponse {
        HTTPResponse {
            request,
            status,
            headers,
            body,
        }
    }

    pub fn as_buf(&self) -> String {
        let mut response = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status.get().1,
            self.status.get().0
        );
        if let Some(headers) = &self.headers {
            for header in headers {
                response.push_str(&format!("{}: {}\r\n", header.key(), header.value()));
            }
        }
        response.push_str("\r\n");
        if let Some(body) = &self.body {
            response.push_str(body);
        }
        response
    }
}

pub enum HTTPStatus {
    OK,
    Created,
    Accepted,
    NoContent,
    MovedPermanently,
    MovedTemporarily,
    NotModified,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
}

impl HTTPStatus {
    pub fn get(&self) -> (String, u16) {
        match self {
            HTTPStatus::OK => ("OK".to_string(), 200),
            HTTPStatus::Created => ("Created".to_string(), 201),
            HTTPStatus::Accepted => ("Accepted".to_string(), 202),
            HTTPStatus::NoContent => ("No Content".to_string(), 204),
            HTTPStatus::MovedPermanently => ("Moved Permanently".to_string(), 301),
            HTTPStatus::MovedTemporarily => ("Moved Temporarily".to_string(), 302),
            HTTPStatus::NotModified => ("Not Modified".to_string(), 304),
            HTTPStatus::BadRequest => ("Bad Request".to_string(), 400),
            HTTPStatus::Unauthorized => ("Unauthorized".to_string(), 401),
            HTTPStatus::Forbidden => ("Forbidden".to_string(), 403),
            HTTPStatus::NotFound => ("Not Found".to_string(), 404),
            HTTPStatus::MethodNotAllowed => ("Method Not Allowed".to_string(), 405),
            HTTPStatus::InternalServerError => ("Internal Server Error".to_string(), 500),
        }
    }
}
