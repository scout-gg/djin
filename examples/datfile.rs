use aoe_djin::dat::DatFile;

fn main() {
    let datfile = DatFile::from_file("tests/game_assets/empires2_x2_p1.dat").expect("Error reading dat file");

    datfile.civilizations.civilizations.iter()
        .for_each(|civ| println!("{}", civ.name))
}