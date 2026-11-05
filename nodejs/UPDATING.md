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

**Note:** `gtfs-realtime.mjs` is not a second pbjs build. `npm run buildProto:esm` generates it from `gtfs-realtime.js` with `scripts/build-esm-wrapper.js`: a thin ES module that re-exports the CommonJS build. `import` and `require` therefore share one module instance and one set of types (`types.d.ts`). Don't edit it by hand; rerun the build.

Update the version number in `package.json`.

Publish the package to NPM:

```
npm publish
```
