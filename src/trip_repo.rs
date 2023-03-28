use std::sync::Mutex;

use crate::{repo::Repo, trip::Trip};

pub trait TripRepo {
  fn first(&self) -> Option<Trip>;
}

impl TripRepo for Mutex<Repo<Trip>> {
  fn first(&self) -> Option<Trip> { self.lock().unwrap().first() }
}
