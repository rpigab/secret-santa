#[cfg(feature = "bench")]
pub mod alloc {
    use peak_alloc::PeakAlloc;

    #[global_allocator]
    pub static PEAK_ALLOC: PeakAlloc = PeakAlloc;

    pub fn check_current_alloc() {
        let current_mem = PEAK_ALLOC.current_usage_as_mb();
        log::trace!("This program currently uses {} MB of RAM.", current_mem);
    }

    pub fn check_final_alloc() {
        let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
        log::debug!("The max RAM amount that was used: {} MB", peak_mem);
    }
}

#[cfg(not(feature = "bench"))]
pub mod alloc {
    pub fn check_current_alloc() {
        log::trace!("noop");
    }

    pub fn check_final_alloc() {
        log::trace!("noop");
    }
}
