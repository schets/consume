use std::sync::atomic::Ordering;

#[cfg(any(target_arch = "x86",
          target_arch = "x86_64",
          target_arch = "aarch64",
          target_arch = "arm",
          target_arch = "powerpc",
          target_arch = "powerpc64"))]
mod can_consume {
    use std::sync::atomic::Ordering;
    pub const CONSUME: Ordering = Ordering::Relaxed;
}

#[cfg(not(any(target_arch = "x86",
              target_arch = "x86_64",
              target_arch = "aarch64",
              target_arch = "arm",
              target_arch = "powerpc",
              target_arch = "powerpc64")))]
mod can_consume {
    use std::sync::atomic::Ordering;
    pub const CONSUME: Ordering = Ordering::Acquire;
}

#[allow(non_upper_case_globals)]
pub const Consume: Ordering = can_consume::CONSUME;
