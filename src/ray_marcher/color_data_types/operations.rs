use super::*;
use std::ops;

macro_rules! color_op_impl {
    ($impl_op: ident, $op: tt, $impl_op_func: ident) => {
        impl ops::$impl_op<Color> for Color {
            type Output = Color;

            fn $impl_op_func(self, rhs: Self) -> Self::Output {
                let mut s = Self { r: self.r $op rhs.r, g: self.g $op rhs.g, b: self.b $op rhs.b };
                s.clamp();
                s
            }
        }
        impl ops::$impl_op<f64> for Color {
            type Output = Color;

            fn $impl_op_func(self, rhs: f64) -> Self::Output {
                let mut s = Self { r: self.r $op rhs, g: self.g $op rhs, b: self.b $op rhs };
                s.clamp();
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
                self.clamp();
            }
        }
        impl ops::$impl_op<f64> for Color {
            fn $impl_op_func(&mut self, rhs: f64){
                self.r $op rhs;
                self.g $op rhs;
                self.b $op rhs;
                self.clamp();
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
