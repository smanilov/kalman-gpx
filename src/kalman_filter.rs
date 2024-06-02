use std::ops::Mul;

use crate::gpx_to_4d::Point4D;
use nalgebra::{base::{Matrix2, Matrix2x6, Matrix6, Matrix6x1}, Matrix2x1};

pub fn kalman_filter(points: &Vec<Point4D>, model_uncertainty: f64, sensor_uncertainty: f64) -> Vec<Point4D> {
    // The math comes from a piece of paper. This is all we know.
    let delta_t = 1f64;

    #[rustfmt::skip]
    let f = Matrix6::<f64>::new(
        1f64, delta_t, 0.5 * delta_t * delta_t, 0f64, 0f64, 0f64,
        0f64, 1f64, delta_t, 0f64, 0f64, 0f64,
        0f64, 0f64, 1f64, 0f64, 0f64, 0f64,
        0f64, 0f64, 0f64, 1f64, delta_t, 0.5 * delta_t * delta_t,
        0f64, 0f64, 0f64, 0f64, 1f64, delta_t,
        0f64, 0f64, 0f64, 0f64, 0f64, 1f64,
    );
    let f_t = f.transpose();

    #[rustfmt::skip]
    let h = Matrix2x6::<f64>::new(
        1f64, 0f64, 0f64, 0f64, 0f64, 0f64,
        0f64, 0f64, 0f64, 1f64, 0f64, 0f64,
    );
    let h_t = h.transpose();

    // Uncertainty in model.
    let sigma_a = model_uncertainty;

    #[rustfmt::skip]
    let q_a = Matrix6::<f64>::new(
        0f64, 0f64, 0f64, 0f64, 0f64, 0f64,
        0f64, 0f64, 0f64, 0f64, 0f64, 0f64,
        0f64, 0f64, 1f64, 0f64, 0f64, 0f64,
        0f64, 0f64, 0f64, 0f64, 0f64, 0f64,
        0f64, 0f64, 0f64, 0f64, 0f64, 0f64,
        0f64, 0f64, 0f64, 0f64, 0f64, 1f64,
    ).mul(sigma_a * sigma_a);

    let q = f.mul(q_a.mul(f_t));

    // Uncertainty in sensor.
    let sigma_x = sensor_uncertainty;
    let sigma_y = sensor_uncertainty;

    #[rustfmt::skip]
    let r = Matrix2::<f64>::new(
        sigma_x * sigma_x, 0f64,
        0f64, sigma_y * sigma_y, 
    );

    #[rustfmt::skip]
    let mut x_hat = Matrix6x1::<f64>::new(
        0f64, 0f64, 0f64, 0f64, 0f64, 0f64,
    );

    let i6 = Matrix6::<f64>::identity();
    let mut p = i6 * 1000f64;

    let mut result = Vec::new();
    for point in points {
        let x_pred = f.mul(x_hat);
        let p_pred = f * p * f_t + q;
        let temp = h * p_pred * h_t + r;
        let k = p_pred * h_t * temp.pseudo_inverse(1e-6f64).unwrap();

        let z = Matrix2x1::<f64>::new(point.x_m, point.y_m);
        x_hat = x_pred + k * (z - h * x_pred);
        let temp2 = i6 - k * h;
        p = temp2 * p_pred * temp2.transpose() + k * r * k.transpose();
        result.push(Point4D {x_m: x_hat.x, y_m: x_hat.w, z_m: 0f64, secs: 0f64});
    }

    result
}
