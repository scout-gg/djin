use crate::dat::common::DeString;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainBlock {
    pub virtual_function_pointer: u32,
    pub map_pointer: u32,
    pub map_width: i32,
    pub map_height: i32,
    pub world_width: i32,
    pub world_height: i32,

    #[protocol(fixed_length(19))]
    pub tile_sizes: Vec<TileSize>,

    pub padding_ts: i16,
    pub padding_ts_2: i16,

    #[protocol(fixed_length(200))]
    pub terrains: Vec<Terrain>,
    pub map_min_x: f32,
    pub map_min_y: f32,
    pub map_max_x: f32,
    pub map_max_y: f32,
    pub map_max_xplus_1: f32,
    pub map_max_y_plus_1: f32,
    pub map_max_y_plus_1_: i16,
    pub removed_block_sused: i16,
    pub borders_used: i16,
    pub max_terrain: i16,
    pub tile_width: i16,
    pub tile_height: i16,
    pub tile_half_height: i16,
    pub tile_half_width: i16,
    pub elevation_height: i16,
    pub current_row: i16,
    pub current_col: i16,
    pub block_begin_row: i16,
    pub block_end_row: i16,
    pub block_begin_col: i16,
    pub block_end_col: i16,
    pub search_map_ptr: u32,
    pub search_map_rows_ptr: u32,
    pub any_frame_change: u8,
    pub map_visible_flag: u8,
    pub fog_flag: u8,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainBorder {
    pub draw_terrain: i16,
    // always 0
    pub underlay_terrain: i16,
    pub border_style: i16,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TileSize {
    pub width: i16,
    pub height: i16,
    pub delta_y: i16,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Terrain {
    pub enabled: u8,
    pub random: u8,
    pub is_water: u8,
    pub hide_in_editor: u8,
    pub string_id: i16,
    pub name: DeString,
    pub sp_name: DeString,
    pub slp: i32,
    pub shape_pointer: i32,
    pub sound_id: i32,
    pub wwise_sound_id: u32,
    pub wwise_sound_stop_id: u32,
    pub blend_priority: i32,
    pub blend_type: i32,
    pub overlay: DeString,
    #[protocol(fixed_length(3))]
    pub colors: Vec<u8>,
    pub cliff_colors: (u8, u8),
    pub passable_terrain: u8,
    pub im_passable_terrain: u8,
    pub is_animated: u8,
    pub animation_frames: i16,
    pub pause_frames: i16,
    pub interval: f32,
    pub pause_between_loops: f32,
    pub frame: i16,
    pub draw_frame: i16,
    pub animate_last: f32,
    pub frame_changed: u8,
    pub drawn: u8,
    #[protocol(fixed_length(19))]
    pub elevation_graphic: Vec<FrameData>,
    pub terrain_to_draw: i16,
    pub terrain_dimensions: (i16, i16),

    #[protocol(fixed_length(30))]
    pub terrain_unit_masked_density: Vec<i16>,
    #[protocol(fixed_length(30))]
    pub terrain_unit_tid: Vec<i16>,
    #[protocol(fixed_length(30))]
    pub terrain_unit_density: Vec<i16>,
    #[protocol(fixed_length(30))]
    pub terrain_unit_centering: Vec<u8>,
    pub number_of_terrain_units_used: i16,
    pub phantom: i16,
    pub phantom2: i16,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct FrameData {
    framecount: i16,
    anglecount: i16,
    shapeid: i16,
}
