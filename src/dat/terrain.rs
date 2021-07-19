use protocol::{Parcel, Settings, Error};
use std::io::{Read, Write};
use protocol::hint::Hints;
use bytes::Buf;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainHeader {
    pub num_terrain_tables: u16,
    pub num_terrains: u16,
    #[protocol(length_prefix(elements(num_terrain_tables)))]
    pub num_terrain_tables_pointer: Vec<TerrainPointer>,
    #[protocol(length_prefix(elements(num_terrains)))]
    pub num_terrains_pointer: Vec<TerrainPointer>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainPointer {
    p1: u64,
    p2: u64,
    p3: u64,
    p4: u64,
}

pub struct TerrainRestriction {
    pub passability: Vec<f32>,
    pub pass_graphics: Vec<TerrainPassGraphic>,
}

impl TerrainRestriction {
    pub fn read(buf: &mut (impl Read + Buf), len: usize, settings: &Settings) -> Self {
        let mut passability = Vec::with_capacity(len);
        for _ in 0..len {
            passability.push(buf.get_f32_le());
        }

        let mut pass_graphics = Vec::with_capacity(len);
        for _ in 0..len {
            let pass_graphic = TerrainPassGraphic::read(buf, settings).expect("TerrainPassGraphic parsing error");
            pass_graphics.push(pass_graphic);
        }

        TerrainRestriction {
            passability,
            pass_graphics,
        }
    }
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainPassGraphic {
    pub exit_tile_sprite: i16,
    pub enter_tile_sprite: i16,
    pub walk_tile_sprite: i16,
    pub replication_amount: i32,
}
