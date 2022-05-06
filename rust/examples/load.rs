use gtfs_realtime_bindings_transit::*;
use prost::*;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("tests/fixtures/input/stm_trip_updates_with_extensions.pb");
    let bytes = fs::read(path).unwrap();
    let message = FeedMessage::decode(&bytes[..]).unwrap();
    let json = serde_json::to_string_pretty(&message).unwrap();
    println!("{}", json);
}
