// The ES module entry must expose the same objects as the CommonJS build:
// named and default imports, one shared instance, and working decoding.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createRequire } from 'node:module';

import bindings, { transit_realtime } from 'gtfs-realtime-bindings-transit';

const require = createRequire(import.meta.url);
const cjs = require('gtfs-realtime-bindings-transit');

assert.equal(typeof transit_realtime.FeedMessage.decode, 'function', 'named import');
assert.equal(bindings.transit_realtime, transit_realtime, 'default import');
assert.equal(cjs.transit_realtime, transit_realtime, 'import and require share one instance');

const feed = transit_realtime.FeedMessage.decode(
  readFileSync(new URL('./vehicle_position.pb', import.meta.url)),
);
assert.equal(feed.entity.length, 1);
assert.equal(feed.entity[0].id, '1');

console.log('ESM entry: ok');
