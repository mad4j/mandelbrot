
use num::Complex;
use anyhow::Result;


pub trait MandelbrotComputation {
    fn compute(
        width: u32,
        height: u32,
        max_iters: usize,
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
    ) -> Result<MandelbrotComputationResult>;

    fn dump_info() -> Result<()>;
}

pub struct MandelbrotComputationResult {
    pub values: Vec<u8>,
    pub elapsed_time: std::time::Duration,
}


pub struct FieldMap {
    pub re_resolution: usize,
    pub im_resolution: usize,
    pub precomputed_re: Vec<f32>,
    pub precomputed_im: Vec<f32>,
}

impl FieldMap {
    pub fn new(
        upper_left: Complex<f32>,
        lower_right: Complex<f32>,
        re_resolution: usize,
        im_resolution: usize,
    ) -> Self {
        let re_delta = (lower_right.re - upper_left.re) / re_resolution as f32;
        let im_delta = (upper_left.im - lower_right.im) / im_resolution as f32;

        let precomputed_re: Vec<f32> = (0..re_resolution)
            .map(|x| upper_left.re + x as f32 * re_delta)
            .collect();

        let precomputed_im: Vec<f32> = (0..im_resolution)
            .map(|y| upper_left.im - y as f32 * im_delta)
            .collect();

        FieldMap {
            re_resolution,
            im_resolution,
            precomputed_re,
            precomputed_im,
        }
    }

    #[inline(always)]
    pub fn get_point(&self, index: usize) -> Complex<f32> {
        let (x, y) = (index % self.re_resolution, index / self.re_resolution);

        Complex::new(self.precomputed_re[x], self.precomputed_im[y])
    }

    pub fn get_limit(&self) -> usize {
        self.re_resolution * self.im_resolution
    }
}
