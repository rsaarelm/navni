use std::time::Duration;

/// struct FrameCounter that tracks frame rate.
pub struct FrameCounter {
    target_duration: f64,
    last_time: f64,
    avg_frame: f64,
}

impl Default for FrameCounter {
    fn default() -> Self {
        FrameCounter {
            target_duration: 0.0,
            last_time: crate::now(),
            avg_frame: 1.0,
        }
    }
}

impl FrameCounter {
    /// Create a new FrameCounter with the given target duration in seconds.
    ///
    /// The target duration may be zero in which case the counter will never
    /// delay on tick and will never report missed frames.
    pub fn new(target_duration_sec: f64) -> Self {
        FrameCounter {
            target_duration: target_duration_sec,
            last_time: crate::now(),
            avg_frame: 1.0,
        }
    }

    /// Advance the frame counter and update frame duration estimate.
    pub fn tick(&mut self) {
        let now = crate::now();
        let mut delta = now - self.last_time;

        if delta < self.target_duration {
            // If we have a target duration and less time than the duration
            // has elapsed, sleep for the remaining time.
            std::thread::sleep(Duration::from_secs_f64(
                self.target_duration - delta,
            ));
            delta = self.target_duration;
        }

        // Update average frame duration with smoothing.
        self.avg_frame = 0.125 * delta + 0.875 * self.avg_frame;
        self.last_time += delta;
    }

    /// Catch up a frame counter that's fallen behind, but don't update frame
    /// duration estimate.
    pub fn catch_up(&mut self) {
        for _ in 0..self.missed_frames() {
            self.last_time += self.target_duration;
        }
    }

    /// Return how many frames were missed since the last tick.
    ///
    /// If more than zero frames were missed,
    pub fn missed_frames(&self) -> usize {
        let now = crate::now();
        if self.target_duration > 0.0 {
            // How many whole frames have passed since the last tick.
            ((now - self.last_time) / self.target_duration).floor() as usize
        } else {
            // If target duration is not specified, there's no frame to miss
            // and this always returns zero.
            0
        }
    }

    pub fn avg_frame(&self) -> f64 {
        self.avg_frame
    }
}
