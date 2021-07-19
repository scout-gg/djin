mod zlib;

use std::path::Path;
use eyre::Result;
use flate2::read::DeflateDecoder;
use std::io::{Read};
use crate::frame::{read_8_char_sized_string};
use bytes::Buf;
use std::borrow::BorrowMut;
use protocol::logic::Aligned;
use protocol::Parcel;
use crate::dat::color::ColorTable;
use crate::dat::terrain::{TerrainHeader, TerrainRestriction};
use crate::dat::sound::SoundTable;

mod color;
mod sound;
mod terrain;

pub struct DatFile {
    game_version: GameVersion,
    terrain_header: TerrainHeader,
    terrain_restrictions: TerrainRestriction,
    color_table: ColorTable,
    sound_table: SoundTable,
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
            ..Default::default()
        };

        let game_version = GameVersion::read(&mut buf, &settings).expect("Read error");
        let terrain_header = TerrainHeader::read(&mut buf, &settings).expect("Read error");
        let terrain_restrictions = TerrainRestriction::read(&mut buf, terrain_header.num_terrain_tables as usize, &settings);
        let color_table = ColorTable::read(&mut buf, &settings).expect("Read error");
        let sound_table = SoundTable::read(&mut buf, &settings).expect("Read error");


        Ok(DatFile {
            game_version,
            terrain_header,
            terrain_restrictions,
            color_table,
            sound_table
        })
    }
}

#[cfg(test)]
mod test {
    use crate::dat::DatFile;
    use eyre::Result;
    use protocol::Parcel;
    use spectral::prelude::*;

    type TestResult = Result<()>;

    #[test]
    fn should_read_dat_file() -> TestResult {
        let dat_file = DatFile::from_file("tests/game_assets/empires2_x2_p1.dat").unwrap();
        assert_that(&dat_file.game_version.game_version).is_equal_to("VER 7.4\0".to_string());

        assert_that(&dat_file.sound_table.sounds).has_length(dat_file.sound_table.sound_size as usize);
        assert_that(&dat_file.color_table.colors).has_length(dat_file.color_table.color_table_size as usize);

        assert_that(&dat_file.terrain_header.num_terrains).is_equal_to(110);
        assert_that(&dat_file.terrain_header.num_terrain_tables).is_equal_to(31);
        assert_that(&dat_file.terrain_restrictions.passability).has_length(31);

        Ok(())
    }
}