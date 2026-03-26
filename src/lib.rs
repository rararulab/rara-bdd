//! rara-bdd — BDD testing framework for rararulab projects.
//!
//! Provides Gherkin scenario discovery, YAML-based evaluator DSL,
//! and a test harness that validates acceptance criteria against
//! real cargo tests and source code assertions.

pub mod cli;
pub mod discovery;
pub mod error;
pub mod evaluator;
pub mod harness;
pub mod reporter;
pub mod traceability;
