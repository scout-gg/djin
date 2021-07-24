const SLOT_SIZE: usize = 10;
const AGES: usize = 5;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TechTree {
    pub tech_tree_ages_size: u8,
    pub building_connections_size: u8,
    pub unit_connections_size: u8,
    pub research_connections_size: u8,
    pub total_unit_tech_groups: i32,
    #[protocol(length_prefix(elements(tech_tree_ages_size)))]
    pub tech_tree_ages: Vec<TechTreeAge>,
    #[protocol(length_prefix(elements(building_connections_size)))]
    pub building_connections: Vec<BuildingConnection>,
    #[protocol(length_prefix(elements(unit_connections_size)))]
    pub unit_connections: Vec<UnitConnection>,
    #[protocol(length_prefix(elements(research_connections_size)))]
    pub research_connections: Vec<ResearchConnection>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TechTreeAge {
    pub id: u32,
    pub status: u8,
    pub buildings_size: u8,
    #[protocol(length_prefix(elements(buildings_size)))]
    pub buildings: Vec<i32>,
    pub units_size: u8,
    #[protocol(length_prefix(elements(units_size)))]
    pub units: Vec<i32>,
    pub techs_size: u8,
    #[protocol(length_prefix(elements(techs_size)))]
    pub techs: Vec<i32>,
    pub common: TechTreeCommon,
    pub num_building_levels: u8,
    #[protocol(fixed_length(SLOT_SIZE))]
    pub buildings_per_zone: Vec<u8>,
    #[protocol(fixed_length(SLOT_SIZE))]
    pub group_length_per_zone: Vec<u8>,
    pub max_age_length: u8,
    pub line_mode: i32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct BuildingConnection {
    pub id: i32,
    pub status: u8,
    pub buildings_size: u8,
    #[protocol(length_prefix(elements(buildings_size)))]
    pub buildings: Vec<i32>,
    pub units_size: u8,
    #[protocol(length_prefix(elements(units_size)))]
    pub units: Vec<i32>,
    pub techs_size: u8,
    #[protocol(length_prefix(elements(techs_size)))]
    pub techs: Vec<i32>,
    pub common: TechTreeCommon,
    pub location_in_age: u8,
    #[protocol(fixed_length(AGES))]
    pub units_techs_total: Vec<u8>,
    #[protocol(fixed_length(AGES))]
    pub units_techs_first: Vec<u8>,
    pub line_mode: i32,
    pub enabling_research: i32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct UnitConnection {
    id: i32,
    status: u8,
    upper_building: i32,
    tech_tree_common: TechTreeCommon,
    vertical_line: u32,
    units_size: u8,
    #[protocol(length_prefix(elements(units_size)))]
    units: Vec<i32>,
    location_in_age: i32,
    required_research: i32,
    line_mode: i32,
    enabling_research: i32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct ResearchConnection {
    id: u32,
    status: u8,
    upper_building: u32,
    buildings_size: u8,
    #[protocol(length_prefix(elements(buildings_size)))]
    buildings: Vec<u32>,
    units_size: u8,
    #[protocol(length_prefix(elements(units_size)))]
    units: Vec<u32>,
    techs_size: u8,
    #[protocol(length_prefix(elements(techs_size)))]
    techs: Vec<u32>,
    tech_tree_common: TechTreeCommon,
    vertical_line: u32,
    location_in_age: u32,
    line_mode: u32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TechTreeCommon {
    slots_used: u32,
    /// Connection lines when selected
    #[protocol(fixed_length(SLOT_SIZE))]
    unit_research: Vec<u32>,
    /// 0 Age/Tech-level, 1 Building, 2 Unit, 3 Tech.
    #[protocol(fixed_length(SLOT_SIZE))]
    mode: Vec<TechTreeMode>,
}

#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
#[protocol(discriminant = "integer")]
#[repr(u32)]
pub enum TechTreeMode {
    AgeOrTechlevel = 0,
    Building = 1,
    Unit = 2,
    Tech = 3,
}
