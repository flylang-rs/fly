![](static/logo.png)

# Flylang 🪽

A modern, fast, interpreted programming language created just for fun.

It offers simple syntax, meaningful diagnostics and nice CLI (work in progress).

# Examples

See the [fly-examples/](https://github.com/flylang-rs/fly/tree/main/fly-examples) directory for examples.

# Build and installation

Currently the project doesn't provide releases, so you have to build it yourself.

See [BUILDING.md](BUILDING.md) for build instructions.

# Usage

If you installed it to your `~/.cargo/bin/` folder:
```
flylang <script.fly>
```

If you have only built it:
```
cargo r -- <script.fly>
```

***Note: Pass `--help` as a command line argument to show available options.***

# Note

This project is currently work in progress, and because of that some parts of language may be unstable.

You may expect syntax standard changes, interpreter panics, wrong interpreter behaviour, and other kind of bugs.

If you found a bug, you can report it in [Issues tab](https://github.com/flylang-rs/fly/issues).

# Contributing

Flylang welcomes people who is intereseted in language growth and who wants to help.

If you have a spark, follow these steps:
1. *Check [Issues](https://github.com/flylang-rs/fly/issues)*: See if your idea or bug is already being discussed. If not, open a new issue first.
2. Fork & Clone: Fork the repo and create a new branch (`git checkout -b your-feature`).
3. Make your changes
4. Run tests: Make sure that all tests (`cargo test --workspace`) pass.
5. Submit a PR: Push to your fork and submit a Pull Request.