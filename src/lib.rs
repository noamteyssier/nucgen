mod format;
mod nucleotide;
mod sequence;
mod write;

pub use format::Format;
pub use nucleotide::Nucleotide;
pub use sequence::Sequence;
pub use write::{gzip_passthrough, match_output, write_fasta, write_fastq};
