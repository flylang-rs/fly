use phf::Map;
use phf::phf_map;

use crate::token::TokenValue;

/// Keyword Lookup Table (KLT)
pub static KEYWORDS: Map<&'static str, crate::TokenValue> = phf_map! {
        "break" => TokenValue::Break,
        "continue" => TokenValue::Continue,
        "destructor" => TokenValue::Destructor,
        "drop" => TokenValue::Drop,
        "else" => TokenValue::Else,
        "false" => TokenValue::False,
        "for" => TokenValue::For,
        "func" => TokenValue::Func,
        "if" => TokenValue::If,
        "new" => TokenValue::New,
        "nil" => TokenValue::Nil,
        "operator" => TokenValue::Operator,
        "override" => TokenValue::Override,
        "private" => TokenValue::Private,
        "public" => TokenValue::Public,
        "record" => TokenValue::Record,
        "return" => TokenValue::Return,
        "Self" => TokenValue::SelfRecord,
        "static" => TokenValue::Static,
        "true" => TokenValue::True,
        "use" => TokenValue::Use,
        "while" => TokenValue::While,
};

pub fn tokenvalue_to_name(val: &TokenValue) -> Option<&'static str> {
    for (key, value) in KEYWORDS.entries() {
        if value == val {
            return Some(key);
        }
    }

    None
}
