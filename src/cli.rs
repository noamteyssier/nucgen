use std::io::Write;

use anyhow::{bail, Result};
use clap::Parser;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::{
    format::Format,
    write::{gzip_passthrough, match_output},
};

#[derive(Parser, Debug)]
pub struct Args {
    /// Number of records to generate
    #[clap(short, long, default_value = "1000")]
    pub num_records: usize,

    /// Length of each record (primary sequence)
    #[clap(short, long, default_value = "100")]
    pub slen: usize,

    /// Length of each record (secondary sequence)
    #[clap(short, long, default_value = "0")]
    pub xlen: usize,

    /// Output format
    #[clap(short, long, default_value = "a")]
    pub format: Format,

    /// RNG seed
    #[clap(short = 'S', long)]
    pub seed: Option<u64>,

    /// Gzip compress output file(s)
    #[clap(short, long, default_value = "false")]
    pub compress: bool,

    /// Number of threads to use for parallel compression (0 for auto)
    #[clap(short = 'T', long, default_value = "1")]
    pub threads: usize,

    /// Output file [default: stdout]
    ///
    /// If `xlen` is not zero, two output files are required.
    /// If `xlen` is zero, a single output file is required or stdout is used if not provided.
    #[clap(num_args = 0..2)]
    pub output: Vec<String>,
}
impl Args {
    pub fn get_rng(&self) -> ChaCha8Rng {
        if let Some(seed) = self.seed {
            ChaCha8Rng::seed_from_u64(seed) // Use the provided seed
        } else {
            ChaCha8Rng::from_entropy() // Use the system entropy
        }
    }

    pub fn compress(&self) -> bool {
        match self.output.len() {
            0 => self.compress,
            1 => self.output[0].ends_with(".gz") || self.compress,
            _ => self.output.iter().all(|path| path.ends_with(".gz")) || self.compress,
        }
    }

    pub fn output_handle_single(&self) -> Result<Box<dyn Write + Send>> {
        let out = match self.output.len() {
            0 => match_output(None),
            1 => match_output(Some(&self.output[0])),
            _ => bail!("Too many output handles expected for single read stream output"),
        }?;
        gzip_passthrough(out, self.compress(), self.threads)
    }

    pub fn output_handle_paired(&self) -> Result<(Box<dyn Write + Send>, Box<dyn Write + Send>)> {
        match self.output.len() {
            2 => {
                let out1 = match_output(Some(&self.output[0]))?;
                let out2 = match_output(Some(&self.output[1]))?;
                let out1 = gzip_passthrough(out1, self.compress(), self.threads)?;
                let out2 = gzip_passthrough(out2, self.compress(), self.threads)?;
                Ok((out1, out2))
            }
            _ => bail!("Two output handles expected for paired read stream output"),
        }
    }

    pub fn paired(&self) -> bool {
        self.xlen > 0
    }
}