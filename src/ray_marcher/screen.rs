use core::f64;
use std::ops::{Add, Mul, MulAssign};

pub trait Pixelatable {
    type Component;
    fn get_red_channel(&self) -> Self::Component;
    fn get_green_channel(&self) -> Self::Component;
    fn get_blue_channel(&self) -> Self::Component;

    fn set_red_channel(&mut self, new_r: Self::Component);
    fn set_green_channel(&mut self, new_g: Self::Component);
    fn set_blue_channel(&mut self, new_b: Self::Component);

    fn new() -> Self;
}

pub trait Displayable {
    type Component;
    type Unit;
    fn get_red_channel(&self, index: (u32, u32)) -> Self::Component;
    fn get_green_channel(&self, index: (u32, u32)) -> Self::Component;
    fn get_blue_channel(&self, index: (u32, u32)) -> Self::Component;

    fn set_red_channel(&mut self, index: (u32, u32), new_r: Self::Component);
    fn set_green_channel(&mut self, index: (u32, u32), new_g: Self::Component);
    fn set_blue_channel(&mut self, index: (u32, u32), new_b: Self::Component);

    fn get_color_components(&self, index: (u32, u32)) -> (Self::Component, Self::Component, Self::Component);
    fn set_color_components(&mut self, index: (u32, u32), display_unit: &Self::Unit);

    fn get_resolution(&self) -> (u32, u32);
}

pub struct Screen<P>
where
    P: Pixelatable + Add + Mul,
{
    pixels: Vec<P>,
    resolution: (u32, u32),
}

impl<P> Screen<P> 
where
    P: Pixelatable + Add + Mul<P> + Mul<f64> + MulAssign<f64>,
{
    pub fn new(res: (u32, u32)) -> Self{
        let mut v = Vec::<P>::new();
        (0..res.0 * res.1).into_iter().for_each(|_i|{
            v.push(P::new());
        });
        Self { pixels: v, resolution: res }
    }
}

impl<P> Screen<P> 
where
    P: Pixelatable + Add + Mul<P, Output = P> + Mul<f64, Output = P> + MulAssign<f64>{
    pub fn apply_reciperical(&mut self, reciperical: f64){
        self.pixels.iter_mut().for_each(|p| {
            *p *= reciperical
        })
    }
}

impl<P> Displayable for Screen<P>
where
    P: Pixelatable + Add + Mul,
{
    fn get_red_channel(&self, index: (u32, u32)) -> Self::Component {
        let (num_rows, _num_cols) = self.get_resolution();
        let linear_index = (num_rows * index.0) + index.1;
        self.pixels
            .get(linear_index as usize)
            .unwrap()
            .get_red_channel()
    }

    fn get_green_channel(&self, index: (u32, u32)) -> Self::Component {
        let (num_rows, _num_cols) = self.get_resolution();
        let linear_index = (num_rows * index.0) + index.1;
        self.pixels
            .get(linear_index as usize)
            .unwrap()
            .get_green_channel()
    }

    fn get_blue_channel(&self, index: (u32, u32)) -> Self::Component {
        let (num_rows, _num_cols) = self.get_resolution();
        let linear_index = (num_rows * index.0) + index.1;
        self.pixels
            .get(linear_index as usize)
            .unwrap()
            .get_blue_channel()
    }

    fn set_red_channel(&mut self, index: (u32, u32), new_r: Self::Component) {
        let (num_rows, _num_cols) = self.get_resolution();
        let linear_index = (num_rows * index.0) + index.1;
        self.pixels
            .get_mut(linear_index as usize)
            .unwrap()
            .set_red_channel(new_r);
    }

    fn set_green_channel(&mut self, index: (u32, u32), new_g: Self::Component) {
        let (num_rows, _num_cols) = self.get_resolution();
        let linear_index = (num_rows * index.0) + index.1;
        self.pixels
            .get_mut(linear_index as usize)
            .unwrap()
            .set_green_channel(new_g);
    }

    fn set_blue_channel(&mut self, index: (u32, u32), new_b: Self::Component) {
        let (num_rows, _num_cols) = self.get_resolution();
        let linear_index = (num_rows * index.0) + index.1;
        self.pixels
            .get_mut(linear_index as usize)
            .unwrap()
            .set_blue_channel(new_b);
    }

    fn get_color_components(&self, index: (u32, u32)) -> (Self::Component, Self::Component, Self::Component) {
        (
            self.get_red_channel(index.clone()),
            self.get_green_channel(index.clone()),
            self.get_blue_channel(index.clone()),
        )
    }

    fn set_color_components(&mut self, index: (u32, u32), display_unit: &Self::Unit){
        self.set_red_channel(index, display_unit.get_red_channel());
        self.set_green_channel(index, display_unit.get_green_channel());
        self.set_blue_channel(index, display_unit.get_blue_channel());
    }

    fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    type Component = P::Component ;

    type Unit = P;
}
