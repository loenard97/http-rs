#[derive(Debug)]
pub enum Protocol {
    Get,
    Invalid,
}

impl Protocol {
    pub fn from_str(protocol: &str) -> Self {
        match protocol {
            "GET" => Self::Get,
            _ => Self::Invalid, 
        }
    }
}

#[derive(Debug)]
pub enum Version {
    Http1_1,
    Invalid,
}

impl Version {
    pub fn from_str(version: &str) -> Self {
        match version {
            "HTTP/1.1" => Self::Http1_1,
            _ => Self::Invalid, 
        }
    }
}

/// HTTP Request
#[derive(Debug)]
pub struct Request {
    pub protocol: Protocol,
    pub path: Option<String>,
    pub version: Version,
    pub host: Option<String>,
    pub user_agent: Option<String>,
    pub accept: Option<String>,
    pub accept_language: Option<String>,
    pub accept_encoding: Option<String>,
    pub dnt: Option<String>,
    pub connection: Option<String>,
    pub upgrade_insecure_requests: Option<String>,
    pub sec_fetch_dest: Option<String>,
    pub sec_fetch_mode: Option<String>,
    pub sec_fetch_site: Option<String>,
    pub sec_fetch_user: Option<String>,
}

impl Request {
    /// Create new HTTP Request from String
    pub fn from_string(message: String) -> Self {
        let mut buffer_iter = message.split("\r\n");

        let first_line = buffer_iter.next().unwrap();
        let mut first_line_split = first_line.split(" ");
        let protocol = Protocol::from_str(first_line_split.next().unwrap_or_default());
        let path = Some(first_line_split.next().unwrap_or_default().to_string());
        let version = Version::from_str(first_line_split.next().unwrap_or_default());

        let mut host = None;
        let mut user_agent = None;
        let mut accept = None;
        let mut accept_language = None;
        let mut accept_encoding = None;
        let mut dnt = None;
        let mut connection = None;
        let mut upgrade_insecure_requests = None;
        let mut sec_fetch_dest = None;
        let mut sec_fetch_mode = None;
        let mut sec_fetch_site = None;
        let mut sec_fetch_user = None;

        for line in buffer_iter {
            let line = line.to_string();
            let mut line_iter = line.split(": ");
            let name = line_iter.next().unwrap_or_default();
            let value = line_iter.next().unwrap_or_default();

            match name {
                "Host" => host = Some(String::from(value)),
                "User-Agent" => user_agent = Some(String::from(value)),
                "Accept" => accept = Some(String::from(value)),
                "Accept-Language" => accept_language = Some(String::from(value)),
                "Accept-Encoding" => accept_encoding = Some(String::from(value)),
                "DNT" => dnt = Some(String::from(value)),
                "Connection" => connection = Some(String::from(value)),
                "Upgrade-Insecure-Requests" => upgrade_insecure_requests = Some(String::from(value)),
                "Sec-Fetch-Dest" => sec_fetch_dest = Some(String::from(value)),
                "Sec-Fetch-Mode" => sec_fetch_mode = Some(String::from(value)),
                "Sec-Fetch-Site" => sec_fetch_site = Some(String::from(value)),
                "Sec-Fetch-User" => sec_fetch_user = Some(String::from(value)),
                _ => continue,
            }
        }

        Request { protocol, path, version, host, user_agent, accept, accept_language, accept_encoding, dnt, connection, upgrade_insecure_requests, sec_fetch_dest, sec_fetch_mode, sec_fetch_site, sec_fetch_user }
    }
}
