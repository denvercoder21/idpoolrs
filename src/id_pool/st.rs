use std::collections::BTreeSet;

use super::Id;
use super::IdPool;

pub struct IdPoolSt {
    new: Id,
    released: BTreeSet<Id>,
}

impl IdPoolSt {
    pub fn new() -> IdPoolSt {
        IdPoolSt {
            new: 0,
            released: BTreeSet::<u32>::new(),
        }
    }
}

impl IdPool for IdPoolSt {
    fn next(&mut self) -> Option<Id> {
        if !self.released.is_empty() {
            let first = *self.released.iter().next()?;
            return self.released.take(&first);
        }

        if self.new == u32::MAX {
            println!("[idpool] maximum number of objects reached");
            return None;
        }

        self.new += 1;
        Some(self.new)
    }

    fn release(&mut self, id: Id) {
        if id > self.new {
            println!("[idpool] id never issued");
            return;
        }

        self.released.insert(id);
    }

    fn reset(&mut self) {
        self.new = 0;
        self.released.clear();
    }
}
