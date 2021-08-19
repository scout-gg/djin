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

// TODO : Figure out unknown effect type
#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
#[protocol(discriminant = "integer")]
#[repr(u8)]
pub enum EffectType {
    #[protocol(discriminator(0))]
    Modifier {
        target_unit_id: i16,
        target_unit_class: i16,
        modifier_kind: ModifierClass,
    },
    #[protocol(discriminator(1))]
    Bonus {
        // Todo:  Seems to be some kind of target id but it doesn't match anything we know, maybe a sound or a sprite ?
        // Mill upgrate all target : 36  with the correct amount of food
        // Guild: 78 (trading fee) correct amount
        // Banking 46: (free tributes) amount 0.0 (correct)
        // Faith 77: 3.0
        // Faith 178: 2.0
        // Faith 179: 4.0
        // Heresy 192: 1.0
        // C-Bonus, +50f +50w :
        // 91 50.0
        // 92 - 50.0
        // Illumination : 35: 3.0
        //Conversion enabler : 67: 1.0
        // C-Bonus, Conversion Resistance (Teutons)
        // 77: 2.0
        // C-Bonus, Start w/ 6 villagers (Chinese)
        // 234: 1.0
        // C-Bonus, Techs -15% cost (Chinese in castle age ?)
        // 85, 1.0
        // Redemption:
        // 28 1.0
        // Convert Ships
        // 87: 1.0
        // Atonement :
        // 27 : 1.0
        // C-Bonus, Monk Heal Range (Teutons) :
        // 90 8.0
        // C-Bonus, Tec Cost
        // 1- 85 0.9
        // 2- 85 0.85
        // 3- 85 0.8
        // C-Bonus, Market (Saraceans market)
        // 78 0.05
        // C-Bonus, Dominant LOS
        // 97 1.0
        // C-Bonus, +10 pop
        // 32 10.0
        // Monk Ranged Heal
        // 90 4.0
        // Spy Technology:
        // 183
        unknown: i16,
        // Todo: find what this actually enable disable
        enabled: i16,
        // Always -1
        unused: i16,
        amount: f32,
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
        unused_bytes: Vec<u8>,
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
    },
}


// TODO : Figure out unknown modifiers
#[derive(Protocol, Debug, Clone, PartialEq, PartialOrd)]
#[protocol(discriminant = "integer")]
#[repr(i16)]
pub enum ModifierClass {
    #[protocol(discriminator(0))]
    HitPoint(f32),

    #[protocol(discriminator(1))]
    LineOfSight(f32),

    #[protocol(discriminator(2))]
    TransportCapacity(f32),

    #[protocol(discriminator(5))]
    Unknown5(f32),

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

    #[protocol(discriminator(19))]
    Unknown19(f32),

    #[protocol(discriminator(20))]
    Unknown20(f32),

    #[protocol(discriminator(21))]
    Pop(f32),

    #[protocol(discriminator(23))]
    Range(f32),

    // Seems to be damage resistance versus (unit class)
    // and not damage resistance for (unit id) when the unit class
    // appear (in place of "unused")
    #[protocol(discriminator(24))]
    DamageResistance(f32),

    // What is the amount ?
    #[protocol(discriminator(43))]
    MoveInFullTechTree(f32),

    #[protocol(discriminator(50))]
    RenameUnit {
        language_dll_help: u32,
    },

    #[protocol(discriminator(57))]
    Unknown57(f32),

    #[protocol(discriminator(103))]
    Unknown103(f32),

    // Seems related to civ UT (is that unique tech) ?
    #[protocol(discriminator(105))]
    Unknown105(f32),
}
