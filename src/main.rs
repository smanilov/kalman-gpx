mod gpx_reader;
mod gpx_to_4d;

use clap::Parser;
use gpx_reader::read_gpx;
use std::path::PathBuf;

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

    let points = read_gpx(args.filename);

    let points_4d = gpx_to_4d::convert(points);

    println!("{:?}", points_4d);
}
