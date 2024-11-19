# `modelfile`

[![build](https://github.com/covercash2/modelfile/actions/workflows/rust.yml/badge.svg)](https://github.com/covercash2/modelfile/actions/workflows/rust.yml)

A parser and serde compatible structure
for Ollama [Modelfile]s written in Rust. 🦀

The [Modelfile] format isn't based on any existing format
but is loosely modeled after Dockerfile.

The parser is based on the [`nom`] crate
and tested on a few distinct [Modelfile]s.
If you find a [Modelfile] that doesn't parse,
please open an issue!

[Modelfile]: https://github.com/ollama/ollama/blob/main/docs/modelfile.md
[`nom`]: https://github.com/rust-bakery/nom
