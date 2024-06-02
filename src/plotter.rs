use plotters::prelude::*;

use crate::gpx_to_4d::Point4D;

pub fn plot(orig_points: Vec<Point4D>, filtered_points: Vec<Point4D>) {
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    let _ = root.fill(&BLACK).unwrap();

    // Create a new chart context
    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(-100.0..100.0, -100.0..100.0)
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
