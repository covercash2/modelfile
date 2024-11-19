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

## I have a Modelfile, but I want a `struct`!

Good news!

```rust
use modelfile::modelfile::Modelfile;

fn parse() {
    let my_modelfile_string = std::fs::read_to_string("./test/llama3.2:latest.Modelfile")
        .expect("handle your dang errors");

    let my_modelfile: Modelfile = my_modelfile_string.parse()
        .expect("if your Modelfile is good this should work!");

    let mut builder = my_modelfile.build_on();

    let builder = builder
        .system("You are a malevolent, world-ending AI agent")
        .expect("only one system message allowed");

    let my_new_modelfile = builder.build()
        .expect("you did it right. you're smart")
        .render();

    std::fs::write("Modelfile", my_new_modelfile)
        .expect("should be able to write to your own dang disk");
}

```

[Ollama]: https://ollama.com/
[Modelfile]: https://github.com/ollama/ollama/blob/main/docs/modelfile.md
[`nom`]: https://github.com/rust-bakery/nom
