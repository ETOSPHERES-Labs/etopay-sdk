use blake2::digest::{FixedOutput, FixedOutputReset, HashMarker, Output, OutputSizeUser, Reset, Update, consts::U32};

/// Blake2b instance with a 256-bit output.
pub struct Blake2b256(blake2::Blake2b<U32>);

impl Blake2b256 {
    /// Creates a new [`Blake2b256`] instance.
    pub fn new() -> Self {
        Self(blake2::Blake2b::default())
    }
}

impl Default for Blake2b256 {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputSizeUser for Blake2b256 {
    type OutputSize = U32;
}

impl Reset for Blake2b256 {
    fn reset(&mut self) {
        self.0.reset();
    }
}

impl FixedOutput for Blake2b256 {
    fn finalize_into(self, out: &mut Output<Self>) {
        self.0.finalize_into(out);
    }
}

impl FixedOutputReset for Blake2b256 {
    fn finalize_into_reset(&mut self, out: &mut Output<Self>) {
        self.0.finalize_into_reset(out);
    }
}

impl Update for Blake2b256 {
    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }
}

impl HashMarker for Blake2b256 {}
