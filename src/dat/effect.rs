use crate::dat::common::DeString;
use crate::dat::civilization::UnitClass;

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Effects {
    size: u32,
    #[protocol(length_prefix(elements(size)))]
    pub effects: Vec<Effect>,
}

#[derive(Protocol, Debug, Clone, PartialEq)]
pub struct Effect {
    pub name: DeString,
    pub command_size: i16,
    #[protocol(length_prefix(elements(command_size)))]
    pub commands: Vec<EffectType>,
}

// TODO : WORD IN PROGRESS
#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
#[protocol(discriminant = "integer")]
#[repr(u8)]
pub enum EffectType {
    #[protocol(discriminator(0))]
    Modifier {
        unit_id: i16,
        unused: i16,
        // 21 : Chinese Tc pop
        // 19 : Precsion in Balistic
        modifier_kind: i16,
        amount: f32
    },
    #[protocol(discriminator(1))]
    Bonus {
        // Todo:  Seems to be some kind of target id but it doesn't match anything we know, maybe a sound or a sprite ?
        unknown: i16,
        // Todo: find what this actually enable disable
        enabled: i16,
        // Always -1
        unused: i16,
        amount: f32
    },
    #[protocol(discriminator(2))]
    TechTreeEnable {
        unit_id: i16,
        // 1 or 0 migth want to align bool on u16
        enable: i16,
        unused: i32,
        unused2: i16,
    },

    #[protocol(discriminator(3))]
    UnitUpgrade {
        source_unit_id: i16,
        target_unit_id: i16,
        #[protocol(fixed_length(6))]
        unused_bytes: Vec<u8>
    },

    #[protocol(discriminator(4))]
    // TownWatch, Auto upgrade Scout Feudal Age, BLock printing
    // Los modifier : 1, 2,
    // 23 : Range ?
    CivModifier {
        unit_id: i16,
        // TODO :  There are some unknown classes here
        class: i16,
        modifier_type: i16,
        amount: f32,
    },

    #[protocol(discriminator(5))]
    UnitModifier {
        unit_id: i16,
        class: UnitClass,
        modifier_type: i16,
        amount: f32,
    },

    #[protocol(discriminator(6))]
    IncomeIncrease {
        unknown: i16,
        unknown2: i16,
        // This is a guess
        unused: i16,
        amount: f32,
    },

    #[protocol(discriminator(7))]
    SpawnUnit {
        unit_id: i16,
        building_id: i16,
        // This is a guess
        trigger_event: i16,
        amount: f32,
    },

    #[protocol(discriminator(10))]
    TriggerredTeamBonus {
        unit_id: i16,
        unused: i16,
        // find out
        trigger: i16,
        // 82 (Castle for Cumans Kipchak) 5 for slav military pop
        amount_or_building_id: f32,
    },


    #[protocol(discriminator(11))]
    GiveResource {
        resource_type: i16,
        // Check that
        enabled: i16,
        unused: i16,
        amount: f32,
    },

    #[protocol(discriminator(15))]
    IncreaseWorkingSpeed {
        building_id: i16,
        // Check that
        unused: i16,
        unknown: i16,
        amount: f32,
    },


    #[protocol(discriminator(26))]
    // Atheism only
    ReduceGoldGeneration {
        // 191 or 225
        unknown: i16,
        // Check that
        unused2: i16,
        unused: i16,
        amount: f32,
    },

    #[protocol(discriminator(101))]
    AutoResearch {
        // Check in game do we need to click the tech or is it just free
        tech_id: i16,
        unknown: i16,
        unused: i16,
        amount: f32,
    },

    #[protocol(discriminator(102))]
    Unknown {
        #[protocol(fixed_length(6))]
        unused_bytes: Vec<u8>,
        unknown: f32,
    },
    #[protocol(discriminator(103))]
    AutoResearch2 {
        // Compare with 101, maybe 102
        // Might be the research time modifier (see C-Bonus, Instant Loom)
        tech_id: i16,
        unknown: i16,
        unused: i16,
        amount: f32,
    },
    #[protocol(discriminator(255))]
    UnusedOrDeprecated {
        unused: i16,
        unused2: i16,
        unused3: i16,
        amount: f32,
    }
}


// TODO : WORD IN PROGRESS
pub enum ModifierClass {
    #[protocol(discriminator(0))]
    HitPoint(f32),

    #[protocol(discriminator(1))]
    LineOfSight(f32),

    #[protocol(discriminator(2))]
    TransportCapacity(f32),

    // 8 and 9 contains some large amount and does not seems to
    // match any known effects. Maybe it represent some sprite or sound properties or a language file id
    #[protocol(discriminator(8))]
    Unknown8 {
        language_dll_help: u32,
    },

    #[protocol(discriminator(9))]
    Unknown9 {
        language_dll_help: u32,
    },

    #[protocol(discriminator(11))]
    Accuracy(f32),

    #[protocol(discriminator(14))]
    CarryingCapacity(u32),

    #[protocol(discriminator(21))]
    Pop(f32),

    #[protocol(discriminator(23))]
    Range(f32),

    #[protocol(discriminator(24))]
    DamageResistance(f32),

    // What is the amount ?
    #[protocol(discriminator(43))]
    MoveInFullTechTree(f32),

    #[protocol(discriminator(50))]
    RenameUnit {
        language_dll_help: u32,
    }
}
