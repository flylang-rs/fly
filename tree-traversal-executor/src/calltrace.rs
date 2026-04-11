#[derive(Debug)]
pub struct CallFrame {
    pub what_calls: CallSegment
}

#[derive(Debug)]
pub struct CallSegment {
    // Function name
    pub func_name: String,

    // Where is it located, it's decomposed `Address`
    // Because (AFAIK) we can't precisely get line_col coords sometimes
    pub address_filename: String,
    pub address_line_col: Option<(usize, usize)>,

}