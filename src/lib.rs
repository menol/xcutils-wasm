use wasm_bindgen::prelude::*;

mod converter {
    pub mod core;
    pub mod swift;
    pub mod typescript;
    pub mod kt;
    pub mod dart;
}
use converter::{swift, typescript,kt,dart,core::ProtoParserCore};

#[wasm_bindgen]
pub fn proto_to_swift(proto_content: &str) -> String {
    match ProtoParserCore::parse_proto(proto_content) {
        Ok((messages, enums)) => {
            if let Some((name, message)) = messages.iter().next() {
                swift::generate_class(name, message)
            } else if let Some((name, proto_enum)) = enums.iter().next() {
                swift::generate_enum(name, proto_enum)
            } else {
                "// No valid proto message or enum found".to_string()
            }
        }
        Err(e) => format!("// Error parsing proto: {}", e),
    }
}

#[wasm_bindgen]
pub fn proto_to_kotlin(proto_content: &str) -> String {
    match ProtoParserCore::parse_proto(proto_content) {
        Ok((messages, enums)) => {
            if let Some((name, message)) = messages.iter().next() {
                kt::generate_class(name, message)
            } else if let Some((name, proto_enum)) = enums.iter().next() {
                kt::generate_enum(name, proto_enum)
            } else {
                "// No valid proto message or enum found".to_string()
            }
        }
        Err(e) => format!("// Error parsing proto: {}", e),
    }
}

#[wasm_bindgen]
pub fn proto_to_typescript(proto_content: &str) -> String {
    match ProtoParserCore::parse_proto(proto_content) {
        Ok((messages, enums)) => {
            if let Some((name, message)) = messages.iter().next() {
                typescript::generate_interface(name, message)
            } else if let Some((name, proto_enum)) = enums.iter().next() {
                typescript::generate_enum(name, proto_enum)
            } else {
                "// No valid proto message or enum found".to_string()
            }
        }
        Err(e) => format!("// Error parsing proto: {}", e),
    }
}

#[wasm_bindgen]
pub fn proto_to_dart(proto_content: &str) -> String {
    match ProtoParserCore::parse_proto(proto_content) {
        Ok((messages, enums)) => {
            if let Some((name, message)) = messages.iter().next() {
                dart::generate_class(name, message)
            } else if let Some((name, proto_enum)) = enums.iter().next() {
                dart::generate_enum(name, proto_enum)
            } else {
                "// No valid proto message or enum found".to_string()
            }
        }
        Err(e) => format!("// Error parsing proto: {}", e),
    }
}