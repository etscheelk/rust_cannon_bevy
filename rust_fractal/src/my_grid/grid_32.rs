use std::f32::consts::PI;

use rand::Rng;

use crate::fractal::Fractalize;

pub type MyColorImage = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

impl Fractalize for MyColorImage
{
    fn fractalize(&mut self, p: crate::fractal::FractalizeParameters) -> () 
    {
        let (mut x, mut y) = p.init_x_y();
        let max_points = p.max_points();
        
        let rot = p.rot();
        let rot_cos = rot.cos();
        let rot_sin = rot.sin();

        let theta_offset = p.theta_offset();

        let _method = *p.method();

        let distr = 
            rand::distr::Uniform::new(0, u64::MAX).unwrap();
        let rands: Vec<u64> = rand::rng().sample_iter(&distr).take((max_points / 64) as usize).collect();


        let rows = self.height();
        let cols = self.width();

        let transform = 
        move |x: f32, y: f32, s: bool|
        {
            let (x, y) = 
            if s
            {
                (
                    x * rot_cos + y * rot_sin,
                    y * rot_cos - x * rot_sin
                )
            }
            else
            {
                let rad = x * 0.5 + 0.5;
                // let theta: f32 = y * PI + theta_offset;

                use crate::fractal::FractalMethod::*;
                let theta: f32 = match _method
                {
                    Default => y * PI + theta_offset,
                    MultiplyTheta => y * PI * theta_offset,
                };
                (
                    rad * theta.cos(),
                    rad * theta.sin()
                )
            };

            (x, y)
        };

        let xy_to_grid_loc =
        move |x: f32, y: f32| -> (u32, u32)
        {
            let r: f32 = (y * 0.5 + 0.5) * rows as f32;
            let c: f32 = (x * 0.5 + 0.5) * cols as f32;

            // Testing showed that this is faster than using the as operator.
            // Normally rust has protection against floats being too large to fit in an int,
            // but in this case we know the values will be in range 0..width
            unsafe {
                (r.to_int_unchecked(), c.to_int_unchecked())
            }
        };

        let _do_both_transformations =
        ||
        {
            for rr in rands
            {
                for i in 0..64_u64
                {
                    let this_r = rr & (1 << i);

                    // first
                    let (xx, yy) = transform(x, y, this_r == 0);
                    let (r, c) = xy_to_grid_loc(xx, yy);
                    if let Some(p) = self.get_pixel_mut_checked(c, r)
                    {
                        // Testiing showed that using add(..) may be faster, but not
                        // if we want to check for overflow.
                        // This prevents a potential panic on overflow and allows over-exposure.
                        //
                        // Potential target for simd optimization?
                        p[0] = p[0].checked_add(1).unwrap_or(p[0]);
                        p[1] = p[1].checked_add(1).unwrap_or(p[1]);
                        p[2] = p[2].checked_add(1).unwrap_or(p[2]);
                    }

                    // second
                    let (xx, yy) = transform(x, y, this_r != 0);
                    let (r, c) = xy_to_grid_loc(xx, yy);
                    if let Some(p) = self.get_pixel_mut_checked(c, r)
                    {
                        p[0] = p[0].checked_add(1).unwrap_or(p[0]);
                        p[1] = p[1].checked_add(1).unwrap_or(p[1]);
                        p[2] = p[2].checked_add(1).unwrap_or(p[2]);
                    }

                    (x, y) = (xx, yy);
                }
            }
        };
        
        _do_both_transformations();
    }
}