use bytes::Buf;
use protocol::{Settings, Parcel};
use std::io::Read;
use crate::dat::common::DeString;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SpriteEnabled {
    pub size: u16,
    #[protocol(length_prefix(elements(size)))]
    pub sprite_enabled: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteTable {
    pub sprites: Vec<Sprite>,
}

impl SpriteTable {
    pub fn read(buf: &mut (impl Read + Buf), len: usize, settings: &Settings) -> Self {
        let mut sprites = Vec::with_capacity(len);


        for _ in 0..len {
            let sprite = Sprite::read(buf, settings).expect("Read error");
            println!("{:?}", sprite);
            sprites.push(sprite);
        };

        SpriteTable {
            sprites
        }
    }
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Sprite {
    pub name: DeString,
    pub filename: DeString,
    pub unknown: DeString,
    pub slp_id: u32,
    pub is_loaded: bool,
    force_player_color: u8,
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
    graphic_id: i16,
    padding_1: i16,
    sprite_ptr: i32,
    offset_x: i16,
    offset_y: i16,
    display_angle: i16,
    padding_2: i16,
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
