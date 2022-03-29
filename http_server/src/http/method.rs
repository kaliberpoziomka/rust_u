use std::str::FromStr;
#[derive(Debug)]// this allow to use deb! on this object
pub enum Method {
    //& every member of an enum can have different type
    // GET(String),
    // DELETE(u64)
    //& and can have custom number associated to it
    // POST = 10
    GET, // 0
    DELETE, // 1 
    POST, // 2 and so on ..
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "DELETE" => Ok(Self::DELETE),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "HEAD" => Ok(Self::HEAD),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}
// We do not need to implement here nothing, becouse in request module we will be transforming it to the ParseError::InvalidMethod
pub struct MethodError;