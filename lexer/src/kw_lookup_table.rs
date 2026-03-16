use std::{collections::HashMap, sync::LazyLock};

use crate::token::TokenValue;

/// Keyword Lookup Table (KLT)
pub(crate) static KEYWORDS: LazyLock<HashMap<&'static str, crate::TokenValue>> = LazyLock::new(|| {
    HashMap::from([
        ("func", TokenValue::Func),
        ("for", TokenValue::For),
        ("while", TokenValue::While),
        ("return", TokenValue::Return),
        ("public", TokenValue::Public),
        ("use", TokenValue::Use),
        ("null", TokenValue::Null),
        ("record", TokenValue::Record),
        ("self", TokenValue::SelfReference),
        ("Self", TokenValue::SelfRecord),
        ("static", TokenValue::Static),
        ("override", TokenValue::Override),
        ("operator", TokenValue::Operator),
        ("destructor", TokenValue::Destructor),
        ("drop", TokenValue::Drop),
    ])
});