use super::*;
use std::ops;


macro_rules! impl_ops_assign_for_direction {
    ($trait:ident, $func:ident, $op:tt) => {
        impl ops::$trait<Direction> for Direction { // Operation assign to other direction types
            fn $func(&mut self, rhs: Direction) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }
        impl ops::$trait<f64> for Direction { // Operation assign to float 64 types
            fn $func(&mut self, rhs: f64) {
                self.x $op rhs;
                self.y $op rhs;
                self.z $op rhs;
            }
        }
    };
}
macro_rules! impl_ops_for_direction {
    ($trait: ident, $func: ident, $op:tt) => {
        impl ops::$trait<Direction> for Direction { // Operation to other direction types
            type Output = Direction;
            fn $func(self, rhs: Direction) -> Self::Output{
                Direction { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z}
            }
        }
        impl ops::$trait<f64> for Direction { // Operation to float 64 types
            type Output = Direction;
            fn $func(self, rhs: f64) -> Self::Output{
                Direction { x: self.x $op rhs, y: self.y $op rhs, z: self.z $op rhs}
            }
        }
    };
}

impl_ops_assign_for_direction!(AddAssign, add_assign, +=);
impl_ops_assign_for_direction!(SubAssign, sub_assign, -=);
impl_ops_assign_for_direction!(MulAssign, mul_assign, *=);
impl_ops_assign_for_direction!(DivAssign, div_assign, /=);

impl_ops_for_direction!(Add, add, +);
impl_ops_for_direction!(Sub, sub, -);
impl_ops_for_direction!(Mul, mul, *);
impl_ops_for_direction!(Div, div, /);
