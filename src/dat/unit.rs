#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Units {
    pub unit_size: u32,
    #[protocol(length_prefix(elements(unit_size)))]
    pub unit_headers: Vec<UnitHeaders>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct UnitHeaders {
    pub exists: bool,
    #[protocol(skip_if("!exists"))]
    pub tasks_size: Option<TaskList>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TaskList {
    pub size: i16,
    #[protocol(length_prefix(elements(size)))]
    pub tasks: Vec<Task>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Task {
    pub task_type: i16,
    pub id: i16,
    pub is_default: u8,
    pub action_type: i16,
    pub class_id: i16,
    pub unit_id: i16,
    pub terrain_id: i16,
    pub resource_in: i16,
    pub resource_multiplier: i16,
    pub resource_out: i16,
    pub unused_resource: i16,
    pub work_value1: f32,
    pub work_value2: f32,
    pub work_range: f32,
    pub auto_search_targets: u8,
    pub search_wait_time: f32,
    pub enable_targeting: u8,
    pub combat_level_flag: u8,
    pub gather_type: i16,
    pub work_flag2: i16,
    pub target_diplomacy: u8,
    pub carry_check: u8,
    pub pick_for_construction: u8,
    pub moving_graphic_id: i16,
    pub proceeding_graphic_id: i16,
    pub working_graphic_id: i16,
    pub carrying_graphic_id: i16,
    pub resource_gathering_sound_id: i16,
    pub resource_deposit_sound_id: i16,
    pub wwise_resource_gathering_sound_id: u32,
    pub wwise_resource_deposit_sound_id: u32,
}
