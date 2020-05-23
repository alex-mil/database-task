use crate::{date::Date, Command};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct Storage {
    table: BTreeMap<Date, BTreeSet<String>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            table: BTreeMap::default(),
        }
    }

    pub fn execute(&mut self, cmd: Command) -> ExecutionStatus {
        match cmd {
            Command::Add(date, payload) => {
                let record = self.table.entry(date);
                record.or_default().insert(payload.to_owned());
                ExecutionStatus::Ok
            }
            Command::Delete(date, opt) => {
                if let Some(payload) = opt {
                    match self.table.get_mut(&date) {
                        None => ExecutionStatus::NoDate,
                        Some(events) => {
                            if events.remove(&payload.to_string()) {
                                ExecutionStatus::Deleted(None)
                            } else {
                                ExecutionStatus::NoEvent
                            }
                        }
                    }
                } else {
                    match self.table.remove(&date) {
                        None => ExecutionStatus::NoDate,
                        Some(events) => ExecutionStatus::Deleted(Some(events.len())),
                    }
                }
            }
            Command::Find(date) => {
                let opt = self.table.get(&date);
                if let Some(events) = opt {
                    return ExecutionStatus::Found(&events)
                }
                ExecutionStatus::Ok
            }
            Command::Print => ExecutionStatus::Printed(&self.table),
        }
    }
}

pub enum ExecutionStatus<'a> {
    NoDate,
    NoEvent,
    Deleted(Option<usize>),
    Found(&'a BTreeSet<String>),
    Printed(&'a BTreeMap<Date, BTreeSet<String>>),
    Ok,
}
