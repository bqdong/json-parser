use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub type JsonObject = HashMap<String, JsonValue>;
pub type JsonArray = Vec<JsonValue>;

#[derive(Debug)]
pub enum Json {
    Object(JsonObject),
    Array(JsonArray),
}

pub fn stringify<T>(data: T) -> String
where
    T: Into<Json>,
{
    let json: Json = data.into();
    match json {
        Json::Object(object) => {
            println!("object: {:?}", object);
        }
        Json::Array(array) => {
            println!("array: {:?}", array);
        }
    }
    unimplemented!()
}

pub fn parse(_json: &str) -> Json {
    unimplemented!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_stringify() {
        let json = Json::Array(Vec::new());
        let result = stringify(json);
        assert_eq!(result, String::from("[]"));
    }
}
