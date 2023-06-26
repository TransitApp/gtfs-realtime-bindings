# gtfsrt-bindings

Rust bindings to the gtfs-realtime protobuf spec

**Note**

Until https://github.com/tokio-rs/prost/pull/591 is merged which gives support for proto2 extensions, we include a subtree of the spec with extensions with modifications (https://github.com/tokio-rs/prost/issues/100#issuecomment-390266699)

We now check-in the generated code. This avoids downstream consumers having to have "protoc" installed on their system. Another option is to compile protoc from source everytime, using the protobuf-src crate. This works but adds unnecessary compile time.