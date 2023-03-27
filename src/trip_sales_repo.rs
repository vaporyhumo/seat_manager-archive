use crate::{TripSale, TripSaleInput, TripSalesRepo};

impl TripSalesRepo {
  pub fn new() -> Self { Self { trip_sales: vec![], last_id: 0 } }

  pub fn create_trip_sale(&self, input: TripSaleInput) -> Self {
    let id: usize = self.last_id + 1 as usize;
    let trip_sale: TripSale =
      TripSale { id, reserved: input.reserved, trip_id: input.trip_id };
    let mut trip_sales: Vec<TripSale> = self.trip_sales.clone();
    trip_sales.push(trip_sale);
    Self { trip_sales, last_id: id }
  }
}
