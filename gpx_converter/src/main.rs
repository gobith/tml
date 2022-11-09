use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::f64::consts::PI;

#[derive(Debug)]
struct TrackPoint {
    lat: f64,
    lon: f64,
}

#[derive(Debug)]
struct WayPoint {
    lat: f64,
    lon: f64,
    name: String,
}

const EARTH_RADIUS_IN_METERS: f64 = 6371000.0;






fn main() {
    let contents = fs::read_to_string("../website/public/TML-211-km-AU-certifieerde-2021-2025.gpx")
        .expect("Something went wrong reading the file");

    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);

 
    let mut buf = Vec::new();

    let mut track_points = vec![];
    let mut way_points = vec![];

    loop {
        match reader.read_event_into(&mut buf) {
            Err(error) => panic!(
                "Error at position {}: {:?}",
                reader.buffer_position(),
                error
            ),

            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"trkpt" => {
                    let mut lat = 0.0;
                    let mut lon = 0.0;

                    for attr in e.attributes() {
                        let attr = attr.unwrap();

                        let value = attr.unescape_value().unwrap().to_string();

                        match attr.key.as_ref() {
                            b"lat" => lat = value.parse().unwrap(),
                            b"lon" => lon = value.parse().unwrap(),
                            _ => (),
                        }
                    }

                    track_points.push(TrackPoint { lat, lon });
                }
                _ => (),
            },

            Ok(Event::Eof) => break,
            _ => (),
        }
    }

    // println!("points: {:?}", track_points);
    // println!("{}" , PI);

    // total distance of points
    let mut total_distance = 0.0;
    let mut meters = 1000.0;

    for i in 0..track_points.len() - 1 {
        let start_waypoint = &track_points[i];
        let end_waypoint = &track_points[i + 1];

        let distance = calculate_distance(start_waypoint, end_waypoint);

        total_distance += distance;

        if total_distance > meters {
            way_points.push(WayPoint {
                lat: end_waypoint.lat,
                lon: end_waypoint.lon,
                name: format!("{} km", meters / 1000.0),
            });
            println!("{} meters", meters);
            println!("{} km", meters / 1000.0);
            meters += 1000.0;

        };
    };
    println!("total distance: {}", total_distance);
    println!("waypoints: {:?}", way_points);


}



fn calculate_distance(start_waypoint: &TrackPoint , end_waypoint: &TrackPoint) -> f64 {
    let lat1 = start_waypoint.lat * PI / 180.0;
    let lon1 = start_waypoint.lon * PI / 180.0;
    let lat2 = end_waypoint.lat * PI / 180.0;
    let lon2 = end_waypoint.lon * PI / 180.0;

    let dlon = lon2 - lon1;
    let dlat = lat2 - lat1;

    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin() + lat1.cos() * lat2.cos() * (dlon / 2.0).sin() * (dlon / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_IN_METERS  * c
}