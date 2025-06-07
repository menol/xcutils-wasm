use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MESSAGE_RE: Regex = Regex::new(r"message\s+(\w+)\s*\{([^{}]*(?:\{[^{}]*\}[^{}]*)*)\}").unwrap();
    static ref ENUM_RE: Regex = Regex::new(r"enum\s+(\w+)\s*\{([^{}]*)\}").unwrap();
    static ref FIELD_RE: Regex = Regex::new(
        r"(?://\s*(.*?)\s*\n)?\s*(repeated)?\s*(\w+)\s+(\w+)\s*=\s*(\d+)(?:\s*\[.*?\])?;(?:\s*//\s*(.*))?"
    ).unwrap();
    static ref VALUE_RE: Regex = Regex::new(
        r"(?://\s*(.*?)\s*\n)?\s*(\w+)\s*=\s*(\d+)(?:\s*\[.*?\])?;(?:\s*//\s*(.*))?"
    ).unwrap();
}

#[derive(Debug, Clone)]
pub struct ProtoField {
    pub original_name: String,
    pub type_name: String,
    pub is_repeated: bool,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProtoMessage {
    pub fields: Vec<ProtoField>,
}

#[derive(Debug, Clone)]
pub struct ProtoEnumValue {
    pub name: String,
    pub value: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProtoEnum {
    pub values: Vec<ProtoEnumValue>,
}

pub struct ProtoParserCore;

impl ProtoParserCore {
    pub fn parse_proto(proto_content: &str) -> Result<(HashMap<String, ProtoMessage>, HashMap<String, ProtoEnum>), String> {
        let mut messages = HashMap::new();
        let mut enums = HashMap::new();

        // 解析消息
        for cap in MESSAGE_RE.captures_iter(proto_content) {
            let name = cap[1].to_string();
            let body = cap[2].to_string();
            let fields = Self::parse_message_fields(&body)?;
            messages.insert(name, ProtoMessage { fields });
        }

        // 解析枚举
        for cap in ENUM_RE.captures_iter(proto_content) {
            let name = cap[1].to_string();
            let body = cap[2].to_string();
            let values = Self::parse_enum_values(&body)?;
            enums.insert(name, ProtoEnum { values });
        }

        Ok((messages, enums))
    }

    pub fn parse_message_fields(body: &str) -> Result<Vec<ProtoField>, String> {
        let mut fields = Vec::new();
        
        for line in body.lines() {
            if let Some(cap) = FIELD_RE.captures(line) {
                let comment = cap.get(1).map(|m| m.as_str().trim().to_string());
                let is_repeated = cap.get(2).is_some();
                let type_name = cap[3].to_string();
                let name = cap[4].to_string();
                let trailing_comment = cap.get(6).map(|m| m.as_str().trim().to_string());

                let full_comment = match (comment, trailing_comment) {
                    (Some(c), Some(t)) => Some(format!("{} {}", c, t)),
                    (Some(c), None) => Some(c),
                    (None, Some(t)) => Some(t),
                    (None, None) => None,
                };

                fields.push(ProtoField {
                    original_name: name,
                    type_name,
                    is_repeated,
                    comment: full_comment,
                });
            }
        }

        Ok(fields)
    }

    pub fn parse_enum_values(body: &str) -> Result<Vec<ProtoEnumValue>, String> {
        let mut values = Vec::new();
    
        for line in body.lines() {
            if let Some(cap) = VALUE_RE.captures(line) {
                let comment = cap.get(1).map(|m| m.as_str().trim().to_string());
                let name = cap[2].to_string();
                let value = cap[3].parse::<i32>().map_err(|e| format!("Invalid enum value: {}", e))?;
                let trailing_comment = cap.get(4).map(|m| m.as_str().trim().to_string());
    
                let full_comment = match (comment, trailing_comment) {
                    (Some(c), Some(t)) => Some(format!("{} {}", c, t)),
                    (Some(c), None) => Some(c),
                    (None, Some(t)) => Some(t),
                    (None, None) => None,
                };
    
                values.push(ProtoEnumValue {
                    name,
                    value,
                    comment: full_comment,
                });
            }
        }
    
        Ok(values)
    }

    pub fn to_camel_case(snake_str: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        
        for c in snake_str.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        
        result
    }

    pub fn to_pascal_case(snake_str: &str) -> String {
        let camel = Self::to_camel_case(snake_str);
        if let Some(first_char) = camel.chars().next() {
            first_char.to_uppercase().collect::<String>() + &camel[first_char.len_utf8()..]
        } else {
            camel
        }
    }

    pub fn to_snake_case(camel_str: &str) -> String {
        let mut result = String::new();
        
        for (i, c) in camel_str.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        }
        
        result
    }
}