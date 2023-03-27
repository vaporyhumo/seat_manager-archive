use crate::{
  TicketInput, TicketsRepo, TripInput, TripSalesRepo, TripsRepo, DB, TripSaleInput,
};

impl DB {
  pub fn new() -> Self {
    Self {
      tickets_repo: TicketsRepo::new(),
      trips_repo: TripsRepo::new(),
      trip_sales_repo: TripSalesRepo::new(),
    }
  }

  pub fn create_trip(&self, trip: TripInput) -> Self {
    let db = self.clone();
    let trips_repo = self.trips_repo.create_trip(trip);
    Self { trips_repo, ..db }
  }

  pub fn create_ticket(&self, ticket: TicketInput) -> Self {
    let db = self.clone();
    let tickets_repo = self.tickets_repo.create_ticket(ticket);
    Self { tickets_repo, ..db }
  }

  pub fn create_trip_sale(&self, trip_sale: TripSaleInput) -> Self {
    let db = self.clone();
    let trip_sales_repo = self.trip_sales_repo.create_trip_sale(trip_sale);
    Self { trip_sales_repo, ..db }
  }
}
