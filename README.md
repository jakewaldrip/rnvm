# RNVM - Rust Node Version Manager

A blazingly fast Node.js version manager written in Rust.

## Features

- Install multiple Node.js versions
- Switch between versions easily
- Manage your Node.js installations

## Supported Platforms

- Linux (x86_64)
- macOS (Apple Silicon) - Coming soon

## Installation

Install RNVM using Cargo:

```bash
cargo install rnvm
```

Make sure `~/.cargo/bin` is in your PATH.

Then, download the shell script:

```bash
mkdir -p ~/.rnvm
curl -o ~/.rnvm/rnvm.sh https://raw.githubusercontent.com/jakewaldrip/rnvm/main/rnvm.sh
```

## Setup

Add the following to your `.zshrc` (or equivalent shell config):

```bash
# Setup RNVM (Rust Node Version Manager)
export RNVM_DIR="$HOME/.rnvm"
[ -s "$RNVM_DIR/rnvm.sh" ] && \. "$RNVM_DIR/rnvm.sh"  # This loads rnvm
```

Then source your shell config:
```bash
source ~/.zshrc
```

## Usage

### Install a Node.js version
```bash
rnvm install <version>
```

### Switch to a version
```bash
rnvm use <version>
```

### Check current version
```bash
rnvm current
```

### List installed versions
```bash
rnvm list
```

### Remove a version
```bash
rnvm remove <version>
```

## How it works

RNVM downloads Node.js binaries from the official Node.js website and manages them in the `$RNVM_DIR` directory. It modifies your PATH to use the selected version.
