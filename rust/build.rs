use std::env;
use std::fs;
use std::path::Path;

use protobuf_codegen::Codegen;

const PROTO_PATH: &str = "gtfs-spec-no-extensions/gtfs-realtime/proto/gtfs-realtime.proto";

fn main() {
    println!("cargo:rerun-if-changed={PROTO_PATH}");

    let out_dir = env::var("OUT_DIR").unwrap();
    let generated_dir = format!("{}/protobuf-codegen", out_dir);

    if Path::new(&generated_dir).exists() {
        fs::remove_dir_all(&generated_dir).unwrap();
    }

    fs::create_dir(&generated_dir).unwrap();

    Codegen::new()
        .pure()
        .out_dir(generated_dir)
        .inputs(&[PROTO_PATH])
        .include(".")
        .run()
        .expect("failed to build protobuf")
}
