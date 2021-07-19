mod zlib;

use std::path::Path;
use eyre::Result;
use flate2::read::DeflateDecoder;
use std::io::{Cursor, Read};
use crate::frame::{read_8_char_sized_string};
use bytes::Buf;
use std::borrow::BorrowMut;
use protocol::logic::Aligned;
use protocol::Parcel;
use crate::dat::color_table::ColorTable;

mod color_table;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct DatFile {
    #[protocol(fixed_length(8))]
    pub game_version: String,
    pub terrain_header: TerrainHeader,
    #[protocol(length_prefix(elements("terrain_header.num_terrain_tables")))]
    pub terrain_restriction: Vec<TerrainRestriction>,
    pub color_table_size: u16,
    #[protocol(length_prefix(elements(color_table_size)))]
    pub color_tables: Vec<ColorTable>,
}

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

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainRestriction {
    #[protocol(fixed_length(110))]
    pub passability: Vec<f32>,
    #[protocol(fixed_length(110))]
    pub pass_graphics: Vec<TerrainPassGraphic>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct TerrainPassGraphic {
    pub exit_tile_sprite: i16,
    pub enter_tile_sprite: i16,
    pub walk_tile_sprite: i16,
    pub replication_amount: i32,
}

impl DatFile {
    fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Result<DatFile> {
        let mut buf = zlib::decompress(path)?;

        let settings = protocol::Settings {
            byte_order: protocol::ByteOrder::LittleEndian,
            ..Default::default()
        };

        let dat_file = DatFile::read(&mut buf, &settings).expect("Read error");

        Ok(dat_file)
    }
}

#[cfg(test)]
mod test {
    use crate::dat::DatFile;
    use eyre::Result;
    use protocol::Parcel;

    type TestResult = Result<()>;

    #[test]
    fn should_read_dat_file() -> TestResult {
        let dat_file = DatFile::from_file("tests/game_assets/empires2_x2_p1.dat").unwrap();
        assert_eq!(dat_file.game_version, "VER 7.4\0");
        assert_eq!(dat_file.terrain_header.num_terrains, 110);
        assert_eq!(dat_file.terrain_header.num_terrain_tables, 31);
        assert!(!dat_file.terrain_restriction.is_empty());
        assert!(!dat_file.color_tables.is_empty());

        Ok(())
    }
}