use std::fmt;

#[derive(Protocol, Debug, Clone, PartialEq)]
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
