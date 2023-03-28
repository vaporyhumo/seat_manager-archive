use std::sync::Mutex;

use crate::{repo::Repo, ticket::Ticket};

pub trait TicketRepo {
  fn next_id(&self) -> usize;
  fn create(&self, ticket: Ticket) -> Result<(), String>;
}

impl TicketRepo for Mutex<Repo<Ticket>> {
  fn next_id(&self) -> usize {
    let repo: &Repo<Ticket> = &self.lock().unwrap();
    repo.next_id()
  }

  fn create(&self, ticket: Ticket) -> Result<(), String> {
    self.lock().unwrap().push(ticket);
    Ok(())
  }
}
