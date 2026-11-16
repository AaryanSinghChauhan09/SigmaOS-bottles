// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-bottles: bottles component model.
//!
//! Real implementation: bottles_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BottlesItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BottlesError {
    EmptyBottlesItemName,
    DuplicateBottlesItem(String),
    EmptyEventSubject,
}

impl fmt::Display for BottlesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBottlesItemName => write!(f, "bottles_item name must not be empty"),
            Self::DuplicateBottlesItem(id) => write!(f, "duplicate bottles_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BottlesError {}

pub struct Bottles {
    pub name: String,
    bottles_items: BTreeMap<String, BottlesItem>,
    events: BTreeMap<String, Event>,
}

impl Bottles {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            bottles_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_bottles_item(&mut self, item: BottlesItem) -> Result<(), BottlesError> {
        if item.name.trim().is_empty() {
            return Err(BottlesError::EmptyBottlesItemName);
        }
        let id = item.id.clone();
        if self.bottles_items.contains_key(&id) {
            return Err(BottlesError::DuplicateBottlesItem(id.clone()));
        }
        self.bottles_items.insert(id, item);
        Ok(())
    }

    pub fn bottles_item_count(&self) -> usize {
        self.bottles_items.len()
    }

    pub fn get_bottles_item(&self, id: &str) -> Option<&BottlesItem> {
        self.bottles_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BottlesError> {
        if event.subject.trim().is_empty() {
            return Err(BottlesError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bottles_item(id: &str, name: &str) -> BottlesItem {
        BottlesItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Bottles::new("Bottles");
        assert_eq!(m.name, "Bottles");
        assert_eq!(m.bottles_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_bottles_item() {
        let mut m = Bottles::new("Bottles");
        m.add_bottles_item(sample_bottles_item("a1", "Alpha")).unwrap();
        assert_eq!(m.bottles_item_count(), 1);
        assert!(m.get_bottles_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Bottles::new("Bottles");
        assert!(m.add_bottles_item(sample_bottles_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Bottles::new("Bottles");
        m.add_bottles_item(sample_bottles_item("x", "One")).unwrap();
        assert!(m.add_bottles_item(sample_bottles_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Bottles::new("Bottles");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
