
/// Represents the location of the cursor within text
#[derive(Clone, Debug)]
pub struct TextPosition {
    pub row: usize,
    pub byte: usize,
    pub grapheme: usize,
}

impl TextPosition {
    pub fn default()-> Self {
        Self{row:0, byte:0, grapheme:0}
    }
}