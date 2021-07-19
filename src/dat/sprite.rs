use bytes::Buf;
use protocol::Settings;
use std::io::Read;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SpriteEnabled {
    pub size: u16,
    #[protocol(length_prefix(elements(size)))]
    pub sprite_enabled: Vec<u32>,
}

pub struct SpriteTable {
    sprites: Vec<Sprite>,
}

impl SpriteTable {
    pub fn read(buf: &mut (impl Read + Buf), len: usize, settings: &Settings) -> Self {
        todo!();
    }
}

pub struct Sprite {
    pub id: u16,
    pub name: String,
    pub filename: String,
    pub slp_id: u32,
    pub is_loaded: bool,
    force_player_color: Option<u8>,
    pub layer: u8,
    pub color_table: u16,
    pub transparent_selection: bool,
    pub bounding_box: (i16, i16, i16, i16),
    pub sound_id: i16,
    pub num_frames: u16,
    pub num_angles: u16,
    pub base_speed: f32,
    pub frame_rate: f32,
    pub replay_delay: f32,
    pub sequence_type: u8,
    pub mirror_flag: i8,
    other_flag: i8,
    pub deltas: Vec<SpriteDelta>,
    pub attack_sounds: Vec<SpriteAttackSound>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SpriteDelta {
    pub sprite_id: u16,
    pub offset_x: i16,
    pub offset_y: i16,
    pub display_angle: i16,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SpriteAttackSound {
    #[protocol(fixed_length(3))]
    pub sound_props: Vec<SoundProp>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SoundProp {
    pub sound_delay: i16,
    pub sound_id: u16,
    wwise_sound_id: i32,
}
