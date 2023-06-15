include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
include!(concat!(env!("OUT_DIR"), "/transit_realtime.serde.rs"));

// Re-export prost
pub use prost;
