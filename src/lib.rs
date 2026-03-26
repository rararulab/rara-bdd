//! rara-bdd — BDD testing framework for rararulab projects.
//!
//! Discovers Gherkin `.feature` scenarios, matches them to `#[test]`
//! functions by naming convention (`ac_XX_*`), runs matched tests,
//! and reports coverage gaps.

pub mod cli;
pub mod discovery;
pub mod error;
pub mod matcher;
pub mod reporter;
pub mod runner;
pub mod traceability;
