# Build instructions

Requirements:
- Rust compiler (> 1.83)

## Cloning repo

To build and launch Flylang you have to clone Git repo:

```
git clone https://github.com/flylang-rs/fly
```

## Build

Enter the cloned repo and build it:
```
cd fly
cargo b
```

## Install (optional)

If you wish to install Flylang interpreter, execute following command:

```
cargo install --locked --path .
```

It will install Flylang to your `~/.cargo/bin/` directory.