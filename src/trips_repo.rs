use crate::{Trip, TripInput, TripsRepo};

impl TripsRepo {
  pub fn new() -> Self { Self { trips: vec![], last_id: 0 } }

  pub fn create_trip(&self, input: TripInput) -> Self {
    let id: usize = self.last_id + 1 as usize;
    let trip: Trip =
      Trip { id, seat_count: input.seat_count, stop_count: input.stop_count };
    let mut trips: Vec<Trip> = self.trips.clone();
    trips.push(trip);
    Self { trips, last_id: id }
  }
}
