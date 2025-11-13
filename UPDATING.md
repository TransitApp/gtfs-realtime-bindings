# How-To Update Bindings When gtfs-realtime.proto Changes

When
[gtfs-realtime.proto](https://developers.google.com/transit/gtfs-realtime/gtfs-realtime-proto)
is updated, the various languages bindings must be regenerated, packaged, and
deployed.

## Prerequisites

If you've just cloned the repository, initialize the git submodules first:

```bash
git submodule update --init --recursive
```

This is required because the bindings depend on the base GTFS-realtime proto files from the `gtfs-spec` submodule.

## Updating Process

First step is to copy the latest version of `gtfs-realtime.proto` into project.
Then, follow the instructions in each individual UPDATING.md file for each
language type. 

* [.NET](dotnet/UPDATING.md)
* [Java](java/UPDATING.md)
* [JavaScript / Node.js](nodejs/UPDATING.md)
* [PHP](php/UPDATING.md)
* [Python](python/UPDATING.md)
* [Ruby](ruby/UPDATING.md)
* [Golang](golang/UPDATING.md)
