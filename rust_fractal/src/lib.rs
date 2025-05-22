mod fractal;
mod my_grid;

// for full fractal code, see https://github.com/etscheelk/RustFractal.
// also contains gpu experiementation.
// This is a pared down version of the fractal code.

pub use crate::fractal::{Fractalize, FractalizeParameters, FractalMethod};
pub use crate::my_grid::grid_32::MyColorImage;