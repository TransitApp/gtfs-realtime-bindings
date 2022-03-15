# gtfs-rt printer

# Install from git repo

    cargo install --git ssh://git@github.com/TransitApp/gtfs-realtime-bindings.git

# Install local version globally

    cargo install --path .

# Run

## From url

    rtx "https://api.transitapp.com/v3/real_time/feeds/1/gtfs_rt/trip_updates"

## From url with auth headers:

    rtx https://api.goswift.ly/real-time/lametro/gtfs-rt-trip-updates -h Authorization:8493984fakeauth8493894

## From path

    rtx path/to/tripupdates.pb

## Filter trip id (or route/stop)

    rtx path/to/tripupdates.pb --trip-id 4389222

## Print as json

    rtx path/to/tripupdates.pb --json

## From redis (example printing trip updates in gtfs-rt-fetcher redis)

1. Connect to redis
    
        kubectl port-forward svc/gtfs-rt-fetcher-redis 6377:6379

2. Pipe

        redis-cli -p 6377 -a <PASSWORD> GET cached:GTFS-rt:1:100:tripUpdates | rtx

