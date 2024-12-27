use crate::Nucleotide;
use rand::Rng;

pub struct Sequence {
    seq: Vec<u8>,
}
impl Sequence {
    pub fn new() -> Self {
        Self { seq: Vec::new() }
    }
    pub fn clear_buffer(&mut self) {
        self.seq.clear();
    }
    pub fn fill_buffer<R: Rng + ?Sized>(&mut self, rng: &mut R, len: usize) {
        self.clear_buffer();
        for _ in 0..len {
            let nuc: Nucleotide = rng.gen();
            self.seq.push(nuc.into());
        }
    }
    pub fn bytes(&self) -> &[u8] {
        &self.seq
    }
}
