use crate::repo::HasID;

#[derive(Debug, Clone)]
pub struct Ticket {
  pub id: usize,
  pub trip_id: usize,
  pub user_id: usize,
  pub seat: usize,
  pub from_stop: usize,
  pub to_stop: usize,
}

impl HasID for Ticket {
  fn id(&self) -> usize { self.id }
}
