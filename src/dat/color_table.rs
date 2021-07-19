/// Player colour data.
#[derive(Protocol, Debug, Clone, PartialEq, Eq)]
pub struct ColorTable {
    pub id: i32,
    /// Base palette index for this player colour.
    pub base: u8,
    /// The palette index to use for unit outlines when they are obscured by buildings or trees.
    pub unit_outline_color: u8,
    pub unit_selection_colors: (u8, u8),
    /// Palette indices for this colour on the minimap.
    pub minimap_colors: (u8, u8, u8),
    /// Color table to use for this player colour in the in-game statistics in the bottom right.
    pub statistics_text_color: i32,
}