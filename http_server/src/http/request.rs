use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display,
               Debug,
               Result as FmtResult,
               Formatter,};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)] // this allow to use deb! on this object
pub struct Request<'buf> {
    // /            ^^^ 'buf is a lifetime specification
    path: &'buf str,
    query_string: Option<QueryString<'buf>>, // we use option, because this query can be absent
    method: Method, // we use super to show that method module is in parent module
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// implemented trait TryFrom is needed to your type conversion would be more compliant with good practise
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
//&  ^^^^ 'buf - we are giving metadata about the lifetime to the compiler. This 'buf lifetime we specify, is inferred by all the variables inside
    type Error = ParseError;

    // Example of http request:
    // GET /search?name=abc&sort=1 HTTP/1.1 
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
    // /        ^^^ buf is our incoming request string

        //& Questionmark syntax;
        /*
        Instead of this match syntax:
            match str::from_utf8(buf) {
                Ok(request) => {},
                Err(_) => return Err(ParseError::InvalidEncoding);
            };
        We put "?" at the end (where Result<T, E> is returned);
        Here we had to implement From for ParseError, so the "?" could convert UTF8Error to ParseError::InvalidEncoding
        */
        let request = str::from_utf8(buf)?;

        //& We can tranform Option to an Result with .ok_or()
        // and then we can use questionmark syntax
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        //^          ^^^^^^^ we use "request" as a name variable again, this is called "variable shadowing", this is not borrowing or changing an ownership, because we use "let" - new variable
        // if we shadow a variable, the previous one is not usable anymore 

        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+'?'.len_utf8()..])); // transform query_sttring to the QueryString object
            path = &path[..i];
        }

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        } else {
            return Ok(Self{
                path, 
                query_string,
                method});
        }
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    //& One way to loop over elements
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {},
    //         None => break
    //     }
    // }

    //& But this is simpler
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}



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

// This from conversions are needed for "?" syntax - we want to convert errors to the ParseError 
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        return Self::InvalidEncoding;
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        return Self::InvalidMethod;
    }
}


// Error trait for ParseError is not neccesary, but implementing it forces us to meet some basic expectations for error types
impl Error for ParseError {}

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