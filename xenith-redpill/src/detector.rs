/*
Xenith - Xen-based security hypervisor
Copyright (C) 2025-2026 Xenith contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! Detector module
//!
//! This module provides the core functionality for detecting the presence of the Xen hypervisor.
//!
//! A singleton global technique registry (see [`TECHNIQUE_REGISTRY`]) is used to store all registered techniques. Techniques are
//! represented by the [`Technique`] trait, which contains a name, a description, and an execute function.
//!
//! The [`TechniqueRegistry`] struct is used to store a list of techniques and provides functions to register and run techniques.
//!
//! # Example
//!
//! To-do

use std::error::Error;
use std::fmt::Debug;
use std::sync::Mutex;

use log::debug;
use once_cell::sync::Lazy;
use thiserror::Error;

/// Singleton global technique registry, used to store all registered techniques
static TECHNIQUE_REGISTRY: Lazy<Mutex<TechniqueRegistry>> =
    Lazy::new(|| Mutex::new(TechniqueRegistry::new()));

/// The result of a detection technique
pub type TechniqueResult = Result<DetectionResult, TechniqueError>;

/// Detection result
///
/// This enum represents the result of a detection technique. It can be either detected or not detected.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DetectionResult {
    Detected,
    NotDetected,
}

/// Error type for techniques
///
/// This error type is used to represent errors that can occur when running a technique.
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TechniqueError {
    #[error("Technique failed")]
    Failed(),
    #[error("Technique not implemented")]
    NotImplemented,
    #[error("Unknown error")]
    Unknown,
}

/// A redpill technique
/// This trait represents a redpill technique that can be used to detect the presence of the Xen hypervisor.
/// It contains a name, a description, and an execute function.
///
/// # Example
///
/// To-do
pub trait Technique: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn execute(&self) -> TechniqueResult;
}

impl Debug for dyn Technique {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Technique")
            .field("name", &self.name())
            .field("description", &self.description())
            .finish()
    }
}

/// A registry of techniques
#[derive(Debug, Default)]
pub struct TechniqueRegistry {
    techniques: Vec<Box<dyn Technique>>,
}

impl TechniqueRegistry {
    /// Create a new technique registry
    ///
    /// This function creates a new technique registry with an empty list of techniques.
    pub fn new() -> Self {
        let techniques = Vec::new();
        TechniqueRegistry { techniques }
    }

    /// Register a technique with the registry
    ///
    /// This function registers a new technique with the registry. If the technique is already registered, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `technique` - The technique to register
    ///
    /// # Returns
    ///
    /// A result indicating success or failure
    ///
    /// # Errors
    ///
    /// This function returns an error if the technique is already registered
    pub fn register<T: Technique + 'static>(&mut self, technique: T) -> Result<(), Box<dyn Error>> {
        if self.is_registered(&technique) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Technique already registered",
            )));
        }

        // Add the technique to the registry
        debug!("Registering technique: {}", technique.name());
        self.techniques.push(Box::new(technique));
        Ok(())
    }

    /// Check if a technique is already registered
    ///
    /// This function checks if a technique is already registered with the registry.
    ///
    /// # Arguments
    ///
    /// * `technique` - The technique to check
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the technique is registered
    pub fn is_registered<T: Technique + 'static>(&self, technique: &T) -> bool {
        self.techniques.iter().any(|t| t.name() == technique.name())
    }

    /// Get a list of all registered techniques
    ///
    /// This function returns a list of all registered techniques.
    ///
    /// # Returns
    ///
    /// A list of registered techniques
    pub fn techniques(&self) -> &Vec<Box<dyn Technique>> {
        &self.techniques
    }

    /// Run all techniques in the registry
    ///
    /// This function runs all techniques in the registry and returns a list of results.
    ///
    /// # Returns
    ///
    /// A list of tuples containing the technique and the result of the technique
    #[allow(clippy::borrowed_box)] // would have to refactor the whole file to fix this
    pub fn run_all_techniques(&self) -> Vec<(&Box<dyn Technique>, TechniqueResult)> {
        let mut results = Vec::new();
        for technique in self.techniques.iter() {
            debug!("Running technique: {}", technique.name());
            let result = technique.execute();
            results.push((technique, result));
        }
        results
    }
}

/// Wrapper function to safely register a technique with the global registry
///
/// # Arguments
///
/// * `technique` - The technique to register
///
/// # Returns
///
/// A result indicating success or failure
///
/// # Errors
///
/// This function returns an error if the technique is already registered
pub fn register_technique<T: Technique + 'static>(technique: T) -> Result<(), Box<dyn Error>> {
    let mut registry = TECHNIQUE_REGISTRY.lock()?;
    registry.register(technique)
}

/// Run all techniques in the global registry
///
/// This function runs all techniques in the global registry and returns a list of results.
///
/// # Returns
///
/// A list of tuples containing the name of the technique and the result of the technique
///
/// # Errors
///
/// This function returns an error if the global registry is locked
pub fn run_all_techniques() -> Result<Vec<(String, TechniqueResult)>, Box<dyn Error>> {
    let registry = TECHNIQUE_REGISTRY.lock()?;
    let results = registry
        .run_all_techniques()
        .into_iter()
        .map(|(technique, result)| (technique.name().to_string(), result))
        .collect();
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy)]
    struct TestTechnique;

    impl Technique for TestTechnique {
        fn name(&self) -> &'static str {
            "TestTechnique"
        }

        fn description(&self) -> &'static str {
            "Test technique"
        }

        fn execute(&self) -> TechniqueResult {
            Ok(DetectionResult::Detected)
        }
    }

    #[test]
    fn test_technique_registry() {
        let mut registry = TechniqueRegistry::new();
        let technique = TestTechnique;

        assert!(matches!(registry.register(technique), Ok(())));

        assert!(registry.is_registered(&technique));
        assert_eq!(registry.techniques().len(), 1);
        assert_eq!(registry.techniques()[0].name(), "TestTechnique");
        assert_eq!(registry.techniques()[0].description(), "Test technique");
    }

    #[test]
    fn test_register_technique() {
        let mut technique_registry = TechniqueRegistry::new();
        let technique = TestTechnique;

        assert!(matches!(technique_registry.register(technique), Ok(())));
        assert!(matches!(
            technique_registry.register(technique),
            Err(e) if e.to_string() == "Technique already registered"
        ));
    }

    #[test]
    fn test_run_all_techniques() {
        let mut technique_registry = TechniqueRegistry::new();
        let technique = TestTechnique;

        assert!(matches!(technique_registry.register(technique), Ok(())));

        let results = technique_registry.run_all_techniques();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0.name(), "TestTechnique");
        assert_eq!(results[0].1, Ok(DetectionResult::Detected));
    }
}
