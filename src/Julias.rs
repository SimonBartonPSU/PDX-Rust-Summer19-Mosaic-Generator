//base code credited to: https://crates.io/crates/image
//resource on julia_set fractals: https://en.wikipedia.org/wiki/Julia_set#Pseudocode_for_normal_Julia_sets

///Julia Set Fractal - "the Julia set consists of values such that an arbitrarily
/// small perturbation can cause drastic changes in the sequence of iterated function values.
/// Thus the behavior of the Julia set is "chaotic"."

/// Each pixel in the user specified dimensions runs through
/// the loop that calculates the Julia set formula of (f(z) = z^2 + c), and will continue to
/// do so until the value is outside the appropriate range where it can still generate
/// correctly. The int value that is broken out of the function is returned
/// and used for the color shade of the currently specfied pixel.
pub fn pixel_setter((complex_x, complex_y): (f32, f32), mut iteration: u64, randjulia: u64) -> u64 {
    //determine which julia_set fractal will be generated (On the wiki page source under "Quadraic polynomials")
    let complex_num = match randjulia {
        //every stage of the julia set is listed as a possible option
        1 => num::Complex::new(-0.8, 0.0),
        2 => num::Complex::new(0.285, 0.0),
        3 => num::Complex::new(-0.4, 0.6),
        4 => num::Complex::new(0.45, 0.1428),
        5 => num::Complex::new(0.285, 0.01),
        6 => num::Complex::new(-0.70176, -0.3842),
        7 => num::Complex::new(-0.835, -0.2321),
        8 => num::Complex::new(-0.8, 0.156),
        9 => num::Complex::new(-0.7269, 0.1889),
        10 => num::Complex::new(0.0, -0.8),
        _ => num::Complex::new(-0.4, 0.6),
    };

    let mut value = num::Complex::new(complex_x, complex_y);

    while iteration < 255 && value.norm() <= 2.0 {
        //the julia fractal formula (f(z) = z^2 + c)
        value = value * value + complex_num;
        iteration += 1;
    }

    iteration
}


//Multilevel
pub fn pixel_set_multi((complex_x, complex_y): (f32, f32), mut iteration: u64, randjulia: u64) -> u64 {
    
    let complex_num = 0.626;

    let mut value = num::Complex::new(complex_x, complex_y);

    while iteration < 255 && value.norm() <= 2.0 {
        //the multi-julia fractal formula (f(z) = z^n + c)
        value = (value.powf(7.0)) + complex_num,
      

        iteration += 1;
    }

    iteration

    

}