pub struct Timer {
    elapsed: f64,
    wait: f64,
}

impl Timer {
    pub fn new(wait: f64) -> Self {
        Self { elapsed: 0.0, wait }
    }

    pub fn update(&mut self, elapsed_time: f64) -> bool {
        self.elapsed += elapsed_time;
        self.is_done()
    }

    pub fn is_done(&self) -> bool {
        self.wait <= self.elapsed
    }

    pub fn reset(&mut self) {
        while self.is_done() {
            self.elapsed = 0.0;
        }
    }

    pub fn elapsed(&self) -> f64 {
        self.elapsed
    }

    pub fn time_left(&self) -> f64 {
        (self.wait - self.elapsed).max(0.0)
    }

    pub fn percent_elapsed(&self) -> f64 {
        (self.elapsed / self.wait).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_timer() {
        let timer = Timer::new(1.0);
        assert_eq!(timer.elapsed(), 0.0);
        assert_eq!(timer.time_left(), 1.0);
        assert_eq!(timer.percent_elapsed(), 0.0);
    }

    #[test]
    fn test_update_timer() {
        let mut timer = Timer::new(1.0);

        timer.update(0.6);
        assert_eq!(timer.elapsed(), 0.6);
        assert_eq!(timer.time_left(), 0.4);
        assert_eq!(timer.is_done(), false);
        assert_eq!(timer.percent_elapsed(), 0.6);

        timer.update(0.6);
        assert_eq!(timer.elapsed(), 1.2);
        assert_eq!(timer.time_left(), 0.0);
        assert_eq!(timer.is_done(), true);
        assert_eq!(timer.percent_elapsed(), 1.0);
    }

    #[test]
    fn test_reset_timer() {
        let mut timer = Timer::new(1.0);

        timer.update(1.5);
        assert_eq!(timer.elapsed(), 1.5);
        assert_eq!(timer.is_done(), true);

        timer.reset();
        assert_eq!(timer.elapsed(), 0.0);
    }
}
