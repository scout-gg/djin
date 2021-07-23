mod zlib;

use crate::dat::color::ColorTable;
use crate::dat::sound::{SoundTable};
use crate::dat::terrain::{TerrainHeader, TerrainRestrictions};
use eyre::Result;
use protocol::Parcel;
use std::path::Path;
use crate::dat::sprite::{SpriteTable};
use crate::dat::terrain_block::{TerrainBlock, Terrain};
use crate::dat::random_map::RandomMap;
use crate::dat::effect::Effects;
use crate::dat::unit::{Units, Task, UnitHeaders};
use bytes::Buf;
use crate::dat::civilization::Civilizations;

mod civilization;
mod color;
mod sound;
mod sprite;
mod terrain;
mod terrain_block;
mod random_map;
mod effect;
mod unit;
mod common;

pub struct DatFile {
    game_version: GameVersion,
    terrain_header: TerrainHeader,
    terrain_restrictions: TerrainRestrictions,
    color_table: ColorTable,
    sound_table: SoundTable,
    sprite_table: SpriteTable,
    terrain_block: TerrainBlock,
    random_map: RandomMap,
    effects: Effects,
    units: Units,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct GameVersion {
    #[protocol(fixed_length(8))]
    pub game_version: String,
}

impl DatFile {
    fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Result<DatFile> {
        let mut buf = zlib::decompress(path)?;

        let settings = protocol::Settings {
            byte_order: protocol::ByteOrder::LittleEndian,
        };

        let game_version = GameVersion::read(&mut buf, &settings).expect("Read error");
        let terrain_header = TerrainHeader::read(&mut buf, &settings).expect("Read error");
        let terrain_restrictions = TerrainRestrictions::read(
            &mut buf,
            terrain_header.terrain_restriction_size as usize,
            terrain_header.restriction_size as usize,
            &settings,
        );
        let color_table = ColorTable::read(&mut buf, &settings).expect("Read error");
        let sound_table = SoundTable::read(&mut buf, &settings).expect("Read error");
        let sprite_table = SpriteTable::read(&mut buf, &settings).expect("Read error");
        let terrain_block = TerrainBlock::read(&mut buf, &settings).expect("Read error");
        let random_map = RandomMap::read(&mut buf, &settings).expect("Read error");
        let effects = Effects::read(&mut buf, &settings).expect("Read error");
        let units = Units::read(&mut buf, &settings).expect("Read error");
        // let civs = Civilizations::read(&mut buf, &settings).expect("Read error");

        // println!("{:?}", civs);
        Ok(DatFile {
            game_version,
            terrain_header,
            terrain_restrictions,
            color_table,
            sound_table,
            sprite_table,
            terrain_block,
            random_map,
            effects,
            units,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::dat::DatFile;
    use eyre::Result;
    use spectral::prelude::*;

    type TestResult = Result<()>;


    #[test]
    fn should_read_dat_file() -> TestResult {
        let dat_file = DatFile::from_file("tests/game_assets/empires2_x2_p1.dat").unwrap();

        // Version
        assert_that(&dat_file.game_version.game_version).is_equal_to("VER 7.4\0".to_string());

        // Terrain Header
        assert_that(&dat_file.terrain_header.terrain_restriction_size).is_equal_to(31);
        assert_that(&dat_file.terrain_header.restriction_size).is_equal_to(110);
        assert_that(&dat_file.terrain_header.terrain_restriction_size).is_equal_to(31);
        assert_that(&dat_file.terrain_header.terrain_tables_pointer).has_length(31);
        assert_that(&dat_file.terrain_header.terrains_pointer).has_length(31);

        // Terrain restrictions
        assert_that(&dat_file.terrain_restrictions.inner).has_length(31);

        dat_file.terrain_restrictions.inner.iter().for_each(|el| {
            assert_that(&el.pass_graphics).has_length(110);
            assert_that(&el.passability).has_length(110);
        });

        // Colors
        assert_that(&dat_file.color_table.colors).has_length(16);
        assert_that(&dat_file.sound_table.sounds).has_length(685);

        Ok(())
    }
}
