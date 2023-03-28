use crate::{trip_repo::TripRepo, Trip, TRIPS};

pub fn create_trip(seat_count: usize, stop_count: usize) {
  let id = TRIPS.next_id();
  TRIPS.lock().unwrap().push(Trip { id, seat_count, stop_count });
}

// tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_trip() {
    let seat_count: usize = 2;
    let stop_count: usize = 3;
    create_trip(seat_count, stop_count);
    let trip: Trip = TRIPS.lock().unwrap().first().unwrap();
    assert_eq!(trip.id, 1);
    assert_eq!(trip.seat_count, 2);
    assert_eq!(trip.stop_count, 3);
  }
}
