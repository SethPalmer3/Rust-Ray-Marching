use std::ops;
use super::*;

macro_rules! impl_ops_for_point {
    ($trait:ident, $func:ident, $op:tt) => {
        impl ops::$trait<f64> for Point {
            fn $func(&mut self, rhs: f64) {
                self.x $op rhs;
                self.y $op rhs;
                self.z $op rhs;
            }
        }
        impl ops::$trait<Point> for Point {
            fn $func(&mut self, rhs: Point) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }
    };
    ($trait:ident, $func:ident, $op:tt, $output:ty) => {
        impl ops::$trait<f64> for Point {
            type Output = Point;

            fn $func(self, rhs: f64) -> Self::Output {
                Self {x: self.x $op rhs, y: self.y $op rhs, z: self.z $op rhs}
            }
        }
        impl ops::$trait<Point> for Point {
            type Output = Point;

            fn $func(self, rhs: Point) -> Self::Output {
                Self {x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z}
            }
        }
    };
}

impl_ops_for_point!(AddAssign, add_assign, +=);
impl_ops_for_point!(SubAssign, sub_assign, -=);
impl_ops_for_point!(MulAssign, mul_assign, *=);
impl_ops_for_point!(DivAssign, div_assign, /=);

impl_ops_for_point!(Add, add, +, f64);
impl_ops_for_point!(Sub, sub, -, f64);
impl_ops_for_point!(Mul, mul, *, f64);
impl_ops_for_point!(Div, div, /, f64);
