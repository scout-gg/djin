use bytes::Buf;
use protocol::{Parcel, Settings};
use std::io::Read;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainHeader {
    pub terrain_restriction_size: u16,
    pub restriction_size: u16,

    #[protocol(length_prefix(elements(terrain_restriction_size)))]
    pub terrain_tables_pointer: Vec<u32>,

    // Fixme there is a bug in protocol, making this have length 1 when reusing "length_prefix" twice on the same field
    // #[protocol(length_prefix(elements(terrain_restriction_size)))]
    #[protocol(fixed_length(31))]
    pub terrains_pointer: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerrainRestrictions {
    pub(crate) inner: Vec<TerrainRestriction>,
}

impl TerrainRestrictions {
    pub fn read(
        buf: &mut (impl Read + Buf),
        terrain_restriction_size: usize,
        restriction_size: usize,
        settings: &Settings,
    ) -> Self {
        debug_assert_eq!(terrain_restriction_size, 31);
        debug_assert_eq!(restriction_size, 110);
        let mut restrictions = Vec::with_capacity(terrain_restriction_size);
        for _ in 0..terrain_restriction_size {
            restrictions.push(TerrainRestriction::read(buf, restriction_size, settings));
        }

        debug_assert_eq!(restrictions.len(), 31);

        TerrainRestrictions {
            inner: restrictions,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TerrainRestriction {
    pub passability: Vec<f32>,
    pub pass_graphics: Vec<TerrainPassGraphic>,
}

impl TerrainRestriction {
    pub fn read(buf: &mut (impl Read + Buf), len: usize, settings: &Settings) -> Self {
        debug_assert_eq!(len, 110);
        let mut passability = Vec::with_capacity(len);
        for _ in 0..len {
            passability.push(buf.get_f32_le());
        }
        debug_assert_eq!(passability.len(), 110);

        let mut pass_graphics = Vec::with_capacity(len);
        for _ in 0..len {
            let pass_graphic =
                TerrainPassGraphic::read(buf, settings).expect("TerrainPassGraphic parsing error");
            pass_graphics.push(pass_graphic);
        }

        debug_assert_eq!(passability.len(), 110);

        TerrainRestriction {
            passability,
            pass_graphics,
        }
    }
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainPassGraphic {
    pub exit_tile_sprite: u32,
    pub enter_tile_sprite: u32,
    pub walk_tile_sprite: u32,
    pub replication_amount: i32,
}
