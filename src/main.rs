mod cli;
mod format;
mod nucleotide;
mod sequence;
mod write;

use std::io::Write;

use cli::Args;
use format::Format;
use nucleotide::Nucleotide;

use anyhow::Result;
use clap::Parser;
use sequence::Sequence;
use write::{write_fasta, write_fastq};

fn gen_single_end(args: Args) -> Result<()> {
    let mut rng = args.get_rng();
    let mut seq = Sequence::new();
    let qual = vec![b'?'; args.slen];

    let mut out = args.output_handle_single()?;

    // Write the records
    for idx in 0..args.num_records {
        seq.fill_buffer(&mut rng, args.slen);
        match args.format {
            Format::Fasta => write_fasta(&mut out, idx, seq.bytes())?,
            Format::Fastq => write_fastq(&mut out, idx, seq.bytes(), &qual)?,
        }
    }

    // Flush the output buffer
    out.flush()?;

    Ok(())
}

fn gen_paired_end(args: Args) -> Result<()> {
    let mut rng = args.get_rng();
    let mut s1 = Sequence::new();
    let mut s2 = Sequence::new();
    let q1 = vec![b'?'; args.slen];
    let q2 = vec![b'?'; args.xlen];

    let (mut out_r1, mut out_r2) = args.output_handle_paired()?;

    // Write the records
    for idx in 0..args.num_records {
        s1.fill_buffer(&mut rng, args.slen);
        s2.fill_buffer(&mut rng, args.xlen);
        match args.format {
            Format::Fasta => {
                write_fasta(&mut out_r1, idx, s1.bytes())?;
                write_fasta(&mut out_r2, idx, s2.bytes())?;
            }
            Format::Fastq => {
                write_fastq(&mut out_r1, idx, s1.bytes(), &q1)?;
                write_fastq(&mut out_r2, idx, s2.bytes(), &q2)?;
            }
        }
    }

    // Flush the output buffers
    out_r1.flush()?;
    out_r2.flush()?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.paired() {
        gen_paired_end(args)
    } else {
        gen_single_end(args)
    }
}
