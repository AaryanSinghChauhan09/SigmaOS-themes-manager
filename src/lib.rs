// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mint-themes-manager: mint themes manager component model.
//!
//! Real implementation: themes, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
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
pub enum MintThemesManagerError {
    EmptyThemeName,
    DuplicateTheme(String),
    EmptyEventSubject,
}

impl fmt::Display for MintThemesManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyThemeName => write!(f, "theme name must not be empty"),
            Self::DuplicateTheme(id) => write!(f, "duplicate theme {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintThemesManagerError {}

pub struct MintThemesManager {
    pub name: String,
    themes: BTreeMap<String, Theme>,
    events: BTreeMap<String, Event>,
}

impl MintThemesManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            themes: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_theme(&mut self, item: Theme) -> Result<(), MintThemesManagerError> {
        if item.name.trim().is_empty() {
            return Err(MintThemesManagerError::EmptyThemeName);
        }
        let id = item.id.clone();
        if self.themes.contains_key(&id) {
            return Err(MintThemesManagerError::DuplicateTheme(id.clone()));
        }
        self.themes.insert(id, item);
        Ok(())
    }

    pub fn theme_count(&self) -> usize {
        self.themes.len()
    }

    pub fn get_theme(&self, id: &str) -> Option<&Theme> {
        self.themes.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintThemesManagerError> {
        if event.subject.trim().is_empty() {
            return Err(MintThemesManagerError::EmptyEventSubject);
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

    fn sample_theme(id: &str, name: &str) -> Theme {
        Theme {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintThemesManager::new("MintThemesManager");
        assert_eq!(m.name, "MintThemesManager");
        assert_eq!(m.theme_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_theme() {
        let mut m = MintThemesManager::new("MintThemesManager");
        m.add_theme(sample_theme("a1", "Alpha")).unwrap();
        assert_eq!(m.theme_count(), 1);
        assert!(m.get_theme("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintThemesManager::new("MintThemesManager");
        assert!(m.add_theme(sample_theme("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintThemesManager::new("MintThemesManager");
        m.add_theme(sample_theme("x", "One")).unwrap();
        assert!(m.add_theme(sample_theme("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintThemesManager::new("MintThemesManager");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
