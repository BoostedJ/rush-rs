pub struct LcgRng {
    state: u64,
}

impl LcgRng {
    pub fn new(seed: u64) -> Self {
        LcgRng { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        const A: u64 = 6364136223846793005;
        const C: u64 = 1;
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }
}