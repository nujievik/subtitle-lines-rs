# subtitle-lines

Rust library to parse and convert subtitles directly from bytes,
without decoding to UTF-8 or loading the entire subtitle file into
memory.

Unlike many other Rust crates subtitle-lines correctly processing an
ASCII-compatible encoding subtitles (UTF-8, Windows-1251, etc.)
without costs to decode bytes.

[![Tests](https://github.com/nujievik/subtitle-lines-rs/actions/workflows/tests.yml/badge.svg)](
https://github.com/nujievik/subtitle-lines-rs/actions/workflows/tests.yml)


## Supported Formats
- ASS/SSA
- SRT
- VTT

TODO: Add support for other formats.


## Features
- Borrowing architecture
- Streaming iteration
- Streaming format conversion
- Support for ASCII-compatible encodings (UTF-8, Windows-1251, etc.)
- Working with bytes without decode costs
