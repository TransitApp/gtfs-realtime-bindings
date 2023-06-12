use std::{fs, path::Path};

// Re-exports
pub use gtfsrt_bindings::*;
pub use prost;

pub trait JsonFormat {
    fn to_json_string(&self) -> String;
    fn to_json_string_pretty(&self) -> String;
    fn from_json(json: &str) -> Self;
    fn from_json_at_path(path: impl AsRef<Path>) -> Self;
}

impl<T: prost::Message + serde::Serialize + serde::de::DeserializeOwned> JsonFormat for T {
    fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_string_pretty(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }

    fn from_json_at_path(path: impl AsRef<Path>) -> Self {
        Self::from_json(&fs::read_to_string(path).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_json_diff::assert_json_eq;
    use prost::Message;
    use serde_json::Value as Json;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    const GENERATE_FIXTURES: bool = false;
    const FIXTURE_BASE: &str = "tests/fixtures/";

    fn fixture(path: &str) -> PathBuf {
        PathBuf::from(FIXTURE_BASE).join(path)
    }

    #[test]
    fn it_prints_extensions_correctly() {
        let input = fixture("input/stm_trip_updates_with_extensions.pb");
        let output = fixture("output/stm_trip_updates_with_extensions.json");

        let data = fs::read(input).unwrap();
        let message = FeedMessage::decode(&data[..]).unwrap();

        if GENERATE_FIXTURES {
            fs::write(&output, message.to_json_string_pretty()).unwrap();
        }

        let expected = fs::read_to_string(&output).unwrap();

        assert_json_eq!(
            Json::from_str(&message.to_json_string_pretty()).unwrap(),
            Json::from_str(&expected).unwrap()
        );
    }
}
