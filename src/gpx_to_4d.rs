use crate::gpx_reader::GpxPoint;

use chrono::{DateTime, Utc};
use geoconv::{Degrees, Meters, Wgs84, LLE};

fn split_to_lle_time(gpx_point: GpxPoint) -> (LLE<Wgs84>, DateTime<Utc>) {
    (
        LLE::<Wgs84>::new(
            Degrees::new(gpx_point.lat),
            Degrees::new(gpx_point.lon),
            Meters::new(gpx_point.elevation.unwrap()),
        ),
        gpx_point.time.unwrap(),
    )
}

#[derive(Debug)]
pub struct Point4D {
    pub x_m: f64,
    pub y_m: f64,
    #[allow(dead_code)]
    pub z_m: f64,
    #[allow(dead_code)]
    pub secs: f64,
}

impl Point4D {
    fn origin() -> Point4D {
        Point4D {
            x_m: 0f64,
            y_m: 0f64,
            z_m: 0f64,
            secs: 0f64,
        }
    }
}

fn convert_relative(
    first_point_lle: &LLE<Wgs84>,
    first_time: &DateTime<Utc>,
    point: GpxPoint,
) -> Point4D {
    let (other, time) = split_to_lle_time(point);

    let time_diff = time.signed_duration_since(first_time);
    let enu_point = first_point_lle.enu_to(&other);
    Point4D {
        x_m: enu_point.east.as_float(),
        y_m: enu_point.north.as_float(),
        z_m: enu_point.up.as_float(),
        secs: time_diff.to_std().unwrap().as_secs_f64(),
    }
}

pub fn convert(points: Vec<GpxPoint>) -> Vec<Point4D> {
    let mut result = Vec::new();
    let mut is_first = true;
    let mut first_point_lle: Option<LLE<Wgs84>> = None;
    let mut first_time: Option<DateTime<Utc>> = None;
    for point in points {
        if is_first {
            let (lle, time) = split_to_lle_time(point);
            first_point_lle = Some(lle);
            first_time = Some(time);
            is_first = false;
            result.push(Point4D::origin());
            continue;
        }
        result.push(convert_relative(
            &first_point_lle.unwrap(),
            &first_time.unwrap(),
            point,
        ));
    }
    result
}
