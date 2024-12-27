use clap::ValueEnum;

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    #[clap(name = "a")]
    Fasta,
    #[clap(name = "q")]
    Fastq,
}
impl Format {
    pub fn from_path(path: &str) -> Option<Self> {
        let ext = match path.split('.').last()? {
            "gz" => path.split('.').nth_back(1)?,
            ext => ext,
        };
        match ext {
            "fasta" | "fa" => Some(Self::Fasta),
            "fastq" | "fq" => Some(Self::Fastq),
            _ => None,
        }
    }
}
