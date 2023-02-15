pub(self) use zksync_config::configs::chain::StateKeeperConfig;
use zksync_types::block::BlockGasCount;
use zksync_types::tx::ExecutionMetrics;

use super::{SealCriterion, SealResolution};

/// Represents a thread-safe function pointer.
type CustomSealerFn = dyn Fn(
        &StateKeeperConfig,
        u128,
        usize,
        ExecutionMetrics,
        ExecutionMetrics,
        BlockGasCount,
        BlockGasCount,
    ) -> SealResolution
    + Send
    + 'static;

/// Custom criterion made from a user-provided function. Allows to turn your closure into a seal criterion.
/// Mostly useful for tests.
pub(crate) struct FnCriterion(Box<CustomSealerFn>);

impl std::fmt::Debug for FnCriterion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FnCriterion").finish()
    }
}

impl SealCriterion for FnCriterion {
    fn should_seal(
        &self,
        config: &StateKeeperConfig,
        block_open_timestamp_ms: u128,
        tx_count: usize,
        block_execution_metrics: ExecutionMetrics,
        tx_execution_metrics: ExecutionMetrics,
        block_gas_count: BlockGasCount,
        tx_gas_count: BlockGasCount,
    ) -> SealResolution {
        self.0(
            config,
            block_open_timestamp_ms,
            tx_count,
            block_execution_metrics,
            tx_execution_metrics,
            block_gas_count,
            tx_gas_count,
        )
    }

    fn prom_criterion_name(&self) -> &'static str {
        "function_sealer"
    }
}
