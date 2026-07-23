//! Exponential backoff with full jitter. Standard library only.

/// Jittered delay in ms for a 0-based `attempt`.
/// `rand01` supplies a value in [0, 1) (inject a real RNG in production).
pub fn backoff_delay(attempt: u32, base_ms: f64, cap_ms: f64, rand01: impl Fn() -> f64) -> u64 {
    let ceiling = cap_ms.min(base_ms * 2f64.powi(attempt as i32));
    (rand01() * ceiling) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capped_and_nonnegative() {
        for a in 0..20 {
            let d = backoff_delay(a, 100.0, 5000.0, || 1.0);
            assert!(d <= 5000);
        }
    }
}
