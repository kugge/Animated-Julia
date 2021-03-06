use num_complex::Complex;


pub const MAX_ITER: u32 = 80; // Max iteration for the coefficient
const MAX_NORM: f32 = 2.0;      // Max norm


/// Return the Julia coefficient of a given point with a given coefficient
///
/// # Arguments
///
/// * `a` - Real part of the coefficient
/// * `b` - Imaginary part of the coefficient
/// * `x` - Real part of the point
/// * `y` - Imaginary part of the point
pub fn get_coefficient(a: f32, b:f32, x: f32, y:f32) -> f32 {
    let c: Complex<f32> = Complex::new(a, b);
    let mut z: Complex<f32> = Complex::new(x, y);
    let mut i: u32 = 0;
    while (z.norm() < MAX_NORM) && (i < MAX_ITER) {
        z = z*z + c;
        i = i + 1;
    }
    if i == MAX_ITER {
        return i as f32
    } 
    else {
        return i as f32 - z.norm().log2().ln()
    }
}


/// Return the Julia coefficient of a given point with a given coefficient
///
/// # Arguments
///
/// * `x` - Real part of the point
/// * `y` - Imaginary part of the point
pub fn get_mandelbrot_coefficient(x: f32, y:f32) -> f32 {
    let c: Complex<f32> = Complex::new(x, y);
    let mut z: Complex<f32> = Complex::new(0.0, 0.0);
    let mut i: u32 = 0;
    while (z.norm() < MAX_NORM) && (i < MAX_ITER) {
        z = z*z + c;
        i = i + 1;
    }
    if i == MAX_ITER {
        return i as f32
    }
    else {
        return i as f32 - z.norm().log2().ln()
    }
}

