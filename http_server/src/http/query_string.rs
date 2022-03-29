use std::collections::HashMap;

//& Example of simple query string: a=1&b=2&c&d=&e===&d=7&d=abc - thats wy we need a complex structure to store those variables
// Define a new structure QueyString with data field. Data field is a HashMap (from std)
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}
// Enum Value is for storing value in HashMap, we need it to handle multiple or single values in QueryString::data hashmap
#[derive(Debug)] // this allow to use deb! on this object
pub enum Value<'buf> {// we create this, so we can store under the key one or mutliple vaules
    // Both Single and Multiple enum values, takes the same amount of space in memory, so we can swap it
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
    // /     ^^^ Vec is a heap allocated array - it can grow dynamically. We can allocate two elements, but when 3rd element pushed, it will allocate space for it
}
// Implementation of QueryString - we add a function to query data with a key
impl<'buf> QueryString<'buf> {
    // get() function returns an Option, because given key may not exist
    pub fn get(&self, key: &str) -> Option<&Value> {
        // get() method is implemented out-of-the-box in HashMap (because "data" field is a HashMap)
        self.data.get(key)
    } 
}
// Implementation of From trait to QueryString
// We implement only one function from() to transform query string to data HashMap<&'buf str, Value<'buf>>
impl<'buf> From<&'buf str> for QueryString<'buf> {
    // from() function takes "s" string slice (with lifetime buf) and returns Self - which is QueryString object, with a "data" field
    fn from(s: &'buf str) -> Self {
        // create data HashMap
        let mut data = HashMap::new();
        // iterate over a splitted by '&' string, put everythin in "key"
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            // and if there is a '=' sign, then split it for "key" and "value"
            if let Some(i) = key.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+'='.len_utf8()..];
            }
            // Here we need to add to "data" HashMap values under specific keys
            data.entry(key) // entry is a special method for in-map manipulations
                // If the key is already in data, we should modify it, with and_modify()
                .and_modify(|existing: &mut Value| {match existing { // match if "existing" Value is ::Single or ::Multiple
                //     here ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ we pass closure - anonymous function || {}, officualy FnOnce()
                    // "existing" matches Value::Single - that means that there was single value, but now we need to be multiple
                    // so we create a "vec" vector (whith vec! makro) and we add there new "val". New vector wrap in Value::Multiple
                    Value::Single(prev_val) => {// prev_val is simple string which was in Val::Single, and val is defined above
                    // / *existing - it means that under "existing" pointer, we want to allocate new value - because 
                    // we swap Value::Single with Value::Multiple, but pointer address stays the same
                    // This swapping is safe, because all the values in Value enum takes the same amount of space
                        *existing = Value::Multiple(vec![prev_val, val]);
                        // /                       ^^^^ vec! is makro for adding values to the vector
                    }
                    // if "existing" matches Value::Multiple - then simply push value to the Vector
                    Value::Multiple(vec) => vec.push(val)
                }})
                // If the key is not present in data, we add new value under this key
                .or_insert(Value::Single(val));
            
        }

        QueryString{
            data
        }
    }
}