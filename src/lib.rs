#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

/// Errors returned by the Wine-bottle manager.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum BottleError {
    #[error("bottle name cannot be empty")]
    EmptyName,

    #[error("bottle '{0}' already exists")]
    DuplicateBottle(String),

    #[error("bottle '{0}' not found")]
    BottleNotFound(String),

    #[error("invalid prefix path '{0}'")]
    InvalidPrefixPath(String),

    #[error("JSON serialization error: {0}")]
    JsonError(String),
}

/// Bottle environment preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BottleEnvironment {
    Gaming,
    Application,
    Custom,
}

/// Windows architecture for the Wine prefix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WineArch {
    Win32,
    Win64,
}

/// A managed Wine bottle configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BottleConfig {
    pub name: String,
    pub environment: BottleEnvironment,
    pub arch: WineArch,
    pub runner: String,
    pub prefix_path: String,
    pub dxvk_enabled: bool,
    pub esync_enabled: bool,
    pub env_vars: BTreeMap<String, String>,
}

impl BottleConfig {
    pub fn new(
        name: impl Into<String>,
        environment: BottleEnvironment,
        prefix_path: impl Into<String>,
    ) -> Result<Self, BottleError> {
        let name = name.into();
        let prefix_path = prefix_path.into();
        if name.trim().is_empty() {
            return Err(BottleError::EmptyName);
        }
        if prefix_path.trim().is_empty() {
            return Err(BottleError::InvalidPrefixPath(prefix_path));
        }
        let (dxvk_enabled, esync_enabled) = match environment {
            BottleEnvironment::Gaming => (true, true),
            BottleEnvironment::Application => (false, true),
            BottleEnvironment::Custom => (false, false),
        };
        Ok(Self {
            name,
            environment,
            arch: WineArch::Win64,
            runner: "soda-9.0".to_string(),
            prefix_path,
            dxvk_enabled,
            esync_enabled,
            env_vars: BTreeMap::new(),
        })
    }

    /// Computes the effective environment variables for launching an executable in this bottle.
    pub fn effective_env(&self) -> BTreeMap<String, String> {
        let mut env = self.env_vars.clone();
        env.insert("WINEPREFIX".to_string(), self.prefix_path.clone());
        env.insert(
            "WINEARCH".to_string(),
            match self.arch {
                WineArch::Win32 => "win32".to_string(),
                WineArch::Win64 => "win64".to_string(),
            },
        );
        if self.esync_enabled {
            env.insert("WINEESYNC".to_string(), "1".to_string());
        }
        if self.dxvk_enabled {
            env.insert("DXVK_HUD".to_string(), "compiler".to_string());
        }
        env
    }
}

/// Registry of Wine bottles.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BottleManager {
    bottles: BTreeMap<String, BottleConfig>,
}

impl BottleManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_bottle(&mut self, bottle: BottleConfig) -> Result<(), BottleError> {
        if self.bottles.contains_key(&bottle.name) {
            return Err(BottleError::DuplicateBottle(bottle.name));
        }
        self.bottles.insert(bottle.name.clone(), bottle);
        Ok(())
    }

    pub fn get_bottle(&self, name: &str) -> Result<&BottleConfig, BottleError> {
        self.bottles
            .get(name)
            .ok_or_else(|| BottleError::BottleNotFound(name.to_string()))
    }

    pub fn list_names(&self) -> Vec<&str> {
        self.bottles.keys().map(String::as_str).collect()
    }

    pub fn to_json(&self) -> Result<String, BottleError> {
        serde_json::to_string_pretty(self).map_err(|e| BottleError::JsonError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gaming_bottle_effective_env_aaa() {
        // Arrange
        let bottle = BottleConfig::new("Steam", BottleEnvironment::Gaming, "/var/bottles/steam").unwrap();

        // Act
        let env = bottle.effective_env();

        // Assert
        assert_eq!(env.get("WINEPREFIX").map(String::as_str), Some("/var/bottles/steam"));
        assert_eq!(env.get("WINEARCH").map(String::as_str), Some("win64"));
        assert_eq!(env.get("WINEESYNC").map(String::as_str), Some("1"));
        assert!(bottle.dxvk_enabled);
    }

    #[test]
    fn test_duplicate_bottle_rejected_aaa() {
        // Arrange
        let mut mgr = BottleManager::new();
        mgr.create_bottle(BottleConfig::new("Office", BottleEnvironment::Application, "/var/bottles/office").unwrap())
            .unwrap();

        // Act
        let res = mgr.create_bottle(BottleConfig::new("Office", BottleEnvironment::Custom, "/var/bottles/office2").unwrap());

        // Assert
        assert_eq!(res, Err(BottleError::DuplicateBottle("Office".to_string())));
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        // Arrange / Act
        let res = BottleConfig::new("  ", BottleEnvironment::Custom, "/var/bottles/x");

        // Assert
        assert_eq!(res, Err(BottleError::EmptyName));
    }
}
