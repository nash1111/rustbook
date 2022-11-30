fn get_n_diverged(x0: f64, y0: f64, max_iter: usize) -> u8 {
    // z = xn+yn
    let mut xn = 0.0;
    let mut yn = 0.0;
    for i in 1..max_iter {
        let xnext = xn * xn - yn * yn + x0;
        let ynext = 2.0 * xn * yn + y0;
        xn = xnext;
        yn = ynext;
        if xn * xn + yn * yn > 4.0 {
            return i as u8;
        }
    }
    max_iter as u8
}

pub fn generate_mandlbrot_set(
    canvas_x: usize,
    canvas_y: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    max_iter: usize
) -> Vec<u8> {}