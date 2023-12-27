use std::ops;

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
//-------- Point ------------------

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Self { x, y, z }
    }
    pub fn to_direction(self) -> Direction{
        Direction { x: self.x, y: self.y, z: self.z }
    }
    pub fn distance_to(&self, p: &Self) -> f64{
        ((p.x - self.x).powf(2.0) + (p.y - self.y).powf(2.0) + (p.z - self.z).powf(2.0)).abs().sqrt()
    }
    pub fn normalize(&mut self) -> Self{
        *self /= self.distance_to(&Point { x: 0.0, y: 0.0, z: 0.0 });
        *self
    }
}

//------ Direction Vector ---------

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Direction{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Direction {
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Self { x, y, z }
    }
    pub fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn get_norm(&self) -> Self {
        self.clone() / self.length()
    }
    pub fn to_point(self) -> Point {
        Point { x: self.x, y: self.y, z: self.z }
    }
    pub fn get_dot(d1: &Self, d2: &Self) -> f64 {
        d1.x*d2.x + d1.y*d2.y + d1.z*d2.z
    }
    pub fn rotate_vector_around_z(&mut self, around_z: f64){
        // let len = self.length();
        let len = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
        let mut current_around_z = (self.y / self.x).atan();
        if self.x < 0.0{
            current_around_z = current_around_z + 180_f64.to_radians();
        }

        let x_delta = len * (current_around_z + around_z).cos();
        let y_delta = len * (current_around_z + around_z).sin();

        self.x = x_delta;
        self.y = y_delta;
    }
    pub fn rotate_vector_around_y(&mut self, around_y: f64){
        // let len = self.length();
        let len = (self.x.powf(2.0) + self.z.powf(2.0)).sqrt();
        let mut current_around_y = (self.x / self.z).atan();
        if self.z < 0.0{
            current_around_y = current_around_y + 180_f64.to_radians();
        }

        let x_delta = len * (current_around_y + around_y).sin();
        let z_delta = len * (current_around_y + around_y).cos();

        self.x = x_delta;
        self.z = z_delta;
    }
    pub fn rotate_vector(&mut self, around_z: f64, around_y: f64){
        self.rotate_vector_around_z(around_z);
        self.rotate_vector_around_y(around_y);
    }
    pub fn set_norm(&mut self){
        *self = self.get_norm();
    }
}

macro_rules! impl_ops_assign_for_direction {
    ($trait:ident, $func:ident, $op:tt) => {
        impl ops::$trait<Direction> for Direction {
            fn $func(&mut self, rhs: Direction) {
                self.x $op rhs.x;
                self.y $op rhs.y;
                self.z $op rhs.z;
            }
        }
        impl ops::$trait<f64> for Direction {
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
        impl ops::$trait<Direction> for Direction {
            type Output = Direction;
            fn $func(self, rhs: Direction) -> Self::Output{
                Direction { x: self.x $op rhs.x, y: self.y $op rhs.y, z: self.z $op rhs.z}
            }
        }
        impl ops::$trait<f64> for Direction {
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


//---------- Constants ------------
pub mod constants {
    use super::*;
    pub static ORIGIN: Point = Point{x: 0.0, y: 0.0, z: 0.0};
    pub static X_DIR: Direction = Direction{x: 1.0, y: 0.0, z: 0.0};
    pub static Y_DIR: Direction = Direction{x: 0.0, y: 1.0, z: 0.0};
    pub static Z_DIR: Direction = Direction{x: 0.0, y: 0.0, z: 1.0};
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_direction_length(){
        let dir = Direction::new(1.0, 1.0, 0.0);
        assert_eq!(dir.length(), (2.0_f64).sqrt());
    }

    #[test]
    fn test_normalize_single_axis_len(){
        let dir = Direction::new(2.0, 0.0, 0.0);
        assert_eq!(dir.get_norm().length(), 1.0);
    }

    #[test]
    fn test_normalize_single_axis_dot(){
        let dir = Direction::new(2.0, 0.0, 0.0);
        assert_eq!(Direction::get_dot(&dir.get_norm(), &dir), dir.length());
    }

    #[test]
    fn test_normalize_double_axis_len(){
        let dir = Direction::new(1.0, 1.0, 0.0);
        assert!((1.0 - dir.get_norm().length()).abs() < 0.0001);
    }

    #[test]
    fn test_normalize_double_axis_dot(){
        let dir = Direction::new(2.0, 1.0, 0.0);
        assert_eq!(Direction::get_dot(&dir.get_norm(), &dir), dir.length());
    }

    #[test]
    fn test_point_distance_single_axis(){
        let dir1 = Point::new(1.0, 0.0, 0.0);
        let dir2 = Point::new(-1.0, 0.0, 0.0);
        assert_eq!(dir1.distance_to(&dir2).abs(), 2.0);
    }

    #[test]
    fn test_point_distance_double_axis(){
        let dir1 = Point::new(1.0, 1.0, 0.0);
        let dir2 = Point::new(-1.0, -1.0, 0.0);
        let expected_dist = 2.0 * (2_f64.sqrt());
        assert!((expected_dist - dir1.distance_to(&dir2).abs()).abs() < 0.0001);
    }

    #[test]
    fn test_rotation_90_degrees_around_z(){
        let mut dir = Direction::new(1.0, 0.0, 0.0);
        dir.rotate_vector_around_z(90_f64.to_radians());
        println!("{:?}", dir);
        assert!((dir - Direction::new(0.0, 1.0, 0.0)).length() < 0.0001);
    }

    #[test]
    fn test_rotation_around_z_on_z(){
        let mut dir = Direction::new(0.0, 0.0, 1.0);
        dir.rotate_vector_around_z(90_f64.to_radians());
        println!("{:?}", dir);
        assert!((dir - Direction::new(0.0, 0.0, 1.0)).length() < 0.0001);
    }

    #[test]
    fn test_rotation_90_degrees_around_y(){
        let mut dir = Direction::new(1.0, 0.0, 0.0);
        dir.rotate_vector_around_y(90_f64.to_radians());
        println!("{:?}", dir);
        assert!((dir - Direction::new(0.0, 0.0, -1.0)).length() < 0.0001);
    }

    #[test]
    fn test_rotation_around_y_on_y(){
        let mut dir = Direction::new(0.0, 1.0, 0.0);
        dir.rotate_vector_around_y(90_f64.to_radians());
        println!("{:?}", dir);
        assert!((dir - Direction::new(0.0, 1.0, 0.0)).length() < 0.0001);
    }

    #[test]
    fn test_rotation_90_degrees_x_y(){
        let mut dir = Direction::new(1.0, 0.0, 0.0);
        dir.rotate_vector(90_f64.to_radians(), 90_f64.to_radians());
        println!("{:?}", dir);
        assert!((dir - Direction::new(0.0, 1.0, 0.0)).length() < 0.0001);
    }

}
