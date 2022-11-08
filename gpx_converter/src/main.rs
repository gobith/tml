use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;

#[derive(Debug)]
struct TrackPoint {
    lat: f64,
    lon: f64,
}

fn main() {
    let contents = fs::read_to_string("../website/public/TML-5-km-AU-certifieerde-2021-2025.gpx")
        .expect("Something went wrong reading the file");

    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);

    let mut count = 0;
    let mut buf = Vec::new();
    let mut txt = Vec::new();

    let mut points = vec![];

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

                        let value = attr.unescape_value().unwrap().to_string() ;

                        match attr.key.as_ref() {
                            b"lat" => lat = value.parse().unwrap(),
                            b"lon" => lon = value.parse().unwrap(),
                            _ => (),
                        }
                    }

                    points.push(TrackPoint{lat,lon});
                }
                _ => (),
            },

            Ok(Event::Text(e)) => {
                println!("{:?}", e);
                txt.push(e.unescape().unwrap().into_owned())
            }

            Ok(Event::Eof) => break,
            _ => (),
        }
    }

    // println!("{}", contents);
    // println!("count: {}", count);
    // println!("txt: {:?}", txt);
    println!("points: {:?}", points);
}
