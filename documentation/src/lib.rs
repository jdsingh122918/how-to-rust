//! # Art
//!
//! A library for modeling artistic concepts

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary color according to the RYB color model
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color
    #[allow(dead_code)]
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        println!("Colors input: {:?}, {:?}", c1, c2);
        SecondaryColor::Orange
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
