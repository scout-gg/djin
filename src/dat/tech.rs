use crate::dat::common::DeString;

const REQUIRED_TECH_SIZE: usize = 6;
const RESOURCE_COSTS_SIZE: usize = 3;

type TechResourcesCost = (i16, i16, u8);

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Techs {
    pub(crate) size: u16,
    #[protocol(length_prefix(elements(size)))]
    pub(crate) techs: Vec<Tech>,
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
    pub repeatable: u8,
}
