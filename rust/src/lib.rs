include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/prebuilt/transit_realtime.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/prebuilt/transit_realtime.serde.rs"
));

// Re-export prost
pub use prost;
