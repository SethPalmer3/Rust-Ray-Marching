use std::ops;

use super::screen::Pixelatable;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

pub fn f64_to_u8(c: f64) -> u8 {
    (c * 255_f64) as u8
}

pub static BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};

#[allow(dead_code)]
fn between_0_1(i: f64) -> bool {
    0.0 <= i && i <= 1.0
}

impl Pixelatable for Color {
    fn get_red_channel(&self) -> Self::Component {
        self.r()
    }

    fn get_green_channel(&self) -> Self::Component {
        self.g()
    }

    fn get_blue_channel(&self) -> Self::Component {
        self.b()
    }

    fn set_red_channel(&mut self, new_r: Self::Component) {
        self.r = new_r;
    }

    fn set_green_channel(&mut self, new_g: Self::Component) {
        self.g = new_g;
    }

    fn set_blue_channel(&mut self, new_b: Self::Component) {
        self.b = new_b;
    }

    type Component = f64;

    fn new() -> Self {
        Color::new(1.0, 1.0, 1.0)
    }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        let s = Color { r, g, b };
        // s.lamp();
        s
    }

    pub fn get_components(&self) -> (f64, f64, f64) {
        (self.r, self.g, self.b)
    }
    pub fn get_u8_components(&self) -> (u8, u8, u8) {
        let (r, g, b) = self.get_components();
        ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn normalize(&mut self) {
        todo!()
    }

    pub fn clamp(&mut self) {
        self.r = self.r.clamp(0.0, 1.0);
        self.g = self.g.clamp(0.0, 1.0);
        self.b = self.b.clamp(0.0, 1.0);
    }

    pub fn blend_colors(color1: &Self, color2: &Self, ratio: f64) -> Self {
        let col1 = color1.clone();
        let col2 = color2.clone();
        let mut first_part: Color = (col1 * (1.0 - ratio)) + (col2 * ratio);
        first_part.r = first_part.r.sqrt();
        first_part.g = first_part.g.sqrt();
        first_part.b = first_part.b.sqrt();
        first_part
    }
}

macro_rules! color_op_impl {
    ($impl_op: ident, $op: tt, $impl_op_func: ident) => {
        impl ops::$impl_op<Color> for Color {
            type Output = Color;

            fn $impl_op_func(self, rhs: Self) -> Self::Output {
                let s = Self { r: self.r $op rhs.r, g: self.g $op rhs.g, b: self.b $op rhs.b };
                // s.clamp();
                s
            }
        }
        impl ops::$impl_op<f64> for Color {
            type Output = Color;

            fn $impl_op_func(self, rhs: f64) -> Self::Output {
                let s = Self { r: self.r $op rhs, g: self.g $op rhs, b: self.b $op rhs };
                // s.clamp();
                s
            }
        }
    };

}
macro_rules! color_op_assign_impl {
    ($impl_op: ident, $op: tt, $impl_op_func: ident) => {
        impl ops::$impl_op for Color {
            fn $impl_op_func(&mut self, rhs: Self){
                self.r $op rhs.r;
                self.g $op rhs.g;
                self.b $op rhs.b;
                // self.clamp();
            }
        }
        impl ops::$impl_op<f64> for Color {
            fn $impl_op_func(&mut self, rhs: f64){
                self.r $op rhs;
                self.g $op rhs;
                self.b $op rhs;
                // self.clamp();
            }
        }
    };
}

color_op_impl!(Add, +, add);
color_op_impl!(Sub, -, sub);
color_op_impl!(Mul, *, mul);
color_op_impl!(Div, /, div);
color_op_assign_impl!(AddAssign, +=, add_assign);
color_op_assign_impl!(SubAssign, -=, sub_assign);
color_op_assign_impl!(MulAssign, *=, mul_assign);
color_op_assign_impl!(DivAssign, /=, div_assign);

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_op_color() {
        let lhs = Color::new(1.0, 0.0, 0.0);
        let rhs = Color::new(0.0, 1.0, 0.0);
        assert_eq!(lhs + rhs, Color::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_op_f64() {
        let lhs = Color::new(0.5, 0.0, 0.0);
        assert_eq!(lhs * 2.0, Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_op_assign_color() {
        let mut lhs = Color::new(1.0, 0.0, 0.0);
        lhs += Color::new(0.0, 1.0, 0.0);
        assert_eq!(lhs, Color::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_op_assign_f64() {
        let mut lhs = Color::new(0.5, 0.0, 0.0);
        lhs *= 2.0;
        assert_eq!(lhs, Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_op_overflow_color() {
        let lhs = Color::new(1.0, 0.0, 0.0);
        let rhs = Color::new(1.0, 0.0, 0.0);
        assert_eq!(lhs + rhs, Color::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn test_op_overflow_f64() {
        let lhs = Color::new(1.0, 0.0, 0.0);
        assert_eq!(lhs * 2.0, Color::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn test_op_assign_overflow_color() {
        let lhs = Color::new(1.0, 0.0, 0.0);
        let rhs = Color::new(1.0, 0.0, 0.0);
        assert_eq!(lhs + rhs, Color::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn test_op_assign_overflow_f64() {
        let lhs = Color::new(1.0, 0.0, 0.0);
        assert_eq!(lhs * 2.0, Color::new(1.0, 0.0, 0.0))
    }

    #[test]
    #[should_panic]
    fn test_new_greater_1() {
        let _t = Color::new(99.0, 0.0, 0.0);
    }

    #[test]
    #[should_panic]
    fn test_new_less_0() {
        let _t = Color::new(-99.0, 0.0, 0.0);
    }
}
