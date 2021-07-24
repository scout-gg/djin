#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct DeString {
    _delimiter: u16,
    len: u16,
    #[protocol(length_prefix(elements("len")))]
    pub content: String,
}
