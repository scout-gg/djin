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
    required_techs: Vec<i16>,
    #[protocol(fixed_length(RESOURCE_COSTS_SIZE))]
    research_resource_cost: Vec<TechResourcesCost>,
    required_tech_count: i16,
    civ: i16,
    full_tech_mode: i16,
    research_location: i16,
    language_dll_name: i16,
    language_dll_description: i16,
    research_time: i16,
    effect_id: i16,
    r#type: i16,
    icon_id: i16,
    button_id: u8,
    language_dll_help: u32,
    language_dll_tech_tree: u32,
    hot_key: u32,
    name: DeString,
    repeatable: u8,
}
