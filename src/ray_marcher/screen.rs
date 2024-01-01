pub mod implementation;
use std::ops::{Add, Mul};

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
    fn get_red_channel(&self, index: (u32, u32)) -> Self::Component;
    fn get_green_channel(&self, index: (u32, u32)) -> Self::Component;
    fn get_blue_channel(&self, index: (u32, u32)) -> Self::Component;

    fn set_red_channel(&mut self, index: (u32, u32), new_r: Self::Component);
    fn set_green_channel(&mut self, index: (u32, u32), new_g: Self::Component);
    fn set_blue_channel(&mut self, index: (u32, u32), new_b: Self::Component);

    fn get_color_components(&self, index: (u32, u32)) -> (Self::Component, Self::Component, Self::Component);

    fn get_resolution(&self) -> (u32, u32);
}

pub struct Screen<P>
where
    P: Pixelatable + Add + Mul,
{
    pixels: Vec<P>,
    resolution: (u32, u32),
}

