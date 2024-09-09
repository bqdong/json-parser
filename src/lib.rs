use std::collections::HashMap;

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

pub enum Json {
    Object(JsonObject),
    Array(JsonArray),
}

pub fn stringify(json: Json) -> String {
    unimplemented!()
}

pub fn parse(json: &str) -> Json {
    unimplemented!()
}
