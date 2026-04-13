use std::{collections::HashMap, sync::LazyLock};

use crate::token::TokenValue;

/// Keyword Lookup Table (KLT)
pub(crate) static KEYWORDS: LazyLock<HashMap<&'static str, crate::TokenValue>> =
    LazyLock::new(|| {
        HashMap::from([
            ("break", TokenValue::Break),
            ("continue", TokenValue::Continue),
            ("destructor", TokenValue::Destructor),
            ("drop", TokenValue::Drop),
            ("else", TokenValue::Else),
            ("false", TokenValue::False),
            ("for", TokenValue::For),
            ("func", TokenValue::Func),
            ("if", TokenValue::If),
            ("nil", TokenValue::Nil),
            ("operator", TokenValue::Operator),
            ("override", TokenValue::Override),
            ("private", TokenValue::Private),
            ("public", TokenValue::Public),
            ("record", TokenValue::Record),
            ("return", TokenValue::Return),
            ("Self", TokenValue::SelfRecord),
            ("static", TokenValue::Static),
            ("true", TokenValue::True),
            ("use", TokenValue::Use),
            ("while", TokenValue::While),
        ])
    });
