
/// HTTP Response Status
#[derive(Debug)]
pub enum Status {
    Ok,
    NotFound,
}

impl Status {
    /// Get Response Status as String
    pub fn to_string(self) -> String {
        match self {
            Status::Ok => "HTTP/1.1 200 OK",
            Status::NotFound => "HTTP/1.1 404 NOT FOUND",
        }.to_string()
    }
}

/// HTTP Response
pub struct Response {
    status: Status,
    content: Option<String>,
}

impl Response {
    pub fn new(content: Option<String>, status: Status) -> Self {
        Response { content, status }
    }

    pub fn to_string(self) -> String {
        match self.content {
            Some(content) => {
                format!(
                    "{}\r\nContent-Length: {}\r\n\r\n{}",
                    self.status.to_string(), content.len(), content
                )
            },
            None => {
                format!(
                    "{}\r\n\r\n",
                    self.status.to_string()
                )
            }
        }
    }
}
