//! # nucgen
//!
//! A minimal library for generating random nucleotide sequences.
//!
//! ## Example
//!
//! ```rust
//! use nucgen::{write_fasta, write_fastq, Format, Sequence};
//! use std::io::Cursor;
//!
//! fn main() -> anyhow::Result<()> {
//!    // Use any RNG you like
//!    let mut rng = rand::thread_rng();
//!
//!    // Initialize the sequence buffer
//!    let mut seq = Sequence::new();
//!
//!    // Initialize a quality score buffer (you can implement this however youd like.)
//!    let qual = vec![b'?'; 100];
//!
//!    // Fill the sequence buffer with random nucleotides
//!    seq.fill_buffer(&mut rng, 100);  // 100 nucleotides
//!
//!    // Cursor to simulate IO
//!    let mut out_fa = Cursor::new(Vec::new());
//!    let mut out_fq = Cursor::new(Vec::new());
//!
//!    // Write the sequence to a FASTA file
//!    //
//!    // The internal index is used to generate the sequence ID (e.g., `seq.0`)
//!    write_fasta(&mut out_fa, 0, seq.bytes())?;
//!
//!    // Or write the sequence to a FASTQ file
//!    write_fastq(&mut out_fq, 0, seq.bytes(), &qual)?;
//!
//!    Ok(())
//! }

mod format;
mod sequence;
mod write;

pub use format::Format;
pub use sequence::Sequence;
pub use write::{gzip_passthrough, match_output, write_fasta, write_fastq};

#[cfg(test)]
mod testing {

    use std::io::Cursor;

    use super::*;
    use anyhow::Result;

    use seq_io::{
        fasta::{Reader as FastaReader, Record as FastaRecord},
        fastq::{Reader as FastqReader, Record as FastqRecord},
    };

    #[test]
    fn test_write_fasta() -> Result<()> {
        let mut seq = Sequence::new();
        let mut rng = rand::thread_rng();
        seq.fill_buffer(&mut rng, 10);

        let mut out = Cursor::new(Vec::new());
        write_fasta(&mut out, 0, seq.bytes())?;

        let binding = out.into_inner();
        let mut reader = FastaReader::new(binding.as_slice());
        let record = reader.next().unwrap().unwrap();
        assert_eq!(record.id(), Ok("seq.0"));
        assert_eq!(record.seq(), seq.bytes());

        assert!(reader.next().is_none());
        Ok(())
    }

    #[test]
    fn test_write_fastq() -> Result<()> {
        let mut seq = Sequence::new();
        let mut rng = rand::thread_rng();
        seq.fill_buffer(&mut rng, 10);
        let qual = vec![b'?'; 10];

        let mut out = Cursor::new(Vec::new());
        write_fastq(&mut out, 0, seq.bytes(), &qual)?;

        let binding = out.into_inner();
        let mut reader = FastqReader::new(binding.as_slice());
        let record = reader.next().unwrap().unwrap();
        assert_eq!(record.id(), Ok("seq.0"));
        assert_eq!(record.seq(), seq.bytes());
        assert_eq!(record.qual(), &qual);

        assert!(reader.next().is_none());
        Ok(())
    }
}
