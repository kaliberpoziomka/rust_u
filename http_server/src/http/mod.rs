//& Here we specify public interface for our module

// exposing to the outside Request and Method objects
pub use method::Method;
pub use request::Request;
pub use request::ParseError;

pub mod request;
pub mod method;