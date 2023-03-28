use std::sync::Mutex;

use crate::{repo::Repo, trip::Trip};

pub trait TripRepo {
  fn first(&self) -> Option<Trip>;

  fn next_id(&self) -> usize;
}

impl TripRepo for Mutex<Repo<Trip>> {
  fn first(&self) -> Option<Trip> { self.lock().unwrap().first() }

  fn next_id(&self) -> usize { self.lock().unwrap().next_id() }
}
