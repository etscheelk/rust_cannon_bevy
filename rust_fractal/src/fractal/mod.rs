use derive_setters::*;
use derive_getters::*;

pub trait Fractalize
{
    fn fractalize(&mut self, p: FractalizeParameters) -> ();
}

#[derive(Setters, Getters, Clone, Copy, Debug, PartialEq)]
#[setters(prefix = "with_")]
#[getter(prefix = "get_")]
pub struct FractalizeParameters
{
    // #[setters(skip)]
    pub init_x_y: (f32, f32),
    pub rot: f32,
    pub theta_offset: f32,
    pub method: FractalMethod,
    pub max_points: u32,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum FractalMethod
{
    #[default]
    Default,
    MultiplyTheta,
}

impl Default for FractalizeParameters
{
    fn default() -> Self {
        Self 
        { 
            init_x_y: (0.0, 0.5), 
            rot: 1.724643921305295,
            theta_offset: 3.0466792337230033,
            method: FractalMethod::default(),
            max_points: 1_000_000
        }
    }
}