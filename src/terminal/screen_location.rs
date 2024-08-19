use std::ops::{Add, Sub};

/// Struct representing a location on the screen
#[derive(Clone, Debug)]
pub struct ScreenLocation {
    pub row: usize,
    pub col: usize,
}

impl Sub for ScreenLocation{
    type Output = ScreenLocation;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            row: self.row.saturating_sub(rhs.row),
            col: self.col.saturating_sub(rhs.col),
        }
    }
}

impl Add for ScreenLocation {
    type Output = ScreenLocation;

    fn add(self, rhs:Self)-> Self::Output {
        Self{
            row: self.row.saturating_add(rhs.row),
            col: self.col.saturating_add(rhs.col),
        }
    }
}

impl ScreenLocation {
    pub fn default()->Self {
        Self {row:0, col:0}
    }
}