# Djin

⚠️ **Work in progress**

Djin is a *work in progress* replacement for [genie-rs](https://github.com/SiegeEngineers/genie-rs).

It currently support Age of Empire II Definitive Edition only but we plan to support other version of the game
when the main features will be stabilised.

The main difference with genie-rs is the fact that we use a [fork of the protocol crate](https://github.com/oknozor/protocol)
handle game files  serialization and deserialization. This allow us to write almost zero parsing logic.


Age of Empires II © Microsoft Corporation. djin was created under Microsoft's "Game Content Usage Rules" using 
assets from Age of Empires II, and it is not endorsed by or affiliated with Microsoft.

## Example 

You can run this example with `cargo run --example datfile` :

```rust
fn main() {
    let datfile = DatFile::from_file("tests/game_assets/empires2_x2_p1.dat").expect("Error reading dat file");
    
    datfile.civilizations.civilizations.iter()
        .for_each(|civ| println!("{}", civ.name))
}
```


