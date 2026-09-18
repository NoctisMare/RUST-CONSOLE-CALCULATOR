# RUST-CONSOLE-CALCULATOR

A simple console calculator built with Rust that evaluates mathematical expressions step-by-step.

## Features

- ✅ Basic arithmetic operations (+, -, *, /)
- ✅ Step-by-step calculation breakdown
- ✅ Floating-point number support
- ✅ Clean and intuitive command-line interface

## Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── src
│   └── main.rs
└── start.sh
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system

### Installation

1. Clone the repository:
```bash
git clone https://github.com/NoctisMare/RUST-CONSOLE-CALCULATOR.git
cd RUST-CONSOLE-CALCULATOR
```

2. Build the project:
```bash
cargo build --release
```

### Usage

You can run the calculator in two ways:

**Option 1: Using Cargo**
```bash
cargo run
```

**Option 2: Using the start script**
```bash
chmod +x start.sh
./start.sh
```

### Example

```
>>> 3 + 23 - 2 / 5

7
=[1]> 3 + 23 = 26
=[2]> 26 - 2 = 24
=[3]> 24 / 5 = 4.8

[+] Final Result: 4.8
```

## Supported Operations

- Addition (`+`)
- Subtraction (`-`)
- Multiplication (`*`)
- Division (`/`)

## License

This project is licensed under the terms specified in the [LICENSE](LICENSE) file.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
