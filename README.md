# hex_util

A simple **hexdump utility** written in Rust.  
It displays binary files in a traditional hex + ASCII format (similar to `xxd` or `hexdump`) with syntax highlighting for readability.

---

## 📂 Project Structure

```plaintext

hex_util/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── generate_test_bin.py    # Script to generate a test binary file
├── src/
│ ├── args.rs               # CLI argument parsing (using clap)
│ ├── edgecase.bin          # Example test binary file
│ ├── hex.rs                # Core hexdump implementation
│ ├── main.rs               # Entry point
│ ├── main.exe              # Windows build artifact (ignored in git)
│ ├── main.pdb              # Debug symbols (Windows)
├── target/                 # Cargo build artifacts

```

---

## ✨ Features

- Reads and dumps binary files in hex + ASCII view
- Highlights:
  - **Hex bytes in green**
  - **ASCII characters in yellow**
- Supports:
  - File offsets
  - Optional length restriction
  - Output redirection to file

Example output:

```powershell
00000000: 48 65 6c 6c 6f 20 57 6f 72 6c 64 21 |Hello World!|
```

---

## 🚀 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable recommended)

### Clone & Build

```bash
git clone https://github.com/yourusername/hex_util.git
cd hex_util
cargo build --release
```

This will produce the binary in:

```bash
target/release/hex_util
```

---

## Usage

Run hex_util on a binary file :

```bash
cargo run -- <input_file>
```

Or with the compiled binary:

```bash
./target/release/hex_util <input_file>
```

### Options

```bash

USAGE:
    hex_util [OPTIONS] <input>

ARGS:
    <input>    Input file to hexdump

OPTIONS:
    -o, --output <file>    Write output to file instead of stdout
    -s, --offset <n>       Start offset (in bytes) [default: 0]
    -l, --length <n>       Maximum number of bytes to read
    -h, --help             Show help

```

## 🛠 Development

### Generate test binary

A helper python script is included to generate test binaries with edge cases:

```bash
python3 generate_test_bin.py
```

This creates **src/edgecase.bin**, which can be used for testing.

Example Run.

```bash
cargo run -- src/edgecase.bin -s 0 -l 64
```

## License

MIT License

## Author

Andrew Fernandes
