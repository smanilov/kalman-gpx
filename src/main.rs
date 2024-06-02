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
    /// Path to GPX file
    #[arg(short, long)]
    filename: PathBuf,
}

fn main() {
    let args = Args::parse();

    let gpx_points = read_gpx(args.filename);
    let points_4d = gpx_to_4d::convert(gpx_points);
    let filtered_points = kalman_filter(&points_4d);
    plot(points_4d, filtered_points);
}
