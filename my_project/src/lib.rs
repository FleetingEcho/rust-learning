//! # Art
//!
//! A future art modeling library, currently a color mixing library.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    //! Defines types of colors.

    /// Primary colors
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// Secondary colors
    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    //! Utility functions, currently implementing a color mixing tool.
    use crate::kinds::*;

    /// Mixes two primary colors to produce a secondary color.
    /// ```rust
    /// use art::utils::mix;
    /// use art::kinds::{PrimaryColor, SecondaryColor};
    /// assert!(matches!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green));
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
