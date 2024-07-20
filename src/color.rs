use crate::vec3::Vec3;

pub type Color = Vec3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(1.0, 0.5, 0.25);
        assert_eq!(color.r(), 1.0);
        assert_eq!(color.g(), 0.5);
        assert_eq!(color.b(), 0.25);
    }

    #[test]
    fn test_add() {
        let color1 = Color::new(1.0, 0.5, 0.25);
        let color2 = Color::new(0.5, 0.25, 0.125);
        let result = color1 + color2;
        assert_eq!(result.r(), 1.5);
        assert_eq!(result.g(), 0.75);
        assert_eq!(result.b(), 0.375);
    }

    #[test]
    fn test_sub() {
        let color1 = Color::new(1.0, 0.5, 0.25);
        let color2 = Color::new(0.5, 0.25, 0.125);
        let result = color1 - color2;
        assert_eq!(result.r(), 0.5);
        assert_eq!(result.g(), 0.25);
        assert_eq!(result.b(), 0.125);
    }

    #[test]
    fn test_neg() {
        let color = Color::new(1.0, 0.5, 0.25);
        let result = -color;
        assert_eq!(result.r(), -1.0);
        assert_eq!(result.g(), -0.5);
        assert_eq!(result.b(), -0.25);
    }

    #[test]
    fn test_mul_colors() {
        let color1 = Color::new(1.0, 0.5, 0.25);
        let color2 = Color::new(0.5, 0.25, 4.0);
        let result = color1 * color2;
        assert_eq!(result.r(), 0.5);
        assert_eq!(result.g(), 0.125);
        assert_eq!(result.b(), 1.0);
    }

    #[test]
    fn test_mul_color() {
        let color = Color::new(1.0, 0.5, 0.25);
        let result = color * 2.0;
        assert_eq!(result.r(), 2.0);
        assert_eq!(result.g(), 1.0);
        assert_eq!(result.b(), 0.5);
    }

    #[test]
    fn test_div_color() {
        let color = Color::new(1.0, 0.5, 0.25);
        let result = color / 2.0;
        assert_eq!(result.r(), 0.5);
        assert_eq!(result.g(), 0.25);
        assert_eq!(result.b(), 0.125);
    }
}
