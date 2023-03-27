use crate::{Ticket, TicketInput, TicketsRepo};

impl TicketsRepo {
  pub fn new() -> Self { Self { tickets: vec![], last_id: 0 } }

  pub fn create_ticket(&self, input: TicketInput) -> Self {
    let id: usize = self.last_id + 1 as usize;
    let ticket: Ticket = Ticket {
      id,
      trip_id: input.trip_id,
      seat: input.seat,
      from_stop: input.from_stop,
      to_stop: input.to_stop,
    };
    let mut tickets: Vec<Ticket> = self.tickets.clone();
    tickets.push(ticket);
    Self { tickets, last_id: id }
  }
}
