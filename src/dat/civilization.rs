use crate::dat::common::DeString;
use protocol::{Parcel, Settings, Error};
use std::io::{Read, Write};
use protocol::hint::Hints;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Civilizations {
    pub size: u16,
    #[protocol(length_prefix(elements(size)))]
    pub civilizations: Vec<Civilization>
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Civilization {
    player_type: u8,
    name: DeString,
    resource_size: u16,
    tech_tree_id: u16,
    team_bonus_id: u16,
    resources: Vec<f32>,
    icon_set: u8,
    units_size: u16,
    unit_pointers: Vec<u32>,
    units: Vec<Unit>,
}


#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Unit {
    dd: u8,
}