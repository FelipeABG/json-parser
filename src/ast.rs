// <json> ::= <primitive> | <container>
//
// <primitive> ::= <number> | <string> | <boolean>
// ; Where:
// ; <number> is a valid real number expressed in one of a number of given formats
// ; <string> is a string of valid characters enclosed in quotes
// ; <boolean> is one of the literal strings 'true', 'false', or 'null' (unquoted)
//
// <container> ::= <object> | <array>
// <array> ::= '[' [ <json> *(', ' <json>) ] ']' ; A sequence of JSON values separated by commas
// <object> ::= '{' [ <member> *(', ' <member>) ] '}' ; A sequence of 'members'
// <member> ::= <string> ': ' <json> ; A pair consisting of a name, and a JSON value

pub enum JsonValue {
    Primitive(Primitive),
    Container(Container),
}

pub enum Primitive {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

pub enum Container {
    Object(Vec<Member>),
    Array(Vec<JsonValue>),
}

pub struct Member {
    name: String,
    value: JsonValue,
}
