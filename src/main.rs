mod build_seat_map;
mod database;
mod tickets_repo;
mod trip_sales_repo;
mod trips_repo;

use build_seat_map::build_seat_map;

#[derive(Debug, Clone)]
pub struct Trip {
  id: usize,
  seat_count: usize,
  stop_count: usize,
}

#[derive(Debug, Clone)]
pub struct TripInput {
  seat_count: usize,
  stop_count: usize,
}

#[derive(Debug, Clone)]
pub struct TripSale {
  id: usize,
  trip_id: usize,
  reserved: Vec<bool>,
}

#[derive(Debug, Clone)]
pub struct TripSaleInput {
  trip_id: usize,
  reserved: Vec<bool>,
}

#[derive(Debug, Clone)]
pub struct Ticket {
  id: usize,
  trip_id: usize,
  seat: usize,
  from_stop: usize,
  to_stop: usize,
}

#[derive(Debug, Clone)]
pub struct TicketInput {
  trip_id: usize,
  seat: usize,
  from_stop: usize,
  to_stop: usize,
}

#[derive(Debug, Clone)]
pub struct SeatMap {
  seat_count: usize,
  stop_count: usize,
  available: Vec<bool>,
  bought: Vec<bool>,
  reserved: Vec<bool>,
  unavailable: Vec<bool>,
}

#[derive(Debug, Clone)]
pub struct TicketsRepo {
  last_id: usize,
  tickets: Vec<Ticket>,
}

#[derive(Debug, Clone)]
pub struct TripsRepo {
  last_id: usize,
  trips: Vec<Trip>,
}

#[derive(Debug, Clone)]
pub struct TripSalesRepo {
  last_id: usize,
  trip_sales: Vec<TripSale>,
}

#[derive(Debug, Clone)]
pub struct DB {
  tickets_repo: TicketsRepo,
  trips_repo: TripsRepo,
  trip_sales_repo: TripSalesRepo,
}

// type BuildSeatMap = fn(Trip, TicketsRepo, TripSalesRepo) -> SeatMap;
// static BUILD_SEAT_MAP: BuildSeatMap = build_seat_map;
//
// type ReserveSeat = fn(TripSale, TripSalesRepo) -> TripSalesRepo;

fn main() {
  let mut database: DB = DB::new();
  database = database.create_trip(TripInput { seat_count: 2, stop_count: 3 });
  database = database.create_ticket(TicketInput {
    trip_id: 1,
    seat: 0,
    from_stop: 0,
    to_stop: 2,
  });
  database = database.create_ticket(TicketInput {
    trip_id: 1,
    seat: 1,
    from_stop: 0,
    to_stop: 1,
  });

  let trip: Trip = database.trips_repo.trips.first().unwrap().clone();

  database = database.create_trip_sale(TripSaleInput {
    reserved: vec![false, false, true, false, false, false],
    trip_id: trip.id,
  });


  let seat_map =
    build_seat_map(trip, database.tickets_repo, database.trip_sales_repo);
  println!("{:#?}", seat_map);
}
