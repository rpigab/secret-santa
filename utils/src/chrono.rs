use std::time::Instant;

pub struct Chrono(pub Instant);

impl Chrono {
    pub fn new() -> Self {
        Self {
            0: Instant::now(),
        }
    }
}

impl Chrono {
    pub fn stop(&self) {
        log::info!("time elapsed: {:.2?}", self.0.elapsed());
    }
}
