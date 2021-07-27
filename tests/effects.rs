use aoe_djin::dat::DatFile;
use aoe_djin::dat::effect::{Effect, EffectType};
use aoe_djin::dat::civilization::Unit;
use std::collections::HashMap;
use std::io::Cursor;
use bytes::Buf;

#[test]
fn effects() {
    let dat_file = DatFile::from_file("tests/game_assets/empires2_x2_p1.dat").unwrap();
    let mut effect_tech_map = HashMap::new();

    dat_file.tech_table.techs
        .iter()
        .enumerate()
        .for_each(|(id, tech)| {
            effect_tech_map.insert(tech.effect_id, (id, &tech.name.content));
        });

    dat_file.effect_table.effects.iter()
        .map(|effect| effect.commands.iter()
            .flat_map(|e| {
                if let EffectType::Modifier { amount, unit_id, modifier_kind, unused} = e {
                    Some(e)
                } else {
                    None
                }
            })
            .collect()
        )
        .enumerate()
        .for_each(|(effect_id, effect): (usize, Vec<&EffectType>)| {
            let header = effect_tech_map.get(&(effect_id as i16));
            println!(" TECH : {:?} : Effects : ", header);
            for ef in effect {
                if let EffectType::Modifier { amount, unit_id, modifier_kind, unused} = ef {
                    println!("\tunit_id : {:?}, modifier_kind : {:?}, unused : {:?} - amount: {:?}", unit_id, modifier_kind, unused, amount);
                }
            }
        });
}