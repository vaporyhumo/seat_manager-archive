mod build_seat_map;
mod create_ticket;
mod create_trip;
mod repo;
mod reserve_seat;
mod seat_reservation;
mod ticket;
mod ticket_repo;
mod trip;
mod trip_repo;

use {
  build_seat_map::SeatMap,
  create_ticket::create_ticket,
  create_trip::{create_trip, CreateTripInput},
  repo::Repo,
  reserve_seat::reserve_seat,
  seat_reservation::SeatReservation,
  std::sync::Mutex,
  ticket::Ticket,
  trip::Trip,
  trip_repo::TripRepo,
};

type Srr = Mutex<Repo<SeatReservation>>;
static SEAT_RESERVATIONS: Srr = Mutex::new(Repo::<SeatReservation>::new());
static TRIPS: Mutex<Repo<Trip>> = Mutex::new(Repo::<Trip>::new());
static TICKETS: Mutex<Repo<Ticket>> = Mutex::new(Repo::<Ticket>::new());

fn main() -> Result<(), String> {
  create_trip(CreateTripInput { seat_count: 2, stop_count: 3 });
  let trip: Trip = TRIPS.first().unwrap();
  let trip_id: usize = trip.id;
  let seat: usize = 0;
  let stops: (usize, usize) = (0, 2);
  let user_id: usize = 1;
  reserve_seat(seat, stops, trip_id, user_id)?;
  create_ticket(seat, stops, trip_id, user_id)?;

  let seat_map: SeatMap = build_seat_map::build_seat_map(trip.id);
  println!("{:#?}", seat_map);
  Ok(())
}
