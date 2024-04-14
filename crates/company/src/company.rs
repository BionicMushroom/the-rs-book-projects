pub mod departments;
pub mod employees;

use std::collections::btree_map::Entry;

#[derive(Clone, Debug, Default)]
pub struct Database {
    storage: departments::Map,
}

impl Database {
    pub fn new() -> Self {
        Database {
            storage: departments::Map::new(),
        }
    }

    pub fn add_employee(&mut self, name: &str, department: &str) -> bool {
        self.storage
            .entry(department.into())
            .or_default()
            .insert(name.into())
    }

    pub fn remove_employee(&mut self, name: &str, department: &str) -> bool {
        match self.storage.entry(department.into()) {
            Entry::Vacant(_) => false,
            Entry::Occupied(mut employee_set_entry) => {
                if employee_set_entry.get_mut().remove(name) {
                    if employee_set_entry.get_mut().is_empty() {
                        employee_set_entry.remove();
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn employees(&self, department: &str) -> employees::Iter {
        match self.storage.get(department) {
            None => employees::Iter::default(),
            Some(employees_set) => employees::Iter::from_set_iter(employees_set.iter()),
        }
    }

    pub fn departments(&self) -> departments::Iter {
        departments::Iter::from_map_iter(self.storage.iter())
    }
}
