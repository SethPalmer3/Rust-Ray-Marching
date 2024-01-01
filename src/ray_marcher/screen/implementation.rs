use super::*;

impl<P> Screen<P> 
where
    P: Pixelatable + Add + Mul,
{
    pub fn new(res: (u32, u32)) -> Self{
        let mut v = Vec::<P>::new();
        (0..res.0 * res.1).into_iter().for_each(|_i|{
            v.push(P::new());
        });
        Self { pixels: v, resolution: res }
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

    fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    type Component = P::Component ;
}
