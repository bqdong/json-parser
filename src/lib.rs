use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Bool(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(a) => {
                write!(f, "[")?;
                for (i, item) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item.to_string())?;
                }
                write!(f, "]")
            }
            JsonValue::Object(o) => {
                write!(f, "{{")?;
                for (i, (key, value)) in o.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value.to_string())?;
                }
                write!(f, "}}")
            }
        }
    }
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
        Json::Object(object) => JsonValue::Object(object).to_string(),
        Json::Array(array) => JsonValue::Array(array).to_string(),
    }
}

pub struct SpecialToken;

impl SpecialToken {
    pub const LeftBrace: char = '{';
    pub const RightBrace: char = '}';
    pub const LeftBracket: char = '[';
    pub const RightBracket: char = ']';
    pub const Comma: char = ',';
    pub const Colon: char = ':';
    pub const DoubleQuote: char = '"';

    // white space
    pub const Space: char = ' ';
    pub const NewLine: char = '\n';
    pub const Tab: char = '\t';
    pub const CarriageReturn: char = '\r';

    pub const Backslash: char = '\\';
}

pub fn parse(json: &str) -> Json {
    for c in json.chars() {
        println!("{}", c);
    }
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

    #[test]
    fn test_stringify_object() {
        let json = Json::Object(HashMap::from([
            ("key1".to_string(), JsonValue::String("value1".to_string())),
            ("key2".to_string(), JsonValue::Number(42f64)),
        ]));
        let result = stringify(json);
        assert_eq!(result, String::from("{\"key1\": \"value1\", \"key2\": 42}"));
    }
}
