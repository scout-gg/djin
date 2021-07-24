use crate::dat::common::DeString;
use crate::dat::unit::Task;
use crate::dat::ResourceUsage;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Civilizations {
    pub size: u16,
    #[protocol(length_prefix(elements(size)))]
    pub civilizations: Vec<Civilization>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Civilization {
    pub player_type: u8,
    pub name: DeString,
    pub resource_size: u16,
    pub tech_tree_id: i16,
    pub team_bonus_id: u16,
    #[protocol(length_prefix(elements(resource_size)))]
    pub resources: Vec<f32>,
    pub icon_set: u8,
    pub units_pointers_size: u16,
    #[protocol(length_prefix(elements(units_pointers_size)))]
    pub unit_pointers: Vec<u32>,
    #[protocol(length_prefix(pointers(unit_pointers)))]
    pub units: Vec<Unit>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Unit {
    pub unit_type: UnitType,
    pub id: i16,
    pub language_dll_name: i32,
    pub language_dll_creation: i32,
    pub class: i16,
    pub standing_graphics: (i16, i16),
    pub dying_graphic: i16,
    pub undead_graphic: i16,
    pub undead_mode: u8,
    pub hit_points: i16,
    pub line_of_sight: f32,
    pub garrison_capacity: u8,
    pub collision_box: (f32, f32, f32),
    pub train_sound: i16,
    pub damage_sound: i16,
    pub dead_unit_id: i16,
    pub blood_unit_id: i16,
    pub sort_number: u8,
    pub can_be_built_on: u8,
    pub icon_id: i16,
    pub hide_in_editor: u8,
    pub old_portrait_pict: i16,
    pub enabled: u8,
    pub disabled: u8,
    pub placement_side_terrain: (i16, i16),
    pub placement_terrain: (i16, i16),
    pub clearance_size: (f32, f32),
    pub hill_mode: u8,
    pub fog_visibility: u8,
    pub terrain_restriction: i16,
    pub fly_mode: u8,
    pub resource_capacity: i16,
    pub resource_decay: f32,
    pub blast_defense_level: u8,
    pub combat_level: u8,
    pub interaction_mode: u8,
    pub minimap_mode: u8,
    pub interface_kind: u8,
    pub multiple_attribute_mode: f32,
    pub minimap_color: u8,
    pub language_dll_help: i32,
    pub language_dll_hot_key_text: i32,
    pub hot_key: i32,
    pub recyclable: u8,
    pub enable_auto_gather: u8,
    pub create_doppelganger_on_death: u8,
    pub resource_gather_group: u8,
    pub occlusion_mode: u8,
    pub obstruction_type: u8,
    pub obstruction_class: u8,
    pub r#trait: u8,
    pub civilization: u8,
    pub nothing: i16,
    pub selection_effect: u8,
    pub editor_selection_colour: u8,
    pub outline_box: (f32, f32, f32),
    pub data: u32,
    pub data_2: u32,
    #[protocol(fixed_length(3))]
    pub resources_storage: Vec<ResourceStorage>,
    pub damage_graphic_size: u8,
    #[protocol(length_prefix(elements(damage_graphic_size)))]
    pub damage_graphics: Vec<DamageGraphic>,
    pub selection_sound: i16,
    pub dying_sound: i16,
    pub wwise_train_sound_id: u32,
    pub wwise_damage_sound_id: u32,
    pub wwise_selection_sound_id: u32,
    pub wwise_dying_sound_id: u32,
    pub old_attack_reaction: u8,
    pub convert_terrain: u8,
    pub name: DeString,
    pub copy_id: i16,
    pub base_id: i16,
    #[protocol(skip_if("unit_type < UnitType::Flag"))]
    pub speed: Option<f32>,
    #[protocol(skip_if("unit_type < UnitType::DeadFish"))]
    pub dead_fish: Option<DeadFish>,
    #[protocol(skip_if("unit_type < UnitType::Bird"))]
    pub bird: Option<Bird>,
    #[protocol(skip_if("unit_type < UnitType::Combatant"))]
    pub type_50: Option<Combatant>,
    #[protocol(skip_if("unit_type != UnitType::Projectile"))]
    pub projectile: Option<Projectile>,
    #[protocol(skip_if("unit_type < UnitType::Creatable"))]
    pub creatable: Option<Creatable>,
    #[protocol(skip_if("unit_type != UnitType::Building"))]
    pub building: Option<Building>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct ResourceStorage(i16, f32, u8);

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct DamageGraphic {
    pub graphic_id: i16,
    pub damage_percent: i16,
    pub apply_mode: u8,
}

#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
#[protocol(discriminant = "integer")]
#[repr(u8)]
pub enum UnitType {
    /// Basic units like rubble and flares.
    EyeCandy = 10,

    /// Trees, used to be 90?
    Trees = 15,

    /// With Speed but mostly flags.
    Flag = 20,

    /// Only one unit has this type. AOK, DOPL (id 243) same properties as UtFlag
    Dopl = 25,

    /// Dead and fish units. It seems to be unused in SWGB as units just explode
    /// and do not leave carcasses.
    DeadFish = 30,

    /// Only birds in aoe and ror are of this type.
    Bird = 40,

    /// Shared class inherited by combat objects.
    Combatant = 50,

    /// Projectiles
    Projectile = 60,

    /// Units that can be created or trained like Army, Villagers and Ships.
    Creatable = 70,

    /// Buildings
    Building = 80,

    /// Trees in aoe and ror are of this type
    AoeTrees = 90,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct DeadFish {
    pub walking_graphic: i16,
    pub running_graphic: i16,
    pub rotation_speed: f32,
    pub old_size_class: u8,
    pub tracking_unit: i16,
    pub tracking_unit_mode: u8,
    pub tracking_unit_density: f32,
    pub old_move_algorithm: u8,
    pub turn_radius: f32,
    pub turn_radius_speed: f32,
    pub max_yaw_per_second_moving: f32,
    pub stationary_yaw_revolution_time: f32,
    pub max_yaw_per_second_stationary: f32,
    pub min_collision_size_multiplier: f32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Bird {
    pub default_task_id: i16,
    pub search_radius: f32,
    pub work_rate: f32,
    pub drop_sites_size: (i16, i16, i16),
    pub task_swap_group: u8,
    pub attack_sound: i16,
    pub move_sound: i16,
    pub wwise_attack_sound_id: u32,
    pub wwise_move_sound_id: u32,
    pub run_pattern: u8,
    pub task_list_size_count: i16,
    #[protocol(length_prefix(elements(task_list_size_count)))]
    pub task_list: Vec<Task>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Combatant {
    pub base_armor: i16,
    pub attack_count_size: i16,
    #[protocol(length_prefix(elements(attack_count_size)))]
    pub attacks: Vec<AttackOrArmor>,
    pub armor_count_size: i16,
    #[protocol(length_prefix(elements(armor_count_size)))]
    pub armor: Vec<AttackOrArmor>,
    pub defense_terrain_bonus: i16,
    pub bonus_damage_resistance: f32,
    pub max_range: f32,
    pub blast_width: f32,
    pub reload_time: f32,
    pub projectile_unit_id: i16,
    pub accuracy_percent: i16,
    pub break_off_combat: u8,
    pub frame_delay: i16,
    #[protocol(fixed_length(3))]
    pub graphic_displacement: Vec<f32>,
    pub blast_attack_level: u8,
    pub min_range: f32,
    pub accuracy_dispersion: f32,
    pub attack_graphic: i16,
    pub displayed_melee_armour: i16,
    pub displayed_attack: i16,
    pub displayed_range: f32,
    pub displayed_reload_time: f32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Projectile {
    pub projectile_type: u8,
    pub smart_mode: u8,
    pub hit_mode: u8,
    pub vanish_mode: u8,
    pub area_effect_specials: u8,
    pub projectile_arc: f32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Creatable {
    #[protocol(fixed_length(3))]
    pub resources_costs: Vec<ResourceUsage>,
    pub train_time: i16,
    pub train_location_id: i16,
    pub button_id: u8,
    pub rear_attack_modifier: f32,
    pub flank_attack_modifier: f32,
    pub creatable_type: u8,
    pub hero_mode: u8,
    pub garrison_graphic: i32,
    pub spawning_graphic: i16,
    pub upgrade_graphic: i16,
    pub hero_glow_graphic: i16,
    pub max_charge: f32,
    pub recharge_rate: f32,
    pub charge_event: i16,
    pub charge_type: i16,
    pub total_projectiles: f32,
    pub max_total_projectiles: u8,
    pub projectile_spawning_area: (f32, f32, f32),
    pub secondary_projectile_unit: i32,
    pub special_graphic: i32,
    pub special_ability: u8,
    pub displayed_pierce_armour: i16,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Building {
    pub construction_graphic_id: i16,
    pub snow_graphic_id: i16,
    pub destruction_graphic_id: i16,
    pub destruction_rubble_graphic_id: i16,
    pub researching_graphic: i16,
    pub research_completed_graphic: i16,
    pub adjacent_mode: u8,
    pub graphics_angle: i16,
    pub disappears_when_built: u8,
    pub stack_unit_id: i16,
    pub foundation_terrain_id: i16,
    pub old_overlay_id: i16,
    pub tech_id: i16,
    pub can_burn: u8,
    #[protocol(fixed_length(4))]
    pub building_annexes: Vec<Annexe>,
    pub head_unit: i16,
    pub transform_unit: i16,
    pub transform_sound: i16,
    pub construction_sound: i16,
    pub wwise_transform_sound_id: u32,
    pub wwise_construction_sound_id: u32,
    pub garrison_type: u8,
    pub garrison_heal_rate: f32,
    pub garrison_repair_rate: f32,
    pub pile_unit: i16,
    #[protocol(fixed_length(6))]
    pub looting_table: Vec<u8>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Annexe {
    pub unit_id: i16,
    pub misplacement: (f32, f32),
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct AttackOrArmor {
    pub class: i16,
    pub amount: i16,
}
