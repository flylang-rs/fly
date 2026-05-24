use crate::realm::Realm;

pub struct CallFrame {
    realm: Realm,
}

#[derive(Debug)]
pub struct CallFrameInfo {
    pub function_name: String, // name of the function currently executing
    pub from: Option<String>,
    pub call_site: CallSegment, // where in source this function was called from
}

#[derive(Debug)]
pub struct CallSegment {
    // Function name
    // pub func_name: String,

    // Where is it located, it's decomposed `Address`
    // Because (AFAIK) we can't precisely get line_col coords sometimes
    pub address_filename: String,
    pub address_line_col: Option<(usize, usize)>,
}
