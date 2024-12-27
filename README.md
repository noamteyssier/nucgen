# nucgen

A fast and simple configurable nucleotide generator for testing bioinformatics tools with random fasta and fastq files.

All nucleotides `{A,C,T,G}` are generated randomly with equal probability.

## Installation

```bash
cargo install nucgen
```

## Usage (CLI)

All the options are configurable via the command line.

You can see the available options by running:

```bash
nucgen --help
```
## Examples

Generate 10,000 reads of length 100bp in a FASTQ format and output to stdout:

```bash
nucgen -n 10000 -l 100 -fq
```

Generate a paired-end dataset of 100 reads with R1 length 30 and R2 length 50.
Output as FASTA format and write to files in gzip format.

```bash
nucgen -n 100 -l 30 -L 50 -fa reads_R1.fasta.gz reads_R2.fasta.gz
```

Seed the random number generator with a specific value:

```bash
nucgen -n 100 -l 100 -fq -S 42
```

## Usage (Library)

Add `nucgen` as a dependency in your `Cargo.toml`:

```bash
cargo add nucgen
```

You can use the `Sequence` struct to generate random nucleotide sequences:

```rust
use nucgen::{Sequence, write_fasta};

// Generate a cursor to write the output to
let mut out = Cursor::new(Vec::new());

// Initialize the random number generator
let mut rng = rand::thread_rng();

// Initialize the sequence struct
let mut seq = Sequence::new();

// Generate 100 random nucleotides into the sequence
seq.fill_buffer(&mut rng, 100);

// Write the sequence to the output cursor
write_fasta(&mut out, 0, seq.bytes())?;

// Generate another 100 random nucleotides
seq.fill_buffer(&mut rng, 100);

// Write the second sequence to the output cursor
write_fasta(&mut out, 1, seq.bytes())?;
```
