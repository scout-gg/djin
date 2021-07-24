use crate::dat::common::DeString;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SpriteTable {
    pub size: u16,
    #[protocol(length_prefix(elements(size)))]
    pub sprite_enabled: Vec<u32>,
    #[protocol(length_prefix(pointers(sprite_enabled)))]
    pub sprites: Vec<Sprite>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Sprite {
    pub name: DeString,
    pub filename: DeString,
    pub particle_effect_name: DeString,
    pub slp_id: i32,
    pub is_loaded: bool,
    pub force_player_color: u8,
    pub layer: u8,
    pub color_table: u16,
    pub transparent_selection: bool,
    pub bounding_box: (i16, i16, i16, i16),
    pub num_deltas: u16,
    pub sound_id: i16,
    pub w_wise_sound_id: i32,
    pub angle_sound_used: u8,
    pub num_frames: u16,
    pub num_angles: u16,
    pub base_speed: f32,
    pub frame_rate: f32,
    pub replay_delay: f32,
    pub sequence_type: u8,
    pub id: i16,
    pub mirror_flag: i8,
    pub editor_flag: i8,
    #[protocol(length_prefix(elements(num_deltas)))]
    pub deltas: Vec<SpriteDelta>,
    #[protocol(skip_if("angle_sound_used == 0"))]
    #[protocol(length_prefix(elements(num_angles)))]
    pub attack_sounds: Option<Vec<AngleSound>>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SpriteDelta {
    graphic_id: i16,
    padding_1: i16,
    sprite_ptr: i32,
    offset_x: i16,
    offset_y: i16,
    display_angle: i16,
    padding_2: i16,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct AngleSound {
    pub sound_delay: i16,
    pub sound_id: u16,
    pub wwise_sound_id: u32,
    pub sound_delay_: i16,
    pub sound_id_: u16,
    pub wwise_sound_id_: u32,
    pub sound_delay__: i16,
    pub sound_id__: u16,
    pub wwise_sound_id__: u32,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SoundProp {
    pub sound_delay: i16,
    pub sound_id: u16,
    pub wwise_sound_id: i32,
}
