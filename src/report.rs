//! Weekly tenant report: aggregates directory metrics into fines, top aliases,
//! and cleanup candidates for a reporting period.

use chrono::{DateTime, Utc};

use crate::model::{AliasStat, CleanupCandidate, DirectoryStats, Fine};

/// Tunable knobs for the weekly report.
#[derive(Debug, Clone, Copy)]
pub struct ReportConfig {
    /// Failed commands at or above this make a directory "noisy".
    pub noisy_failure_threshold: usize,
    /// Directories untouched for this many days are "idle".
    pub idle_after_days: u64,
    /// Idle directories must also have at most this many commands.
    pub idle_max_commands: usize,
    /// Fine (in "shells") per failed command at/above the noisy threshold.
    pub fine_per_failure: u64,
    /// Flat fine for idle directories.
    pub