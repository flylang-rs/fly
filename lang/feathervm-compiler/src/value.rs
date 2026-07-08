use std::collections::HashMap;

pub enum CompileTimeValue {
    String(String),
    BigInt(String),
}

impl CompileTimeValue {
    pub fn string(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }

    pub fn bigint(&self) -> Option<&str> {
        if let Self::BigInt(bi) = self {
            Some(bi.as_str())
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct CompileTimeValues {
    index_counter: usize,
    values: HashMap<usize, CompileTimeValue>,
}

impl CompileTimeValues {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_string(&mut self, string: String) -> usize {
        self.values
            .insert(self.index_counter, CompileTimeValue::String(string));

        let index = self.index_counter;

        self.index_counter += 1;

        index
    }

    pub fn add_bigint(&mut self, bigint: String) -> usize {
        self.values
            .insert(self.index_counter, CompileTimeValue::BigInt(bigint));

        let index = self.index_counter;

        self.index_counter += 1;

        index
    }

    pub fn find_string(&mut self, string: &str) -> Option<usize> {
        self.values
            .iter()
            .find(|&(_, v)| v.string().map(|s| s == string).unwrap_or(false))
            .map(|(i, _)| *i)
    }

    pub fn find_bigint(&mut self, bigint: &str) -> Option<usize> {
        self.values
            .iter()
            .find(|&(_, v)| v.bigint().map(|s| s == bigint).unwrap_or(false))
            .map(|(i, _)| *i)
    }

    pub fn find_or_add_string(&mut self, string: String) -> usize {
        if let Some(id) = self.find_string(&string) {
            id
        } else {
            self.add_string(string)
        }
    }

    pub fn find_or_add_bigint(&mut self, bigint: String) -> usize {
        if let Some(id) = self.find_bigint(&bigint) {
            id
        } else {
            self.add_bigint(bigint)
        }
    }
}
