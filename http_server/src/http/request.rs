use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display,
               Debug,
               Result as FmtResult,
               Formatter,};

pub struct Request {
    path: String,
    query_string: Option<String>, // we use option, because this query can be absent
    method: Method, // we use super to show that method module is in parent module
}



// implemented trait TryFrom is needed to your type conversion would be more professional
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // Example of http request:
    // GET /search?name=abc&sort=1 HTTP/1.1 
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

// We need to implement Display and Debug traits for ParseError, because Error trait expect this 
// They let us change a functionality of displaying
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

// Error trait for ParseError is not neccesary, but implementing it forces us to meet some basic expectations for error types
impl Error for ParseError {}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,// if request is not utf-8 encoded
    InvalidProtocol,// only HTPP /1.1 protocol accepted
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
           Self::InvalidRequest => "InvalidRequest",
           Self::InvalidEncoding => "InvalidEncoding",
           Self::InvalidProtocol => "InvalidProtocol",
           Self::InvalidMethod => "InvalidMethod",
        } // this is the last expression in a function body, so the value matched is returned
    }
}


/*
& Traits can extend functionality, even of crates not created by us

^Here we define trait Encrypt

trait Encrypt {
    fn encrypt(&self) -> Self;
}

^We can extend functionality of String struct with our Encrypt functionality
impl Encrypt for String {
    fn encrypt(&self) -> Self {
        unimplemented!()
    }
}

^So now we can use encypt() function in strings
fn somefn() {
    let a = String::fro("abc");
    a.encrypt();
}


*/