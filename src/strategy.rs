use anyhow::Result;
use num::Complex;

/// Struct to hold the parameters for the computation.
#[derive(Debug, Clone)]
pub struct ComputationParams {
    pub width: u32,
    pub height: u32,
    pub max_iters: usize,
    pub upper_left: Complex<f64>,
    pub lower_right: Complex<f64>,
}

pub trait ComputationStrategy {
    fn dump_info(&self) -> Result<()>;

    fn init(&mut self, params: &ComputationParams) -> Result<()>;

    fn setup(&mut self) -> Result<()>;

    fn compute(&self) -> Result<Vec<u8>>;
}

pub struct ComputationContext {
    strategy: Box<dyn ComputationStrategy>,
}

impl ComputationContext {
    pub fn new(strategy: Box<dyn ComputationStrategy>) -> Self {
        ComputationContext { strategy }
    }
}

impl ComputationStrategy for ComputationContext {
    fn dump_info(&self) -> Result<()> {
        self.strategy.dump_info()
    }

    fn init(&mut self, params: &ComputationParams) -> Result<()> {
        self.strategy.init(params)
    }

    fn setup(&mut self) -> Result<()> {
        self.strategy.setup()
    }

    fn compute(&self) -> Result<Vec<u8>> {
        self.strategy.compute()
    }
}
