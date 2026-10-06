//! Shared weighted request scheduler with recovery allowance (Hyperliquid 1,200 weight/min per IP).

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Default Hyperliquid REST API weight limit per IP per minute.
pub const DEFAULT_MAX_WEIGHT_PER_MIN: u32 = 1200;

/// Default weight reserved exclusively for recovery, reconciliation, and diagnostic tasks.
pub const DEFAULT_RECOVERY_RESERVED_WEIGHT: u32 = 200;

#[derive(Debug)]
struct SchedulerInner {
    max_weight_per_min: u32,
    recovery_reserved_weight: u32,
    window_start: Instant,
    used_weight: u32,
}

/// Shared weighted request scheduler enforcing rate limits across all Hyperliquid REST requests.
///
/// Distinguishes between standard operational traffic and recovery/reconciliation traffic,
/// ensuring that routine background tasks cannot starve critical crash recovery or rebootstrap.
#[derive(Debug, Clone)]
pub struct RequestScheduler {
    inner: Arc<Mutex<SchedulerInner>>,
}

impl Default for RequestScheduler {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_WEIGHT_PER_MIN, DEFAULT_RECOVERY_RESERVED_WEIGHT)
    }
}

impl RequestScheduler {
    pub fn new(max_weight_per_min: u32, recovery_reserved_weight: u32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(SchedulerInner {
                max_weight_per_min,
                recovery_reserved_weight,
                window_start: Instant::now(),
                used_weight: 0,
            })),
        }
    }

    /// Acquire request quota for the specified weight.
    ///
    /// If `is_recovery` is true, the request may utilize the reserved recovery capacity.
    /// Non-recovery requests are throttled when remaining capacity reaches the recovery floor.
    pub async fn acquire(&self, weight: u32, is_recovery: bool) {
        loop {
            let mut guard = self.inner.lock().await;
            let elapsed = guard.window_start.elapsed();
            if elapsed >= Duration::from_secs(60) {
                guard.window_start = Instant::now();
                guard.used_weight = 0;
            }

            let effective_limit = if is_recovery {
                guard.max_weight_per_min
            } else {
                guard
                    .max_weight_per_min
                    .saturating_sub(guard.recovery_reserved_weight)
            };

            if guard.used_weight + weight <= effective_limit {
                guard.used_weight += weight;
                return;
            }

            let wait_time =
                Duration::from_secs(60).saturating_sub(elapsed) + Duration::from_millis(5);
            drop(guard);
            tokio::time::sleep(wait_time).await;
        }
    }

    /// Check currently consumed weight within the active 60s window.
    pub async fn current_usage(&self) -> u32 {
        let guard = self.inner.lock().await;
        if guard.window_start.elapsed() >= Duration::from_secs(60) {
            0
        } else {
            guard.used_weight
        }
    }

    /// Check available capacity for the specified priority.
    pub async fn available_capacity(&self, is_recovery: bool) -> u32 {
        let guard = self.inner.lock().await;
        let used = if guard.window_start.elapsed() >= Duration::from_secs(60) {
            0
        } else {
            guard.used_weight
        };
        let effective_limit = if is_recovery {
            guard.max_weight_per_min
        } else {
            guard
                .max_weight_per_min
                .saturating_sub(guard.recovery_reserved_weight)
        };
        effective_limit.saturating_sub(used)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_request_scheduler_normal_vs_recovery_allowance() {
        // Scheduler with max 100 weight and 20 recovery reserved
        let scheduler = RequestScheduler::new(100, 20);

        // Consume 80 weight with normal requests
        scheduler.acquire(80, false).await;
        assert_eq!(scheduler.current_usage().await, 80);

        // Normal request of weight 1 should have 0 available capacity
        assert_eq!(scheduler.available_capacity(false).await, 0);

        // Recovery request still has 20 capacity available
        assert_eq!(scheduler.available_capacity(true).await, 20);

        // Recovery can consume up to 20
        scheduler.acquire(15, true).await;
        assert_eq!(scheduler.current_usage().await, 95);
        assert_eq!(scheduler.available_capacity(true).await, 5);
    }
}
