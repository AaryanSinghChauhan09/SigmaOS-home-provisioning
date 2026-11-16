// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-home-provisioning: home provisioning component model.
//!
//! Real implementation: provisionings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provisioning {
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
pub enum HomeProvisioningError {
    EmptyProvisioningName,
    DuplicateProvisioning(String),
    EmptyEventSubject,
}

impl fmt::Display for HomeProvisioningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProvisioningName => write!(f, "provisioning name must not be empty"),
            Self::DuplicateProvisioning(id) => write!(f, "duplicate provisioning {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for HomeProvisioningError {}

pub struct HomeProvisioning {
    pub name: String,
    provisionings: BTreeMap<String, Provisioning>,
    events: BTreeMap<String, Event>,
}

impl HomeProvisioning {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            provisionings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_provisioning(&mut self, item: Provisioning) -> Result<(), HomeProvisioningError> {
        if item.name.trim().is_empty() {
            return Err(HomeProvisioningError::EmptyProvisioningName);
        }
        let id = item.id.clone();
        if self.provisionings.contains_key(&id) {
            return Err(HomeProvisioningError::DuplicateProvisioning(id.clone()));
        }
        self.provisionings.insert(id, item);
        Ok(())
    }

    pub fn provisioning_count(&self) -> usize {
        self.provisionings.len()
    }

    pub fn get_provisioning(&self, id: &str) -> Option<&Provisioning> {
        self.provisionings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), HomeProvisioningError> {
        if event.subject.trim().is_empty() {
            return Err(HomeProvisioningError::EmptyEventSubject);
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

    fn sample_provisioning(id: &str, name: &str) -> Provisioning {
        Provisioning {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = HomeProvisioning::new("HomeProvisioning");
        assert_eq!(m.name, "HomeProvisioning");
        assert_eq!(m.provisioning_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_provisioning() {
        let mut m = HomeProvisioning::new("HomeProvisioning");
        m.add_provisioning(sample_provisioning("a1", "Alpha")).unwrap();
        assert_eq!(m.provisioning_count(), 1);
        assert!(m.get_provisioning("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = HomeProvisioning::new("HomeProvisioning");
        assert!(m.add_provisioning(sample_provisioning("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = HomeProvisioning::new("HomeProvisioning");
        m.add_provisioning(sample_provisioning("x", "One")).unwrap();
        assert!(m.add_provisioning(sample_provisioning("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = HomeProvisioning::new("HomeProvisioning");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
