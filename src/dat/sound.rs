use protocol::Parcel;
use std::io::Read;
use bytes::Buf;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SoundTable {
    pub sound_size: u16,
    #[protocol(length_prefix(elements(sound_size)))]
    pub sounds: Vec<Sound>

}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Sound {
    /// Unique ID for this sound.
    pub id: u16,
    pub play_delay: i16,
    pub sound_item_size: u16,
    pub cache_time: i32,
    pub total_probability: u16,
    #[protocol(length_prefix(elements(sound_item_size)))]
    pub items: Vec<SoundItem>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SoundItem {
    #[protocol(fixed_length(13))]
    pub filename: String,
    pub resource_id: i32,
    pub probability: i16,
    pub civilization: i16,
    pub icon_set: i16,
}