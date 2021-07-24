use crate::dat::common::DeString;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Effects {
    size: u32,
    #[protocol(length_prefix(elements(size)))]
    pub effects: Vec<Effect>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Effect {
    name: DeString,
    command_size: i16,
    #[protocol(length_prefix(elements(command_size)))]
    commands: Vec<EffectCommand>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct EffectCommand {
    command_type: u8,
    a: i16,
    b: i16,
    c: i16,
    d: f32,
}
