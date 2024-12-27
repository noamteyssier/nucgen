use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Debug, Clone, Copy)]
pub enum Nucleotide {
    A,
    C,
    G,
    T,
}
impl Distribution<Nucleotide> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Nucleotide {
        match rng.gen_range(0..4) {
            0 => Nucleotide::A,
            1 => Nucleotide::C,
            2 => Nucleotide::G,
            3 => Nucleotide::T,
            _ => unreachable!(),
        }
    }
}
impl From<Nucleotide> for u8 {
    fn from(nuc: Nucleotide) -> u8 {
        nuc.to_u8()
    }
}
impl Nucleotide {
    pub fn to_char(&self) -> char {
        match self {
            Nucleotide::A => 'A',
            Nucleotide::C => 'C',
            Nucleotide::G => 'G',
            Nucleotide::T => 'T',
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Nucleotide::A => b'A',
            Nucleotide::C => b'C',
            Nucleotide::G => b'G',
            Nucleotide::T => b'T',
        }
    }

    pub fn generate_sequence<R: Rng + ?Sized>(rng: &mut R, len: usize) -> Vec<Nucleotide> {
        (0..len).map(|_| rng.gen()).collect()
    }
}
