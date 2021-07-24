use crate::dat::common::DeString;
use protocol::{Parcel, Settings, Error};
use std::io::{Read, Write};
use protocol::hint::Hints;
use crate::dat::unit::Task;

type ResourceUsage = (i16, i16, i16);

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Civilizations {
    size: u16,
    #[protocol(length_prefix(elements(size)))]
    pub civilizations: Vec<Civilization>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Civilization {
    player_type: u8,
    name: DeString,
    resource_size: u16,
    tech_tree_id: i16,
    team_bonus_id: u16,
    #[protocol(length_prefix(elements(resource_size)))]
    resources: Vec<f32>,
    icon_set: u8,
    units_pointers_size: u16,
    #[protocol(length_prefix(elements(units_pointers_size)))]
    unit_pointers: Vec<u32>,
    #[protocol(length_prefix(pointers(unit_pointers)))]
    pub(crate) units: Vec<Unit>,
}


#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Unit {
    unit_type: u8,
    id: i16,
    language_dll_name: i32,
    language_dll_creation: i32,
    class: i16,
    standing_graphics: (i16, i16),
    dying_graphic: i16,
    undead_graphic: i16,
    undead_mode: u8,
    hit_points: i16,
    line_of_sight: f32,
    garrison_capacity: u8,
    collision_box: (f32, f32, f32),
    train_sound: i16,
    damage_sound: i16,
    dead_unit_id: i16,
    blood_unit_id: i16,
    sort_number: u8,
    can_be_built_on: u8,
    icon_id: i16,
    hide_in_editor: u8,
    old_portrait_pict: i16,
    enabled: u8,
    disabled: u8,
    placement_side_terrain: (i16, i16),
    placement_terrain: (i16, i16),
    clearance_size: (f32, f32),
    hill_mode: u8,
    fog_visibility: u8,
    terrain_restriction: i16,
    fly_mode: u8,
    resource_capacity: i16,
    resource_decay: f32,
    blast_defense_level: u8,
    combat_level: u8,
    interaction_mode: u8,
    minimap_mode: u8,
    interface_kind: u8,
    multiple_attribute_mode: f32,
    minimap_color: u8,
    language_dll_help: i32,
    language_dll_hot_key_text: i32,
    hot_key: i32,
    recyclable: u8,
    enable_auto_gather: u8,
    create_doppelganger_on_death: u8,
    resource_gather_group: u8,
    occlusion_mode: u8,
    obstruction_type: u8,
    obstruction_class: u8,
    r#trait: u8,
    civilization: u8,
    nothing: i16,
    selection_effect: u8,
    editor_selection_colour: u8,
    outline_box: (f32, f32, f32),
    data: u32,
    data_2: u32,
    #[protocol(fixed_length(3))]
    resources_storage: Vec<ResourceStorage>,
    damage_graphic_size: u8,
    #[protocol(length_prefix(elements(damage_graphic_size)))]
    damage_graphics: Vec<DamageGraphic>,
    selection_sound: i16,
    dying_sound: i16,
    wwise_train_sound_id: u32,
    wwise_damage_sound_id: u32,
    wwise_selection_sound_id: u32,
    wwise_dying_sound_id: u32,
    old_attack_reaction: u8,
    convert_terrain: u8,
    name: DeString,
    copy_id: i16,
    base_id: i16,
    #[protocol(skip_if("!(unit_type >= 20)"))]
    speed: Option<f32>,
    #[protocol(skip_if("!(unit_type >= 30)"))]
    dead_fish: Option<DeadFish>,
    #[protocol(skip_if("!(unit_type >= 40)"))]
    bird: Option<Bird>,
    #[protocol(skip_if("!(unit_type >= 50)"))]
    type_50: Option<Type50>,
    #[protocol(skip_if("!(unit_type == 60)"))]
    projectile: Option<Projectile>,
    #[protocol(skip_if("!(unit_type >= 70)"))]
    creatable: Option<Creatable>,
    #[protocol(skip_if("!(unit_type == 80)"))]
    building: Option<Building>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct ResourceStorage(i16, f32, u8);

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct DamageGraphic {
    graphic_id: i16,
    damage_percent: i16,
    apply_mode: u8,
}


#[derive(Protocol, Debug, Clone, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)]
pub enum UniType {
    /// Basic units like rubble and flares.
    Eyecandy = 10,

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
    walking_graphic: i16,
    running_graphic: i16,
    rotation_speed: f32,
    old_size_class: u8,
    tracking_unit: i16,
    tracking_unit_mode: u8,
    tracking_unit_density: f32,
    old_move_algorithm: u8,
    turn_radius: f32,
    turn_radius_speed: f32,
    max_yaw_per_second_moving: f32,
    stationary_yaw_revolution_time: f32,
    max_yaw_per_second_stationary: f32,
    min_collision_size_multiplier: f32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Bird {
    default_task_id: i16,
    search_radius: f32,
    work_rate: f32,
    drop_sites_size: (i16, i16, i16),
    task_swap_group: u8,
    attack_sound: i16,
    move_sound: i16,
    wwise_attack_sound_id: u32,
    wwise_move_sound_id: u32,
    run_pattern: u8,
    task_list_size_count: i16,
    #[protocol(length_prefix(elements(task_list_size_count)))]
    task_list: Vec<Task>,

}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Type50 {
    base_armor: i16,
    attack_count_size: i16,
    #[protocol(length_prefix(elements(attack_count_size)))]
    attacks: Vec<AttackOrArmor>,
    armor_count_size: i16,
    #[protocol(length_prefix(elements(armor_count_size)))]
    armor: Vec<AttackOrArmor>,
    defense_terrain_bonus: i16,
    bonus_damage_resistance: f32,
    max_range: f32,
    blast_width: f32,
    reload_time: f32,
    projectile_unit_id: i16,
    accuracy_percent: i16,
    break_off_combat: u8,
    frame_delay: i16,
    #[protocol(fixed_length(3))]
    graphic_displacement: Vec<f32>,
    blast_attack_level: u8,
    min_range: f32,
    accuracy_dispersion: f32,
    attack_graphic: i16,
    displayed_melee_armour: i16,
    displayed_attack: i16,
    displayed_range: f32,
    displayed_reload_time: f32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Projectile {
    projectile_type: u8,
    smart_mode: u8,
    hit_mode: u8,
    vanish_mode: u8,
    area_effect_specials: u8,
    projectile_arc: f32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Creatable {
    #[protocol(fixed_length(3))]
    resources_costs: Vec<ResourceUsage>,
    train_time: i16,
    train_location_id: i16,
    button_id: u8,
    rear_attack_modifier: f32,
    flank_attack_modifier: f32,
    creatable_type: u8,
    hero_mode: u8,
    garrison_graphic: i32,
    spawning_graphic: i16,
    upgrade_graphic: i16,
    hero_glow_graphic: i16,
    max_charge: f32,
    recharge_rate: f32,
    charge_event: i16,
    charge_type: i16,
    total_projectiles: f32,
    max_total_projectiles: u8,
    projectile_spawning_area: (f32, f32, f32),
    secondary_projectile_unit: i32,
    special_graphic: i32,
    special_ability: u8,
    displayed_pierce_armour: i16,

}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Building {
    construction_graphic_id: i16,
    snow_graphic_id: i16,
    destruction_graphic_id: i16,
    destruction_rubble_graphic_id: i16,
    researching_graphic: i16,
    research_completed_graphic: i16,
    adjacent_mode: u8,
    graphics_angle: i16,
    disappears_when_built: u8,
    stack_unit_id: i16,
    foundation_terrain_id: i16,
    old_overlay_id: i16,
    tech_id: i16,
    can_burn: u8,
    #[protocol(fixed_length(4))]
    building_annexes: Vec<Annexe>,
    head_unit: i16,
    transform_unit: i16,
    transform_sound: i16,
    construction_sound: i16,
    wwise_transform_sound_id: u32,
    wwise_construction_sound_id: u32,
    garrison_type: u8,
    garrison_heal_rate: f32,
    garrison_repair_rate: f32,
    pile_unit: i16,
    #[protocol(fixed_length(6))]
    looting_table: Vec<u8>,

}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Annexe {
    unit_id: i16,
    misplacement: (f32, f32),
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct AttackOrArmor {
    class: i16,
    amount: i16,
}