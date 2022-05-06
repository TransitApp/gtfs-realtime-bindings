use gtfs_realtime_bindings_transit::trip_update::StopTimeUpdate;
use gtfs_realtime_bindings_transit::*;

pub trait Filter {
    fn filter_stop(&mut self, stop_id: &str);
    fn filter_trip(&mut self, trip_id: &str);
    fn filter_no_stops(&mut self);
    fn filter_route(&mut self, route_id: &str);
}

impl Filter for FeedMessage {
    fn filter_trip(&mut self, trip_id: &str) {
        self.entity
            .retain(|e| matches!(get_trip_id(e), Some(id) if id == trip_id))
    }

    fn filter_stop(&mut self, stop_id: &str) {
        for entity in self.entity.iter_mut() {
            if let Some(tu) = entity.trip_update.as_mut() {
                tu.stop_time_update
                    .retain(|stu| matches!(get_stop_id(stu), Some(id) if id == stop_id))
            }
        }

        self.filter_no_stops()
    }

    /// Remove trip updates if they don't contain stops
    fn filter_no_stops(&mut self) {
        self.entity.retain(|e| {
            if let Some(tu) = e.trip_update.as_ref() {
                !tu.stop_time_update.is_empty()
            } else {
                true
            }
        })
    }
    fn filter_route(&mut self, route_id: &str) {
        self.entity
            .retain(|e| matches!(get_route_id(e), Some(id) if id == route_id))
    }
}

fn get_trip_descriptor(entity: &FeedEntity) -> Option<&TripDescriptor> {
    let trip_update_trip = entity.trip_update.as_ref().map(|tu| &tu.trip);
    let vehicle_trip = entity.vehicle.as_ref().and_then(|tu| tu.trip.as_ref());
    trip_update_trip.or(vehicle_trip)
}

fn get_trip_id(entity: &FeedEntity) -> Option<&str> {
    get_trip_descriptor(entity).and_then(|trip| trip.trip_id.as_deref())
}

fn get_route_id(entity: &FeedEntity) -> Option<&str> {
    get_trip_descriptor(entity).and_then(|trip| trip.route_id.as_deref())
}

fn get_stop_id(stop_time_update: &StopTimeUpdate) -> Option<&str> {
    stop_time_update.stop_id.as_deref()
}
