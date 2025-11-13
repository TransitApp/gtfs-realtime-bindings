# How-To Update Bindings When gtfs-realtime.proto Changes

## Prerequisites

If you've just cloned the repository, initialize the git submodules first:

```
git submodule update --init --recursive
```

This is required because the build process depends on the base GTFS-realtime proto files from the `gtfs-spec` submodule.

## Building

Regenerate the language binding source from gtfs-realtime.proto.

```
npm run buildProto
```

You might need to optionally `npm install` to install the protobuf package
before building the proto.

Add the license header back to the generated source file.

Test the generated code:

```
npm run test
```

**Note:** The build script automatically replaces `import Long = require("long")` with `import { Long } from "protobufjs"` in the TypeScript definition files. This maintains compatibility with CommonJS TypeScript projects by using the Long stub type included in the ProtobufJS library instead of requiring a separate "long" package. 

Update the version number in `package.json`.

Publish the package to NPM:

```
npm publish
```
