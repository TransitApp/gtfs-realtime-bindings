# gtfsrt-bindings

Rust bindings to the gtfs-realtime protobuf spec

# Updating bindings

Then pull in new changes in the git subtree from the official gtfs-rt spec, run the following in the repo root:

        git subtree pull --prefix rust/gtfs-spec-no-extensions git@github.com:google/transit.git master --squash
        

and run code generation again by running:

        COMPILE_GTFSRT_PROTOBUF=true cargo build

Then check-in the new generated code.

**Note**

Until https://github.com/tokio-rs/prost/pull/591 is merged which gives support for proto2 extensions, we include a subtree of the spec with extensions with modifications (https://github.com/tokio-rs/prost/issues/100#issuecomment-390266699)

We now check-in the generated code. This avoids downstream consumers having to have "protoc" installed on their system. Another option is to compile protoc from source everytime, using the protobuf-src crate. This works but adds unnecessary compile time.