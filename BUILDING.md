# Build instructions

Requirements:
- Rust compiler (> 1.83)

## Cloning repo

To build and launch Flylang you have to clone Git repo:

You have to options:

1. Download pinned version (for example `v0.1.0`):
```
git clone https://github.com/flylang-rs/fly -b <version>
```

2. Or download latest commit:
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
