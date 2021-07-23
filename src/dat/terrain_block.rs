use protocol::logic::Aligned;
use crate::dat::common::DeString;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainBlock {
    virtual_function_pointer: u32,
    map_pointer: u32,
    map_width: i32,
    map_height: i32,
    world_width: i32,
    world_height: i32,

    #[protocol(fixed_length(19))]
    tile_sizes: Vec<TileSize>,

    pub padding_ts: i16,
    pub padding_ts_2: i16,

    #[protocol(fixed_length(200))]
    pub terrains: Vec<Terrain>,

    map_min_x: f32,
    map_min_y: f32,
    map_max_x: f32,
    map_max_y: f32,
    map_max_xplus_1: f32,
    map_max_y_plus_1: f32,
    map_max_y_plus_1_: i16,
    removed_block_sused: i16,
    borders_used: i16,
    max_terrain: i16,
    tile_width: i16,
    tile_height: i16,
    tile_half_height: i16,
    tile_half_width: i16,
    elevation_height: i16,
    current_row: i16,
    current_col: i16,
    block_begin_row: i16,
    block_end_row: i16,
    block_begin_col: i16,
    block_end_col: i16,
    search_map_ptr: u32,
    search_map_rows_ptr: u32,
    any_frame_change: u8,
    map_visible_flag: u8,
    fog_flag: u8,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainBorder {
    draw_terrain: i16,
    // always 0
    underlay_terrain: i16,
    border_style: i16,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TileSize {
    width: i16,
    height: i16,
    delta_y: i16,
}

/*
WwiseSoundID : 0 //
WwiseSoundStopID : 0 //

OverlayMaskName :
Drawn :
FRAME DATA SIZE : 19
TerrainToDraw : -1
TerrainDimensions : (i16, i16)
TerrainUnitMaskedDensity
TerrainUnitID
TerrainUnitDensity
TerrainUnitCentering
NumberOfTerrainUnitsUsed : 0
Phantom : 0
 */
#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Terrain {
    pub enabled: u8,
    pub random: u8,
    is_water: u8,
    hide_in_editor: u8,
    string_id: i16,
    name: DeString,
    sp_name: DeString,

    slp: i32,
    shape_pointer: i32,
    sound_id: i32,
    wwise_sound_id: u32,
    wwise_sound_stop_id: u32,

    blend_priority: i32,
    blend_type: i32,
    overlay: DeString,

    #[protocol(fixed_length(3))]
    colors: Vec<u8>,
    cliff_colors: (u8, u8),
    passable_terrain: u8,
    im_passable_terrain: u8,
    is_animated: u8,
    animation_frames: i16,
    pause_frames: i16,
    interval: f32,
    pause_between_loops: f32,
    frame: i16,
    draw_frame: i16,
    animate_last: f32,
    frame_changed: u8,
    drawn: u8,
    #[protocol(fixed_length(19))]
    elevation_graphic: Vec<FrameData>,
    terrain_to_draw: i16,
    terrain_dimensions: (i16, i16),

    #[protocol(fixed_length(30))]
    terrain_unit_masked_density: Vec<i16>,
    #[protocol(fixed_length(30))]
    terrain_unit_tid: Vec<i16>,
    #[protocol(fixed_length(30))]
    terrain_unit_density: Vec<i16>,
    #[protocol(fixed_length(30))]
    terrain_unit_centering: Vec<u8>,
    number_of_terrain_units_used: i16,
    phantom: i16,
    phantom2: i16,

}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct FrameData {
    framecount: i16,
    anglecount: i16,
    shapeid: i16,
}