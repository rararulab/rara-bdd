//! rara-bdd — cucumber-rs scaffolding tool for AI agents.
//!
//! Generates cucumber-rs project skeletons and step definition code
//! from Gherkin `.feature` files, enabling AI agents to quickly
//! set up and use BDD testing in Rust projects.

pub mod cli;
pub mod discovery;
pub mod error;
pub mod generate;
pub mod setup;
pub mod step_coverage;
