use std::ops::Range;

use crate::{
  build_seat_map::{build_seat_map, SeatMap},
  SeatReservation, Trip, SEAT_RESERVATIONS, TRIPS, repo::MutexRepo,
};

pub fn reserve_seat(
  seat: usize,
  stops: (usize, usize),
  trip_id: usize,
  user_id: usize,
) -> Result<(), String> {
  let trip: Trip = TRIPS.lock().unwrap().find(trip_id).unwrap();
  let seat_map: SeatMap = build_seat_map(trip_id);
  let seat_count: usize = trip.seat_count;
  let stop_count: usize = trip.stop_count;
  let from_stop: usize = stops.0;
  let to_stop: usize = stops.1;
  let reserved = seat_request(seat_count, stop_count, seat, from_stop, to_stop);

  for (is_reserved, is_available) in
    reserved.iter().zip(seat_map.available.iter())
  {
    if let (true, false) = (is_reserved, is_available) {
      return Err(String::from("Seat is not available"));
    }
  }

  SEAT_RESERVATIONS.create(SeatReservation {
    id: 0,
    trip_id,
    user_id,
    seat,
    seat_count,
    stop_count,
    from_stop,
    to_stop,
  })?;
  Ok(())
}

fn seat_request(
  seat_count: usize,
  stop_count: usize,
  seat: usize,
  from_stop: usize,
  to_stop: usize,
) -> Vec<bool> {
  let mut seat_request: Vec<bool> = vec![false; seat_count * stop_count];

  let range: Range<usize> =
    (seat * stop_count + from_stop)..(seat * stop_count + to_stop);
  for i in range {
    seat_request[i] = true;
  }
  seat_request
}
