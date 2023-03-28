use crate::repo::HasID;

#[derive(Clone, Debug)]
pub struct Trip {
  pub id: usize,
  pub seat_count: usize,
  pub stop_count: usize,
}

impl HasID for Trip {
  fn id(&self) -> usize { self.id }
}
