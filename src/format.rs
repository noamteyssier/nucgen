use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    #[clap(name = "a")]
    Fasta,
    #[clap(name = "q")]
    Fastq,
}
