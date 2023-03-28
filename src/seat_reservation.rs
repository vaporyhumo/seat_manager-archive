use crate::repo::HasID;

#[derive(Debug, Clone)]
pub struct SeatReservation {
  pub id: usize,
  pub seat_count: usize,
  pub stop_count: usize,
  pub seat: usize,
  pub from_stop: usize,
  pub to_stop: usize,
  pub trip_id: usize,
  pub user_id: usize,
}

impl HasID for SeatReservation {
  fn id(&self) -> usize { self.id }

  fn with_id(self, id: usize) -> Self { SeatReservation { id, ..self } }
}
