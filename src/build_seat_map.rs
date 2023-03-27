use std::ops::Range;

use crate::{SeatMap, TicketsRepo, Trip, TripSalesRepo, Ticket, TripSale};

pub fn build_seat_map(
  trip: Trip,
  tickets_repo: TicketsRepo,
  trip_sales_repo: TripSalesRepo,
) -> SeatMap {
  let mut available: Vec<bool> = Vec::new();
  let mut bought: Vec<bool> = Vec::new();
  let mut reserved: Vec<bool> = Vec::new();
  let mut unavailable: Vec<bool> = Vec::new();
  let tickets: Vec<Ticket> = tickets_repo
    .tickets
    .into_iter()
    .filter(|ticket| ticket.trip_id == trip.id)
    .collect();

  let trip_sales: Vec<TripSale> = trip_sales_repo
    .trip_sales
    .into_iter()
    .filter(|trip_sale| trip_sale.trip_id == trip.id)
    .collect();

  (0..trip.seat_count * trip.stop_count).for_each(|_| {
    available.push(true);
    bought.push(false);
    reserved.push(false);
    unavailable.push(false);
  });

  tickets.iter().for_each(|ticket| {
    let range: Range<usize> = (ticket.seat * trip.stop_count + ticket.from_stop)
      ..(ticket.seat * trip.stop_count + ticket.to_stop);

    for i in range {
      available[i as usize] = false;
      bought[i as usize] = true;
      unavailable[i as usize] = true;
    }
  });

  trip_sales.iter().for_each(|trip_sale| {
    trip_sale.reserved.iter().enumerate().for_each(|(i, is_reserved)| {
      match *is_reserved {
        true => {
          available[i] = false;
          reserved[i] = true;
          unavailable[i] = true;
        }
        false => (),
      }
    });
  });

  SeatMap {
    seat_count: trip.seat_count,
    stop_count: trip.stop_count,
    available,
    bought,
    reserved,
    unavailable,
  }
}
