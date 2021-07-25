use crate::dat::common::DeString;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Effects {
    size: u32,
    #[protocol(length_prefix(elements(size)))]
    pub effects: Vec<Effect>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Effect {
    pub name: DeString,
    pub command_size: i16,
    #[protocol(length_prefix(elements(command_size)))]
    pub commands: Vec<EffectCommand>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct EffectCommand {
    pub command_type: u8,
    pub a: i16,
    pub b: i16,
    pub c: i16,
    pub d: f32,
}
