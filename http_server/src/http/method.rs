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