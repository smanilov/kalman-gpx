use plotters::prelude::*;

use crate::gpx_to_4d::Point4D;
use core::cmp::Ordering;
use std::path::PathBuf;

const EPSILON: f64 = 1e-12;

fn compare_floats(a: &f64, b: &f64) -> Ordering {
    if a - b < -EPSILON {
        Ordering::Less
    } else if b - a < -EPSILON {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn min_x(points: &Vec<Point4D>) -> f64 {
    points.iter().map(|p| p.x_m).min_by(compare_floats).unwrap()
}

fn max_x(points: &Vec<Point4D>) -> f64 {
    points.iter().map(|p| p.x_m).max_by(compare_floats).unwrap()
}

fn min_y(points: &Vec<Point4D>) -> f64 {
    points.iter().map(|p| p.y_m).min_by(compare_floats).unwrap()
}

fn max_y(points: &Vec<Point4D>) -> f64 {
    points.iter().map(|p| p.y_m).max_by(compare_floats).unwrap()
}

pub fn plot(orig_points: Vec<Point4D>, filtered_points: Vec<Point4D>, output_file: PathBuf) {
    let plot_width = 800;
    let plot_height = 600;
    let root = BitMapBackend::new(&output_file, (plot_width, plot_height)).into_drawing_area();
    let _ = root.fill(&BLACK).unwrap();

    let minx = min_x(&orig_points);
    let maxx = max_x(&orig_points);
    let miny = min_y(&orig_points);
    let maxy = max_y(&orig_points);

    let mut width = maxx - minx + 1f64;
    let mut height = maxy - miny + 1f64;

    let scale1 = (plot_width as f64) / width;
    let scale2 = (plot_height as f64) / height;

    let scale = if scale1 < scale2 { scale1 } else { scale2 };
    width = (plot_width as f64) / scale;
    height = (plot_height as f64) / scale;

    let left_plot_bound = (minx + maxx) / 2.0 - width * 0.6;
    let right_plot_bound = (minx + maxx) / 2.0 + width * 0.6;
    let bottom_plot_bound = (miny + maxy) / 2.0 - height * 0.6;
    let top_plot_bound = (miny + maxy) / 2.0 + height * 0.6;

    // Create a new chart context
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(
            left_plot_bound..right_plot_bound,
            bottom_plot_bound..top_plot_bound,
        )
        .unwrap();

    // Plot the orig_points
    let _ = chart
        .draw_series(LineSeries::new(
            orig_points.iter().map(|p| (p.x_m, p.y_m)),
            &RED,
        ))
        .unwrap();

    // Plot the orig_points
    let _ = chart
        .draw_series(LineSeries::new(
            filtered_points.iter().map(|p| (p.x_m, p.y_m)),
            &GREEN,
        ))
        .unwrap();
}
