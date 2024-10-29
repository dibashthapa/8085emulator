# 8085 Microprocessor Emulator
This is being rewritten in javascript, and the original rust code is moved to `rust` branch

Emulator for 8085 from scratch using Rust.
![8085](images/8085emulator.png 'Program to loop from 01 to 04')

## Documentation

See the [documentation](DOCUMENTATION.md) for more information.

## Installation

Install with cargo

```bash
cargo build
```

## How to Run

### CLI

```bash
cargo run --bin cli <filename>
```

### GUI

```bash
cargo run --bin app
```

## Lessons Learned

I shouldn't have skipped my digital logic classes.

## Roadmap

- Support all the 8085 instructions. Please refer to the [instruction set](https://pastraiser.com/cpu/i8085/i8085_opcodes.html) for more information.

- Support flag registers

## Contributing

Contributions are always welcome!

See the [Contribution](CONTRIBUTING.md) for ways to get started.
