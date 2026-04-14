use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub status_text: &'static str,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: 200,
            status_text: "OK",
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

impl Response {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_status(mut self, status: u16, text: &'static str) -> Self {
        self.status = status;
        self.status_text = text;
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self.headers
            .insert("Content-Length".to_string(), self.body.len().to_string());
        self
    }

    pub fn not_found() -> Self {
        Self::new()
            .with_status(404, "Not Found")
            .with_body("404 Not Found")
    }

    pub fn internal_server_error() -> Self {
        Self::new()
            .with_status(500, "Internal Server Error")
            .with_body("500 Internal Server Error")
    }

    pub fn bad_request() -> Self {
        Self::new()
            .with_status(400, "Bad Request")
            .with_body("400 Bad Request")
    }
}
