use std::{convert::TryFrom, error::Error, fmt::Display, str::Utf8Error};

use super::method::{Method, MethodError};
use super::Query;

#[derive(Debug)]
pub struct Request<'a> {
    path: &'a str,
    query: Option<Query<'a>>,
    method: Method,
}

impl<'a> Request<'a> {
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn method(&self) -> &Method {
        &&self.method
    }
    pub fn query(&self) -> Option<&Query> {
        self.query.as_ref()
    }
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;
    // GET /search?name=abcd&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
        let request = std::str::from_utf8(buf)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method = method.parse::<Method>()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(Query::from(&path[i + 1..]));
            path = &path[..i];
        }
        Ok(Self {
            path,
            query: query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}
