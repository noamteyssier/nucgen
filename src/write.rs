use anyhow::Result;
use std::io::{BufWriter, Write};

use gzp::{
    deflate::Gzip,
    par::compress::{ParCompress, ParCompressBuilder},
};

pub fn write_fasta<W: Write>(writer: &mut W, index: usize, seq: &[u8]) -> Result<()> {
    write!(writer, ">seq.{}\n", index)?;
    writer.write_all(seq)?;
    writer.write_all(b"\n")?;
    Ok(())
}

pub fn write_fastq<W: Write>(writer: &mut W, index: usize, seq: &[u8], qual: &[u8]) -> Result<()> {
    write!(writer, "@seq.{}\n", index)?;
    writer.write_all(seq)?;
    writer.write_all(b"\n+\n")?;
    writer.write_all(qual)?;
    writer.write_all(b"\n")?;
    Ok(())
}

pub fn match_output(path: Option<&str>) -> Result<Box<dyn Write + Send>> {
    match path {
        Some(path) => {
            let file = std::fs::File::create(path)?;
            let buffer = BufWriter::new(file);
            Ok(Box::new(buffer))
        }
        None => {
            let buffer = BufWriter::new(std::io::stdout());
            Ok(Box::new(buffer))
        }
    }
}

pub fn gzip_passthrough(
    writer: Box<dyn Write + Send>,
    compress: bool,
    n_cpus: usize,
) -> Result<Box<dyn Write + Send>> {
    if compress {
        let num_threads = match n_cpus {
            0 => num_cpus::get(),
            _ => n_cpus.min(num_cpus::get()),
        };

        let encoder: ParCompress<Gzip> = ParCompressBuilder::new()
            .num_threads(num_threads)?
            .from_writer(writer);

        Ok(Box::new(encoder))
    } else {
        Ok(writer)
    }
}
