//& Here we specify public interface for our module

// exposing to the outside Request and Method objects
pub use method::Method;
pub use request::Request;
pub use request::ParseError;
pub use query_string::{QueryString, Value as QueryStringValue};
pub use response::Response;
pub use status_code::StatusCode;

pub mod request;
pub mod method;
pub mod query_string;
pub mod response;
pub mod status_code;