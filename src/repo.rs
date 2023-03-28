pub struct Repo<A> {
  pub records: Vec<A>,
}

pub trait HasID {
  fn id(&self) -> usize;
}

impl<A> Repo<A>
where
  A: Clone + HasID,
{
  pub const fn new() -> Repo<A> { Repo { records: Vec::new() } }

  pub fn first(&self) -> Option<A> { self.records.first().cloned() }

  pub fn push(&mut self, record: A) { self.records.push(record); }

  pub fn find(&self, id: usize) -> Option<A> {
    self.records.iter().find(|record| record.id() == id).cloned()
  }

  pub fn _last(&self) -> Option<A> { self.records.last().cloned() }

  pub fn next_id(&self) -> usize {
    self
      .records
      .iter()
      .map(|record| record.id())
      .max()
      .map(|id| id + 1)
      .unwrap_or(1)
  }
}
