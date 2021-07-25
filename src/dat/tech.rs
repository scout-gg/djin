use crate::dat::common::DeString;

const REQUIRED_TECH_SIZE: usize = 6;
const RESOURCE_COSTS_SIZE: usize = 3;

#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
pub struct TechResourcesCost {
    pub resource_type: ResourceCostType,
    pub amount: i16,
    pub flag: ResourceCostTrigger,
}

#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
#[protocol(discriminant = "integer")]
#[repr(u8)]
pub enum ResourceCostTrigger {
    OnCreate = 0,
    OnQueue = 1,
}

#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
#[protocol(discriminant = "integer")]
#[repr(u16)]
pub enum ResourceCostType {
    Food = 0,
    Wood = 1,
    Stone = 2,
    Gold = 3,
    /// Used only for Lithuanians unique bonus
    Relic = 7,
    // We cannot use i16 as enum discriminant but this is actually -1
    None = 65535,
    /// Used only for Cumans free elite Kipchaks team bonus
    Free = 215,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Techs {
    pub size: u16,
    #[protocol(length_prefix(elements(size)))]
    pub techs: Vec<Tech>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Tech {
    #[protocol(fixed_length(REQUIRED_TECH_SIZE))]
    pub required_techs: Vec<i16>,
    #[protocol(fixed_length(RESOURCE_COSTS_SIZE))]
    pub research_resource_cost: Vec<TechResourcesCost>,
    pub required_tech_count: i16,
    pub civ: i16,
    pub full_tech_mode: i16,
    pub research_location: i16,
    pub language_dll_name: i16,
    pub language_dll_description: i16,
    pub research_time: i16,
    pub effect_id: i16,
    pub r#type: i16,
    pub icon_id: i16,
    pub button_id: u8,
    pub language_dll_help: u32,
    pub language_dll_tech_tree: u32,
    pub hot_key: u32,
    pub name: DeString,
    pub repeatable: bool,
}
