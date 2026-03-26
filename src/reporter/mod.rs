//! Report generation for BDD suite results.

mod json;
mod markdown;
mod terminal;

use crate::cli::ReportFormat;
use crate::discovery::Scenario;
use crate::evaluator::SuiteResults;

/// Output suite results in the requested format.
pub fn report(results: &SuiteResults, format: ReportFormat) {
    match format {
        ReportFormat::Terminal => terminal::report(results),
        ReportFormat::Json => json::report(results),
        ReportFormat::Markdown => markdown::report(results),
    }
}

/// List discovered scenarios to terminal.
pub fn list_scenarios(scenarios: &[Scenario]) {
    terminal::list_scenarios(scenarios);
}
