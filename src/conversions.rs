use consts;

// Right now this only supports 4/4 time.
pub struct TimeCalculator {
    sample_time: u64,
    bpm: f64
}

impl TimeCalculator {
    pub fn new(bpm: f64) -> Self {
        Self { sample_time: 0, bpm }
    }

    pub fn add_seconds(&self, seconds: f64) -> Self {
        Self {
            sample_time: self.sample_time + (consts::SAMPLE_RATE as f64 * seconds) as u64,
            bpm: self.bpm
        }
    }

    pub fn add_quarters(&self, quarters: f64) -> Self {
        let seconds_per_beat = 60.0 / self.bpm;
        self.add_seconds(seconds_per_beat * quarters)
    }

    pub fn add_bars(&self, bars: f64) -> Self {
        self.add_quarters(bars * 4.0)
    }

    pub fn add_eigths(&self, eigths: f64) -> Self {
        self.add_quarters(eigths / 2.0)
    }

    pub fn add_sixteenths(&self, sixteenths: f64) -> Self {
        self.add_quarters(sixteenths / 4.0)
    }

    pub fn time(&self) -> u64 {
        self.sample_time
    }
}

pub fn decibels(db: f32) -> f32 {
    10.0_f32.powf(db/20.0)
}
