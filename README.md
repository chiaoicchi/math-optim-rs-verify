# Library Checker for Rust

This is a repository to verify [math-optim-rs](https://github.com/chiaoicchi/math-optim-rs).

## Environment

- Rust 1.89.0 (pinned via [rust-toolchain.toml](rust-toolchain.toml))
- Nix flake for reproducible development environment

## Setup

### Prerequisites

- [Nix](https://nixos.org/) with flakes enabled
- [direnv](https://github.com/direnv/direnv) (optional)

### Getting Started

Enter the development environment:

```bash
nix develop
```

Or, if you have direnv set up:

```bash
direnv allow
```

## License

This project is licensed under the [MIT License](LICENSE).
