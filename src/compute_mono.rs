use anyhow::Result;

use num::Complex;

use crate::{
    field_map::FieldMap,
    strategy::{ComputationParams, ComputationStrategy},
};

pub struct MandelbrotMono {
    params: Option<ComputationParams>,
}

impl MandelbrotMono {
    pub fn new() -> Self {
        MandelbrotMono { params: None }
    }

    #[inline(always)]
    fn escape_time(c: Complex<f64>, max_iters: usize) -> u8 {
        let mut z = Complex::new(0.0, 0.0);
        let mut i = 0;
        while i < max_iters && z.norm_sqr() <= 4.0 {
            z = z * z + c;
            i += 1;
        }
        ((max_iters - i) & 0xff) as u8
    }
}

impl ComputationStrategy for MandelbrotMono {
    /// This function is called to dump the computation context info.
    fn dump_info(&self) -> Result<()> {
        println!("MandelbrotMono computation info: Single core computation.");
        println!("------------------------------------------------------------");
        Ok(())
    }

    /// This function is called to initialize the computation context.
    fn init(&mut self, params: &ComputationParams) -> Result<()> {
        self.params = Some(params.clone());
        Ok(())
    }

    /// This function is called to setup the computation context.
    fn setup(&mut self) -> Result<()> {
        Ok(())
    }

    /// This function is called to compute the Mandelbrot set.
    fn compute(&self) -> Result<Vec<u8>> {
        let params = self
            .params
            .as_ref()
            .expect("Computation parameters not initialized.");

        let field_map = FieldMap::new(
            params.upper_left,
            params.lower_right,
            params.width as usize,
            params.height as usize,
        );

        let values: Vec<u8> = (0..field_map.get_limit())
            .map(|i| Self::escape_time(field_map.get_point(i), params.max_iters))
            .collect();

        Ok(values)
    }
}
