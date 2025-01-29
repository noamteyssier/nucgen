use rand::Rng;

#[derive(Default)]
pub struct Sequence {
    seq: Vec<u8>,
    scratch: Vec<u8>, // Scratch space for generating new sequences
}

impl Sequence {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            seq: Vec::with_capacity(capacity),
            scratch: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn clear_buffer(&mut self) {
        self.seq.clear();
    }

    pub fn fill_buffer<R: Rng + ?Sized>(&mut self, rng: &mut R, len: usize) {
        // Ensure scratch has enough capacity
        self.scratch.resize(len, 0);

        // Generate new random nucleotides in scratch buffer
        for b in self.scratch.iter_mut() {
            *b = match rng.gen::<u8>() & 0b11 {
                0 => b'A',
                1 => b'C',
                2 => b'G',
                3 => b'T',
                _ => {
                    unreachable!();
                }
            };
        }

        // Swap buffers to avoid unnecessary copying
        std::mem::swap(&mut self.seq, &mut self.scratch);
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        &self.seq
    }
}
