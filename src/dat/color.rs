#[derive(Protocol, Debug, Clone, PartialEq, Eq)]
pub struct ColorTable {
    pub size: u16,
    #[protocol(length_prefix(elements(size)))]
    pub colors: Vec<Color>,
}

/// Player colour data.
#[derive(Protocol, Debug, Clone, PartialEq, Eq)]
pub struct Color {
    pub id: i32,
    /// Base palette index for this player colour.
    pub base: i32,
    /// The palette index to use for unit outlines when they are obscured by buildings or trees.
    pub unit_outline_color: i32,
    pub unit_selection_colors: (i32, i32),
    /// Palette indices for this colour on the minimap.
    pub minimap_colors: (i32, i32, i32),
    /// Color table to use for this player colour in the in-game statistics in the bottom right.
    pub statistics_text_color: i32,
}
