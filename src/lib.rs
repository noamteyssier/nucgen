mod format;
mod nucleotide;
mod sequence;
mod write;

pub use format::Format;
pub use nucleotide::Nucleotide;
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
