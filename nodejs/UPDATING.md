# How-To Update Bindings When gtfs-realtime.proto Changes

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

Note that, to maintain compatability with CommonJS typescript projects, we need to manually change the `Long` type to the stub type included in the ProtobufJS library. 

Update the version number in `package.json`.

Publish the package to NPM:

```
npm publish
```
