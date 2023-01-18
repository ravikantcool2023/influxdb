//! CLI config for compactor2-related commands

use std::num::NonZeroUsize;

/// CLI config for compactor2
#[derive(Debug, Clone, Copy, clap::Parser)]
pub struct Compactor2Config {
    /// Number of partitions that should be compacted in parallel.
    #[clap(
        long = "compaction-partition-concurrency",
        env = "INFLUXDB_IOX_COMPACTION_PARTITION_CONCURRENCY",
        default_value = "10",
        action
    )]
    pub compaction_partition_concurrency: NonZeroUsize,
}
