# subtitle-lines

Streaming, byte-oriented subtitle parsing and conversion library.

Unlike many other Rust crates subtitle-lines correctly processing an
ASCII-compatible encoding subtitles (UTF-8, Windows-1251, etc.)
without decode to UTF-8 String.

[![Tests](https://github.com/nujievik/subtitle-lines-rs/actions/workflows/tests.yml/badge.svg)](
https://github.com/nujievik/subtitle-lines-rs/actions/workflows/tests.yml)


## Supported Formats
- ASS/SSA
- SRT
- VTT


## Features
- Borrowing architecture.
- Streaming iteration.
- Streaming format conversion (without styles support).
- Support for ASCII-compatible encodings (UTF-8, Windows-1251, etc.).
- Working with bytes without decode costs.


## TODO
- Add support for other formats.
- Styles support.
