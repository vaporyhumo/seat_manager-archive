use crate::{Trip, TRIPS};

pub struct CreateTripInput {
  pub seat_count: usize,
  pub stop_count: usize,
}

pub fn create_trip(input: CreateTripInput) {
  TRIPS.lock().unwrap().push(Trip {
    id: 1,
    seat_count: input.seat_count,
    stop_count: input.stop_count,
  });
}
