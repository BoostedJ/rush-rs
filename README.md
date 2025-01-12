# Rush-rs Chess Engine
Rushers is a toy chess engine written in Rust, primarily used to help me learn Rust. The name comes from "Puzzle Rush," as I wanted to make my own engine that could compete with me in puzzle rush. The end goal is to have it beat me at the 2400 puzzle ELO level.

## Status: Early Development
Currently implementing core chess functionality:
- Position evaluation
- Move generation

## Features
- Pure Rust implementation (thus far)
- Bitboard representation
- FEN string parsing
- Zobrist hashing

## Building 
If you want to build Rushers for yourself, ensure cargo is installed and run:
```bash
git clone https://github.com/BoostedJ/rush-rs.git
cd rush-rs
# Build in release for optimized performance
cargo build --release
```
## Contributing
This is an early-stage project, but you can feel free to:
- Open issues for bugs or feature requests
- Submit PRs for improvements
- Suggest optimizations
- Help with documentation
