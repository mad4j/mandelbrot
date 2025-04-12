use num::Complex;

use anyhow::Result;

use rayon::prelude::*;

use crate::{
    field_map::FieldMap,
    strategy::{ComputationParams, ComputationStrategy},
};

pub struct MandelbrotRayon {
    params: Option<ComputationParams>,
}

impl MandelbrotRayon {
    pub fn new() -> Self {
        MandelbrotRayon { params: None }
    }
}

impl ComputationStrategy for MandelbrotRayon {
    fn dump_info(&self) -> Result<()> {
        println!("MandelbrotRayon computation info: Parallelized using Rayon.");
        println!("------------------------------------------------------------");
        Ok(())
    }

    fn init(&mut self, params: &ComputationParams) -> Result<()> {
        self.params = Some(params.clone());
        Ok(())
    }

    fn setup(&mut self) -> Result<()> {
        Ok(())
    }

    fn compute(&self) -> Result<Vec<u8>> {
        let params = self
            .params
            .as_ref()
            .expect("Computation parameters not initialized.");

        compute(
            params.width,
            params.height,
            params.max_iters,
            params.upper_left,
            params.lower_right,
        )
    }
}

#[inline(always)]
fn compute(
    width: u32,
    height: u32,
    max_iters: usize,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Result<Vec<u8>> {
    let field_map = FieldMap::new(upper_left, lower_right, width as usize, height as usize);

    let values: Vec<u8> = (0..field_map.get_limit())
        .into_par_iter()
        .map(|i| FieldMap::escape_time(field_map.get_point(i), max_iters))
        .collect();

    Ok(values)
}
