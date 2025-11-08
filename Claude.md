# Helix - DNA Blockchain Project

## Project Overview

Helix is a blockchain-based application written in Rust for anonymous DNA storage and validation. The project aims to provide completely anonymous DNA sequencing, storage, and distribution using blockchain technology to ensure data integrity and privacy.

## Architecture

### Core Components

#### 1. Blockchain Module (`src/main.rs`)
- **Block Structure**: Each block contains:
  - `p_hash`: Previous block hash (SHA-256)
  - `id`: User ID hash (SHA-256)
  - `dna_hash`: DNA sequence hash (SHA-256)

- **Chain Structure**: Manages the blockchain state
  - `blockchain`: Vector of mined blocks
  - `pending_txns`: Vector of pending transactions waiting to be mined

- **Key Functions**:
  - `add_dna()`: Adds DNA sequence to pending transactions
  - `mine_option()`: Mines pending transactions into blocks
  - `upload_option()`: Interactive CLI for DNA upload

#### 2. Compression Modules (`src/compression/`)

**Perceptual Hash Module** (`perceptual.rs`)
- Implements perceptual hashing for DNA sequences
- Uses hamming distance for similarity measurement
- Key functions:
  - `hash()`: Creates a 64-bit perceptual hash from DNA sequence
  - `distance_u64()`: Calculates hamming distance between two hashes
  - `distance()`: Calculates distance between two byte sequences

**FASTA Module** (`fasta.rs`)
- Implements FASTA format support for DNA storage
- Supports full IUB/IUPAC nucleic acid codes: A, C, G, T, N, U, K, S, Y, M, W, R, B, D, H, V, -
- Key features:
  - `Fasta` struct with definition, sequence, and perceptual hash
  - `valid_seq()`: Validates DNA sequences against IUB/IUPAC standards
  - `set_seq()`: Sets and validates DNA sequence
  - `distance_to()`: Computes similarity between DNA sequences

## Current State

### What Works ✓
- Basic blockchain creation and mining
- SHA-256 hashing for blocks and DNA
- DNA validation (currently only ACTG in main.rs)
- Duplicate DNA detection
- Perceptual hashing module (fully tested)
- FASTA compression module (fully tested)
- Interactive CLI interface

### What's Not Working/Missing ✗
1. **Compression modules disconnected**: FASTA and perceptual modules exist but are never used by main application
2. **No persistence**: Blockchain data lost on exit
3. **Limited DNA validation**: Main app only validates ACTG, doesn't use FASTA's extended nucleic acid support
4. **No blockchain verification**: Can't validate chain integrity
5. **No search/query**: Can't look up specific DNA or user records
6. **Outdated dependencies**: Using Rust 2015 edition, old crate versions

### Test Coverage
All compression module tests pass (6/6):
- `compression::fasta::tests::distance_to`
- `compression::fasta::tests::valid_functional`
- `compression::perceptual::tests::byte_distance_functional`
- `compression::perceptual::tests::distance_functional`
- `compression::perceptual::tests::hash_similarity`
- `compression::perceptual::tests::distance_safety`

## Dependencies

```toml
sha2 = "0.7.1"          # SHA-256 hashing
serde = "1.0"           # Serialization
serde_json = "1.0"      # JSON support
serde_derive = "1.0"    # Derive macros
regex = "1.0.2"         # DNA validation
hex = "0.3.2"           # Hex encoding
data-encoding = "2.1.1" # Base encoding
rand = "0.5.5"          # Random generation (tests)
```

## Building and Running

```bash
cargo build    # Build the project
cargo run      # Run interactive CLI
cargo test     # Run all tests
```

## Usage Flow

1. User starts application
2. Choose to upload DNA (creates pending transaction)
3. Enter DNA sequence (validated for ACTG only)
4. Enter user ID
5. Choose to mine block (converts pending txn to block)
6. Blockchain printed to console
7. Exit loses all data (no persistence)

## Security Features

- All DNA sequences hashed with SHA-256 before storage
- User IDs hashed with SHA-256 for anonymity
- Duplicate DNA detection prevents same sequence being added twice
- Each block linked to previous via cryptographic hash

## Known Issues

- Genesis block uses hardcoded hash "0000000000"
- No proof-of-work mechanism
- Perceptual hashing not utilized for similarity searches
- FASTA compression capabilities unused
- Main app exits on most operations (excessive `process::exit(0)`)

## File Structure

```
helix/
├── Cargo.toml              # Project dependencies
├── Cargo.lock              # Locked dependency versions
├── README.md               # User-facing documentation
├── Claude.md               # This file - AI assistant context
├── PLAN.md                 # Implementation roadmap
└── src/
    ├── main.rs             # Main blockchain application
    └── compression/
        ├── mod.rs          # Module declarations
        ├── perceptual.rs   # Perceptual hashing
        └── fasta.rs        # FASTA format support
```

## Development Notes

- Code uses Rust 2015 edition (default when no edition specified)
- Several compiler warnings present (mostly from old serde_derive)
- Perceptual hash algorithm based on: https://arxiv.org/ftp/arxiv/papers/1412/1412.5517.pdf
- FASTA codes reference: https://blast.ncbi.nlm.nih.gov/Blast.cgi?CMD=Web&PAGE_TYPE=BlastDocs&DOC_TYPE=BlastHelp

## Next Steps

See `PLAN.md` for detailed implementation roadmap.
