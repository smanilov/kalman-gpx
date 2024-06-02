// Mostly generated with ChatGPT.
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug, PartialEq)]
pub struct GpxPoint {
    lat: f64,
    lon: f64,
    elevation: Option<f64>,
    time: Option<DateTime<Utc>>,
    heart_rate: Option<u32>,
    cadence: Option<u32>,
}

pub fn read_gpx(filename: &str) -> Vec<GpxPoint> {
    let file = File::open(filename).expect("Unable to open file");
    let file = BufReader::new(file);
    parse_gpx(file)
}

fn parse_gpx<R: std::io::Read>(file: BufReader<R>) -> Vec<GpxPoint> {
    let mut parser = EventReader::new(file);

    let mut points = Vec::new();
    let mut current_point = GpxPoint {
        lat: 0.0,
        lon: 0.0,
        elevation: None,
        time: None,
        heart_rate: None,
        cadence: None,
    };
    let mut inside_trkpt = false;
    let mut inside_extensions = false;

    loop {
        let event = parser.next();
        match event {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "trkpt" => {
                    inside_trkpt = true;
                    for attr in attributes {
                        match attr.name.local_name.as_str() {
                            "lat" => current_point.lat = attr.value.parse().unwrap(),
                            "lon" => current_point.lon = attr.value.parse().unwrap(),
                            _ => {}
                        }
                    }
                }
                "ele" => {
                    if inside_trkpt {
                        if let Ok(XmlEvent::Characters(elevation)) = parser.next() {
                            match elevation.parse() {
                                Ok(elevation) => current_point.elevation = Some(elevation),
                                Err(err) => println!("warning: could not parse elevation: {err}"),
                            };
                        }
                    }
                }
                "time" => {
                    if inside_trkpt {
                        if let Ok(XmlEvent::Characters(time)) = parser.next() {
                            match time.parse::<DateTime<Utc>>() {
                                Ok(time) => current_point.time = Some(time),
                                Err(err) => println!("warning: could not parse datetime: {err}"),
                            }
                        }
                    }
                }
                "extensions" => {
                    if inside_trkpt {
                        inside_extensions = true;
                    }
                }
                "gpxtpx:hr" => {
                    if inside_extensions {
                        if let Ok(XmlEvent::Characters(hr)) = parser.next() {
                            match hr.parse() {
                                Ok(heart_rate) => current_point.heart_rate = Some(heart_rate),
                                Err(err) => println!("warning: could not parse heart_rate: {err}"),
                            };
                        }
                    }
                }
                "gpxtpx:cad" => {
                    if inside_extensions {
                        if let Ok(XmlEvent::Characters(cadence)) = parser.next() {
                            match cadence.parse() {
                                Ok(cadence) => current_point.cadence = Some(cadence),
                                Err(err) => println!("warning: could not parse cadence: {err}"),
                            };
                        }
                    }
                }
                _ => {}
            },
            Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                "trkpt" => {
                    inside_trkpt = false;
                    points.push(current_point);
                    current_point = GpxPoint {
                        lat: 0.0,
                        lon: 0.0,
                        elevation: None,
                        time: None,
                        heart_rate: None,
                        cadence: None,
                    };
                }
                "extensions" => {
                    inside_extensions = false;
                }
                _ => {}
            },
            Ok(XmlEvent::EndDocument) => break,
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_parse_gpx() {
        let gpx_data = r#"
        <gpx>
            <trkpt lat="44.4705880" lon="26.1146640">
                <ele>76.1</ele>
                <time>2023-08-24T16:49:58Z</time>
                <extensions>
                    <gpxtpx:TrackPointExtension>
                        <gpxtpx:hr>93</gpxtpx:hr>
                        <gpxtpx:cad>49</gpxtpx:cad>
                    </gpxtpx:TrackPointExtension>
                </extensions>
            </trkpt>
        </gpx>
        "#;
        let cursor = Cursor::new(gpx_data);
        let reader = BufReader::new(cursor);
        let points = parse_gpx(reader);

        let expected_point = GpxPoint {
            lat: 44.4705880,
            lon: 26.1146640,
            elevation: Some(76.1),
            time: Some(Utc.with_ymd_and_hms(2023, 8, 24, 16, 49, 58).unwrap()),
            heart_rate: Some(93),
            cadence: Some(49),
        };

        assert_eq!(points.len(), 1);
        assert_eq!(points[0], expected_point);
    }
}
