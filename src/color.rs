use std::fmt;

/// Defines a pixel's color as an RGB value.
pub type Color = crate::Vec3;

impl fmt::Display for Color {
    /// Write the translated \[0,255\] value of each Color.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.x()) as u8,
            (255.999 * self.y()) as u8,
            (255.999 * self.z()) as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_string_test() {
        let color = Color::new(0.126, 0.314, 0.631);
        let expected = "32 80 161".to_string();
        assert_eq!(format!("{}", color), expected);
    }
}
