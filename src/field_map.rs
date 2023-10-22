use super::complex::Z;

#[derive(Clone, Copy)]
pub struct FieldMap {
    pub upper_left: Z,
    //pub lower_right: Z,
    pub re_resolution: usize,
    pub im_resolution: usize,
    pub re_delta: f64,
    pub im_delta: f64,
}

impl FieldMap {
    pub fn new(upper_left: Z, lower_right: Z, re_resolution: usize, im_resolution: usize) -> Self {
        FieldMap {
            upper_left,
            //lower_right,
            re_resolution,
            im_resolution,
            re_delta: (lower_right.re() - upper_left.re()) / re_resolution as f64,
            im_delta: (upper_left.im() - lower_right.im()) / im_resolution as f64,
        }
    }

    pub fn get_point(self, index: usize) -> Z {
        let (x, y) = (index % self.re_resolution, index / self.re_resolution);

        Z::new(
            self.upper_left.re() + x as f64 * self.re_delta,
            self.upper_left.im() - y as f64 * self.im_delta,
        )
    }

    pub fn get_limit(self) -> usize {
        self.re_resolution * self.im_resolution
    }
}

