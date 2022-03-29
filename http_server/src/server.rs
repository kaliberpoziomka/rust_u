//^ This file is a module - every file in rust is treated as a module

use crate::http::{Request, Response, StatusCode, ParseError}; // crate is a root of our crate (package)
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::{TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

//& Server is a struct, it's kinda like a clss
// data of a Server struct
pub struct Server {
    addr: String,
}
// functionality of a Server is in implementation
impl Server {
    // methods in a structs are functions that are defined in a contects of a struct, they take "self" argument. They are associated with an instance
    // associated functions are associated with struct, but they don't need an instance (like static method)

    // constructor for our struct new():
    // new() is associated function - it does not need an instance of a Struct
    pub fn new(addr: String) -> Self {// this could be Server 
        Server {
            addr // the same as addr: addr
        }
    }

    // method run
    // self points to an instance of a class, so it means it is an method, we call it <instance>.run();
    // self follows a normal rules of an ownership of variable, so we need to borrow it if we don't want to method run took ownership of a struct
    pub fn run(&self, mut handler: impl Handler) {
        println!("Listen on {}", &self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); // here returns Result <- result is Ok or Err
        // unwrap() -> if the result is ok returns listener, else it terminates a programm (unrecoverable error)

        loop { // the same as while true
            // match <any object> with patterns - similar to scala
            match listener.accept() {
                Ok((mut stream, _)) => {// unpacking a tuple, underscore is anything
                // \^^^ here mut because read() accepts &mut self
                    let mut buffer = [0; 1024]; // creating array with 1024 zeros, size is 1024 bytes
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) { // here we convert type &[u8; 1024] to type &[u8] by slicing it - we take whole byte array
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };
                            // let res: &Result<Request, _> = &buffer[..].try_into(); // that is another option to implement request convertino
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to parse a request: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read from connection: {}", e),//this basically never happens, because we match enything to Ok(_) and use from_utf8_lossy() method, which will parse even wrong encoding
                    }
                },
                // _ => println!() // underscore is catch all
                Err(e) => println!("Failed to establish a connection: {}", e), // print err and go to the next request in a loop
            }
        }
    }

}