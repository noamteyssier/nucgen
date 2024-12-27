# nucgen

A fast and simple configurable fast[aq] generator for testing bioinformatics tools.

All nucleotides `{A ,C, T, G}` are generated randomly with equal probability.

## Installation

```bash
cargo install nucgen
```

## Usage

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
