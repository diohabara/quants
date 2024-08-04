pub struct Window {
    size: usize,
    ramp_up: usize,
}

impl Window {
    pub fn new(size: usize, ramp_up: usize) -> Self {
        Window { size, ramp_up }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn ramp_up(&self) -> usize {
        self.ramp_up
    }
}
