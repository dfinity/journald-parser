use crate::entry::Entry;

#[derive(PartialEq, Debug)]
pub struct Batch {
    pub entries: Vec<Entry>,
}

impl Batch {
    pub fn new() -> Batch {
        Batch {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}
