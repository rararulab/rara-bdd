//! Report generation for BDD suite results.

mod json;
mod markdown;
mod terminal;

use crate::{cli::ReportFormat, matcher::MatchedAc, runner::SuiteResults};

/// Output suite results in the requested format.
pub fn report(results: &SuiteResults, format: ReportFormat) {
    match format {
        ReportFormat::Terminal => terminal::report(results),
        ReportFormat::Json => json::report(results),
        ReportFormat::Markdown => markdown::report(results),
    }
}

/// List discovered ACs and their matched tests to terminal.
pub fn list_matched(matched: &[MatchedAc]) { terminal::list_matched(matched); }
