mod gpx_reader;
mod gpx_to_4d;
mod plotter;
mod kalman_filter;

use clap::Parser;
use gpx_reader::read_gpx;
use plotter::plot;
use std::path::PathBuf;
use kalman_filter::kalman_filter;

/// Visualize a GPX track and fit a curve via Kalman filter.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to input GPX file
    #[arg(short, long, default_value = "input/stadium_loops.gpx")]
    input_file: PathBuf,

    /// Path to output PNG file
    #[arg(short, long, default_value = "plot.png")]
    output_file: PathBuf,

    /// How much we trust the model (x, Δx, ΔΔx, y, Δy, ΔΔy)
    #[arg(short, long, default_value = "1")]
    model_uncertainty: f64,

    /// How much we trust the sensor (GPS)
    #[arg(short, long, default_value = "1")]
    sensor_uncertainty: f64,
}

fn main() {
    let args = Args::parse();

    let gpx_points = read_gpx(args.input_file);
    let points_4d = gpx_to_4d::convert(gpx_points);
    let filtered_points = kalman_filter(&points_4d, args.model_uncertainty, args.sensor_uncertainty);
    plot(points_4d, filtered_points, args.output_file);
}
