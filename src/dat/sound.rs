use crate::dat::common::DeString;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SoundTable {
    pub sound_table_size: u16,
    #[protocol(length_prefix(elements(sound_table_size)))]
    pub sounds: Vec<Sound>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Sound {
    pub id: u16,
    pub play_delay: i16,
    pub file_count: u16,
    pub cache_time: u32,
    pub total_probability: u16,
    #[protocol(length_prefix(elements(file_count)))]
    pub items: Vec<SoundItem>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct SoundItem {
    pub filename: DeString,
    pub resource_id: u32,
    pub probability: i16,
    pub civilization: i16,
    pub icon_set: i16,
}
