use std::fmt;

#[derive(Protocol, Debug, Clone)]
pub struct DeString {
    _delimiter: u16,
    len: u16,
    #[protocol(length_prefix(elements("len")))]
    pub content: String,
}

impl fmt::Display for DeString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl PartialEq for DeString {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
            && self.len == other.len
            && self._delimiter == other._delimiter
    }
}

impl PartialEq<&str> for DeString {
    fn eq(&self, other: &&str) -> bool {
        &self.content == other
    }
}

impl PartialEq<String> for DeString {
    fn eq(&self, other: &String) -> bool {
        &self.content == other
    }
}
