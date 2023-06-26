use std::env;
use std::path::PathBuf;

fn main() {
    if env::var("COMPILE_GTFSRT_PROTOBUF").is_err() {
        // Skip compilation by default
        return;
    }

    let root = crate_root().join("gtfs-spec-no-extensions/gtfs-realtime/proto");

    let proto_files = vec![root.join("gtfs-realtime.proto")];

    // Tell cargo to recompile if any of these proto files are changed
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");

    prost_build::Config::new()
        // Save descriptors to file
        .file_descriptor_set_path(&descriptor_path)
        // Override prost-types with pbjson-types
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types")
        // Generate prost structs
        .compile_protos(&proto_files, &[root])
        .unwrap();

    let descriptor_set = std::fs::read(descriptor_path).unwrap();
    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)
        .unwrap()
        .build(&[".transit_realtime"])
        .unwrap();

    // Copy the generated files to the output directory
    std::fs::create_dir_all(crate_root().join("prebuilt")).unwrap();

    std::fs::copy(
        out_dir().join("transit_realtime.rs"),
        crate_root().join("prebuilt/transit_realtime.rs"),
    )
    .unwrap();

    std::fs::copy(
        out_dir().join("transit_realtime.serde.rs"),
        crate_root().join("prebuilt/transit_realtime.serde.rs"),
    )
    .unwrap();
}

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}
