// Mostly generated with ChatGPT.
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::BufReader;
use std::mem;
use std::path::PathBuf;
use xml::{
    attribute::OwnedAttribute,
    reader::{EventReader, Result, XmlEvent},
};

#[derive(Debug, PartialEq)]
pub struct GpxPoint {
    pub lat: f64,
    pub lon: f64,
    pub elevation: Option<f64>,
    pub time: Option<DateTime<Utc>>,
    pub heart_rate: Option<u32>,
    pub cadence: Option<u32>,
}

pub fn read_gpx(filename: PathBuf) -> Vec<GpxPoint> {
    let file = File::open(filename).expect("Unable to open file");
    let file = BufReader::new(file);
    GpxParser::new().parse_gpx(file)
}

struct GpxParser {
    current_point: GpxPoint,
    points: Vec<GpxPoint>,
    inside_trkpt: bool,
    inside_extensions: bool,
}

impl GpxParser {
    fn new() -> GpxParser {
        GpxParser {
            current_point: GpxPoint {
                lat: 0.0,
                lon: 0.0,
                elevation: None,
                time: None,
                heart_rate: None,
                cadence: None,
            },
            points: Vec::new(),
            inside_trkpt: false,
            inside_extensions: false,
        }
    }

    fn parse_gpx<R: std::io::Read>(mut self, file: BufReader<R>) -> Vec<GpxPoint> {
        let mut parser = EventReader::new(file);

        loop {
            let event = parser.next();
            if !self.handle_event(&mut parser, event) {
                break;
            }
        }

        self.points
    }

    fn handle_trkpt(&mut self, attributes: Vec<OwnedAttribute>) -> bool {
        self.inside_trkpt = true;
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "lat" => self.current_point.lat = attr.value.parse().unwrap(),
                "lon" => self.current_point.lon = attr.value.parse().unwrap(),
                _ => {}
            }
        }
        true
    }

    fn handle_event<R: std::io::Read>(
        &mut self,
        mut parser: &mut EventReader<R>,
        event: Result<XmlEvent>,
    ) -> bool {
        match event {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "trkpt" => self.handle_trkpt(attributes),
                "ele" => self.handle_ele(&mut parser),
                "time" => self.handle_time(&mut parser),
                "extensions" => self.handle_extensions(),
                "hr" => self.handle_hr(&mut parser),
                "cad" => self.handle_cad(&mut parser),
                _ => true,
            },
            Ok(XmlEvent::EndElement { name }) => match name.local_name.as_str() {
                "trkpt" => self.handle_end_trkpt(),
                "extensions" => self.handle_end_extensions(),
                _ => true,
            },
            Ok(XmlEvent::EndDocument) => false,
            Err(e) => {
                println!("Error: {}", e);
                false
            }
            _ => true,
        }
    }

    fn handle_end_trkpt(&mut self) -> bool {
        self.inside_trkpt = false;
        let mut new_point = GpxPoint {
            lat: 0.0,
            lon: 0.0,
            elevation: None,
            time: None,
            heart_rate: None,
            cadence: None,
        };
        mem::swap(&mut self.current_point, &mut new_point);
        self.points.push(new_point);
        true
    }

    fn handle_end_extensions(&mut self) -> bool {
        self.inside_extensions = false;
        true
    }

    fn handle_ele<R: std::io::Read>(&mut self, parser: &mut EventReader<R>) -> bool {
        if self.inside_trkpt {
            let event = parser.next();
            if let Ok(XmlEvent::Characters(elevation)) = &event {
                match elevation.parse() {
                    Ok(elevation) => self.current_point.elevation = Some(elevation),
                    Err(err) => {
                        println!("warning: could not parse elevation: {err}")
                    }
                };
                true
            } else {
                self.handle_event(parser, event)
            }
        } else {
            println!("warning: ele outside trkpt");
            true
        }
    }

    fn handle_time<R: std::io::Read>(&mut self, parser: &mut EventReader<R>) -> bool {
        if self.inside_trkpt {
            let event = parser.next();
            if let Ok(XmlEvent::Characters(time)) = &event {
                match time.parse::<DateTime<Utc>>() {
                    Ok(time) => self.current_point.time = Some(time),
                    Err(err) => {
                        println!("warning: could not parse datetime: {err}")
                    }
                }
                true
            } else {
                self.handle_event(parser, event)
            }
        } else {
            // note: this is ok: the trk can have a top-level time
            // println!("warning: time outside trkpt");
            true
        }
    }

    fn handle_extensions(&mut self) -> bool {
        if self.inside_trkpt {
            self.inside_extensions = true;
        }
        true
    }

    fn handle_hr<R: std::io::Read>(&mut self, parser: &mut EventReader<R>) -> bool {
        if self.inside_extensions {
            let event = parser.next();
            if let Ok(XmlEvent::Characters(hr)) = &event {
                match hr.parse() {
                    Ok(heart_rate) => self.current_point.heart_rate = Some(heart_rate),
                    Err(err) => {
                        println!("warning: could not parse heart_rate: {err}")
                    }
                };
                true
            } else {
                self.handle_event(parser, event)
            }
        } else {
            println!("warning: hr outside extensions");
            true
        }
    }

    fn handle_cad<R: std::io::Read>(&mut self, parser: &mut EventReader<R>) -> bool {
        if self.inside_extensions {
            let event = parser.next();
            if let Ok(XmlEvent::Characters(cadence)) = &event {
                match cadence.parse() {
                    Ok(cadence) => self.current_point.cadence = Some(cadence),
                    Err(err) => {
                        println!("warning: could not parse cadence: {err}")
                    }
                };
                true
            } else {
                self.handle_event(parser, event)
            }
        } else {
            println!("warning: cad outside extensions");
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_parse_gpx() {
        let gpx_data = r#"
        <gpx xmlns:gpxtpx="http://www.garmin.com/xmlschemas/TrackPointExtension/v1">
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
        let points = GpxParser::new().parse_gpx(reader);

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
