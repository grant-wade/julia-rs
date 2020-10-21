mod julia {
    use image;
    // use math;
    use num::Complex;
    use num::traits::Pow;
    use num::traits::real::Real;

    /// # Description
    /// Checks if a complex point `z` is in the julia set defined from `c`
    ///
    /// # Arguments
    /// * `z` - the current pixel being checked
    /// * `c` - the equation for the julia set
    /// * `n` - number of iterations to run
    /// * `e` - exponential value to use
    ///
    /// # Returns
    /// -1 indicates that the point has not left the set, if any
    /// other number is returned that indicates the steps before
    /// leaving the julia set
    pub fn in_jset(mut z: Complex<f64>, c: Complex<f64>, n: i64, e: f64) -> i64 {
        for x in 0..n {
            z = z.expf(e) + c;
            if ((z.re.pow(2) + z.im.pow(2)) as f64).sqrt() > 2.0 {
                return x;
            }
        }
        return -1;
    }

    /// # Description
    /// Generate the input set at a determined zoom level and top/right location
    /// the width is used to determine the largest dimension and ratio is used
    /// to decide how many pixels in the other dimension
    ///
    /// # Arguments
    /// * `zoom` - zoom level (1 is the default)
    /// * `top` - top location (-2 is the default)
    /// * `left` - top left location (-1.5 is the default)
    /// * `bot` - bot location (2 is the default)
    /// * `right` - bot right location (1.5 is the default)
    /// * `width` - pixel width for biggest dimension
    ///
    /// # Returns
    /// This returns a list that contains all the Z values in
    /// the set to generate the julia set against
    pub fn generate_input_set(
        top: f64,
        left: f64,
        bot: f64,
        right: f64,
        width: u64,
    ) -> Vec<Complex<f64>> {
        let mut output: Vec<Complex<f64>> = Vec::new();
        let mut real: f64 = top;
        let mut imag: f64 = left;
        let ratio: f64 = (top.abs() + bot.abs()) / (left.abs() + right.abs());

        let real_step: f64 = (top.abs() + bot.abs()) / width as f64;
        let imag_step: f64 = (left.abs() + right.abs()) / (width as f64 / ratio);

        println!("Ratio: {}", ratio);
        println!("Real Step: {}", real_step);
        println!("Imag Step: {}", imag_step);

        for x in 0..width {
            let mut row: Vec<Complex<f64>> = Vec::new();
            real += real_step;
            for y in 0..(width as f64 / ratio) as u64 {
                row.push(Complex::new(real, imag));
                imag += imag_step;
            }
            imag = left;
            output.extend(row);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::julia;

    #[test]
    fn test_gen() {
        let output = julia::generate_input_set(
            -2.0, -2.0, 2.0, 2.0, 10);
        println!("Output: {:?}", output);
    }
}
