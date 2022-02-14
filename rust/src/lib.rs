pub use protobuf;

use protobuf::Message;
use std::{fs, path::Path};

include!(concat!(env!("OUT_DIR"), "/protobuf-codegen/mod.rs"));

pub trait JsonFormat {
    fn to_json_string(&self) -> String;
    fn to_json_string_pretty(&self) -> String;
    fn from_json(json: &str) -> Self;
    fn from_json_at_path(path: impl AsRef<Path>) -> Self;
}

impl<T: Message> JsonFormat for T {
    fn to_json_string(&self) -> String {
        protobuf::json::print_to_string(self).unwrap()
    }

    fn to_json_string_pretty(&self) -> String {
        let json = self.to_json_string();
        let parsed_json: serde_json::Value = serde_json::from_str(&json).unwrap();
        serde_json::to_string_pretty(&parsed_json).unwrap()
    }

    fn from_json(json: &str) -> Self {
        protobuf::json::parse_from_str(json).unwrap()
    }

    fn from_json_at_path(path: impl AsRef<Path>) -> Self {
        Self::from_json(&fs::read_to_string(path).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::gtfs_realtime::FeedMessage;

    use super::JsonFormat;
    use assert_json_diff::assert_json_eq;
    use protobuf::Message;
    use serde_json::Value as Json;
    use std::fs;
    use std::path::Path;
    use std::str::FromStr;

    const GENERATE_FIXTURES: bool = false;

    macro_rules! fixture {
        ($path:expr) => {
            Path::new(concat!("tests/fixtures/", $path))
        };
    }

    #[test]
    fn it_prints_extensions_correctly() {
        let data = fs::read(fixture!("input/stm_trip_updates_with_extensions.pb")).unwrap();
        let message = FeedMessage::parse_from_bytes(&data[..]).unwrap();

        let output_fixture = fixture!("output/stm_trip_updates_with_extensions.json");

        if GENERATE_FIXTURES {
            fs::write(output_fixture, message.to_json_string_pretty()).unwrap();
        }

        let expected = fs::read_to_string(output_fixture).unwrap();

        assert_json_eq!(
            Json::from_str(&message.to_json_string_pretty()).unwrap(),
            Json::from_str(&expected).unwrap()
        );
    }
}
