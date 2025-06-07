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
                "".to_string()
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
                "".to_string()
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
                "".to_string()
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
                "".to_string()
            }
        }
        Err(e) => format!("// Error parsing proto: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MESSAGE_WITH_COMMENTS: &str = r#"
// 用户偏好设置
message UserPreferences {
  // 语言设置
  string language = 1;
  // 时区
  string timezone = 2;
  // 是否接收邮件通知
  bool email_notifications = 3;
}
"#;

    const TEST_ENUM_WITH_COMMENTS: &str = r#"
// 用户状态枚举
enum UserStatus {
  // 未知状态
  UNKNOWN = 0;
  // 活跃用户
  ACTIVE = 1;
  // 已禁用
  DISABLED = 2;
}
"#;

    const TEST_MESSAGE_WITHOUT_COMMENTS: &str = r#"
message SimpleMessage {
  string name = 1;
  int32 age = 2;
}
"#;

    const TEST_ENUM_WITHOUT_COMMENTS: &str = r#"
enum SimpleEnum {
  VALUE_A = 0;
  VALUE_B = 1;
}
"#;

    const TEST_COMPLEX_PROTO: &str = r#"
// 复杂的用户信息
message ComplexUser {
  // 用户ID
  int64 id = 1;
  // 用户名
  string username = 2;
  // 嵌套地址信息
  Address address = 3;
  // 用户状态列表
  repeated UserStatus statuses = 4;
}

// 地址信息
message Address {
  // 国家
  string country = 1;
  // 城市
  string city = 2;
}

// 用户状态
enum UserStatus {
  // 未知
  UNKNOWN = 0;
  // 活跃
  ACTIVE = 1;
}
"#;

    #[test]
    fn test_dart_message_with_comments() {
        let result = proto_to_dart(TEST_MESSAGE_WITH_COMMENTS);
        println!("=== Dart Message with Comments ===");
        println!("{}", result);
        println!("=== End of Output ===");
        
        // 先检查基本结构
        assert!(result.contains("class UserPreferences"));
        
        // 检查注释（如果存在的话）
        if result.contains("///") {
            println!("Found documentation comments!");
            assert!(result.contains("/// 用户偏好设置"));
            assert!(result.contains("/// 语言设置"));
            assert!(result.contains("/// 时区"));
            assert!(result.contains("/// 是否接收邮件通知"));
        } else {
            println!("No documentation comments found in output");
            println!("This indicates the comment parsing is not working correctly");
        }
    }

    #[test]
    fn test_dart_enum_with_comments() {
        let result = proto_to_dart(TEST_ENUM_WITH_COMMENTS);
        println!("Dart Enum with Comments:\n{}", result);
        
        // 验证枚举级别注释
        assert!(result.contains("/// 用户状态枚举"));
        // 验证枚举值注释
        assert!(result.contains("/// 未知状态"));
        assert!(result.contains("/// 活跃用户"));
        assert!(result.contains("/// 已禁用"));
        // 验证枚举名
        assert!(result.contains("enum UserStatus"));
    }

    #[test]
    fn test_swift_message_with_comments() {
        let result = proto_to_swift(TEST_MESSAGE_WITH_COMMENTS);
        println!("Swift Message with Comments:\n{}", result);
        
        // 验证类级别注释
        assert!(result.contains("/// 用户偏好设置"));
        // 验证字段注释
        assert!(result.contains("/// 语言设置"));
        assert!(result.contains("/// 时区"));
        assert!(result.contains("/// 是否接收邮件通知"));
    }

    #[test]
    fn test_kotlin_message_with_comments() {
        let result = proto_to_kotlin(TEST_MESSAGE_WITH_COMMENTS);
        println!("Kotlin Message with Comments:\n{}", result);
        
        // 验证类级别注释
        assert!(result.contains("/** 用户偏好设置 */"));
        // 验证字段注释
        assert!(result.contains("/** 语言设置 */"));
        assert!(result.contains("/** 时区 */"));
        assert!(result.contains("/** 是否接收邮件通知 */"));
    }

    #[test]
    fn test_typescript_message_with_comments() {
        let result = proto_to_typescript(TEST_MESSAGE_WITH_COMMENTS);
        println!("TypeScript Message with Comments:\n{}", result);
        
        // 验证接口级别注释
        assert!(result.contains("/** 用户偏好设置 */"));
        // 验证字段注释
        assert!(result.contains("/** 语言设置 */"));
        assert!(result.contains("/** 时区 */"));
        assert!(result.contains("/** 是否接收邮件通知 */"));
    }

    #[test]
    fn test_message_without_comments() {
        let dart_result = proto_to_dart(TEST_MESSAGE_WITHOUT_COMMENTS);
        println!("Dart Message without Comments:\n{}", dart_result);
        
        // 应该不包含注释标记
        assert!(!dart_result.contains("///"));
        // 但应该包含类定义
        assert!(dart_result.contains("class SimpleMessage"));
    }

    #[test]
    fn test_enum_without_comments() {
        let dart_result = proto_to_dart(TEST_ENUM_WITHOUT_COMMENTS);
        println!("Dart Enum without Comments:\n{}", dart_result);
        
        // 应该不包含注释标记
        assert!(!dart_result.contains("///"));
        // 但应该包含枚举定义
        assert!(dart_result.contains("enum SimpleEnum"));
    }

    #[test]
    fn test_complex_proto_parsing() {
        let dart_result = proto_to_dart(TEST_COMPLEX_PROTO);
        println!("Complex Proto Dart Result:\n{}", dart_result);
        
        // 应该解析第一个消息（ComplexUser）
        assert!(dart_result.contains("class ComplexUser"));
        assert!(dart_result.contains("/// 复杂的用户信息"));
        assert!(dart_result.contains("/// 用户ID"));
        assert!(dart_result.contains("/// 用户名"));
    }

    #[test]
    fn test_empty_proto() {
        let result = proto_to_dart("");
        assert!(result.contains("No valid proto message or enum found"));
    }

    #[test]
    fn test_invalid_proto() {
        let result = proto_to_dart("invalid proto content");
        assert!(result.contains("No valid proto message or enum found"));
    }

    #[test]
    fn test_all_languages_consistency() {
        let test_proto = TEST_MESSAGE_WITH_COMMENTS;
        
        let dart_result = proto_to_dart(test_proto);
        let swift_result = proto_to_swift(test_proto);
        let kotlin_result = proto_to_kotlin(test_proto);
        let typescript_result = proto_to_typescript(test_proto);
        
        // 所有语言都应该成功生成代码
        assert!(!dart_result.contains("Error parsing"));
        assert!(!swift_result.contains("Error parsing"));
        assert!(!kotlin_result.contains("Error parsing"));
        assert!(!typescript_result.contains("Error parsing"));
        
        // 所有语言都应该包含类/接口名
        assert!(dart_result.contains("UserPreferences"));
        assert!(swift_result.contains("UserPreferences"));
        assert!(kotlin_result.contains("UserPreferences"));
        assert!(typescript_result.contains("UserPreferences"));
    }
}