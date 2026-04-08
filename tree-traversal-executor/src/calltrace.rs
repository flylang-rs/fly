#[derive(Debug)]
pub struct CallFrame {
    // Function name where we currently are.
    pub func_name: String,

    // Where is it located, it's decomposed `Address`
    // Because (AFAIK) we can't precisely get line_col coords sometimes
    pub address_filename: String,
    pub address_line_col: Option<(usize, usize)>,
}