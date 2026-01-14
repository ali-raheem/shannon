# Shannon

A Rust CLI utility that calculates and visualizes block-wise Shannon entropy of files. Shannon aims to be command line based replacement for `binwalk -E`.

Shannon is available on [crates.io](https://crates.io/crates/shannon-cli) and on [github](https://github.com/ali-raheem/shannon).

## What is Shannon Entropy?

Shannon entropy measures the randomness or information density in data. Values range from 0 to 8 bits per byte:

- **0**: Completely uniform (e.g., a file of all zeros)
- **~4-5**: Typical text or code
- **~7-8**: High randomness (encrypted or compressed data)

## Use Cases

- **Malware analysis**: Identify packed or encrypted sections in executables
- **File forensics**: Detect hidden or embedded data within files
- **Compression analysis**: Visualize which parts of a file are already compressed
- **Binary reverse engineering**: Understand file structure and locate data regions

## Screenshot

![Shannon entropy visualization](screenshot.png)

## Installation

### Via crates.io

```bash
cargo add shannon-cli
```

### Local build

```bash
cargo build --release
```

The binary will be at `target/release/shannon`.

## Usage

```bash
shannon <input_file> [OPTIONS]
```

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--block-size` | `-b` | 1024 | Block size in bytes for entropy calculation |
| `--width` | | 180 | Chart width in characters |
| `--height` | | 100 | Chart height in characters |
| `--y-max` | `-y` | auto | Maximum Y-axis value (defaults to max entropy found) |
| `--no-plot` | `-n` | false | If set then no graph is plotted |
| `--quiet` | `-q` | false | If set then no summary is printed |
| `--high` | `-H` | 0.95 | High threshold for rising edge detection |
| `--low` | `-L` | 0.85 | Low threshold for falling edge detection |
| `--no-table` | | false | If set then no edge table is printed |

### Examples

Analyze a binary with default settings:
```bash
shannon /usr/bin/ls
```

Use smaller blocks for finer granularity:
```bash
shannon firmware.bin --block-size 256
```

Compact output for smaller terminals:
```bash
shannon large_file.dat --width 80 --height 40
```

## Output

A line summarizing the review is outputted.

The tool renders a bar chart in the terminal where:
- **X-axis**: Block index (position in file)
- **Y-axis**: Entropy value (0-8 bits per byte)

High entropy regions appear as tall bars, making it easy to spot encrypted or compressed sections at a glance.

A table of rising and falling edges in the entropy.

## Library Usage

The crate also exposes a library for use in your own projects:

```rust
use shannon::{entropy, total_entropy, detect_edges, EdgeType};

// Calculate entropy of data (returns bits per byte, 0.0-8.0)
let data = b"Hello, world!";
let e: f64 = entropy(data);

// Calculate total entropy (bits per byte * length)
let total: f64 = total_entropy(data);

// Detect rising/falling entropy edges in a sequence
let values: Vec<(usize, f64)> = vec![(0, 7.8), (1, 7.9), (2, 2.0)];
let edges = detect_edges(&values, 0.95, 0.85); // high/low thresholds
for edge in edges {
    match edge.edge_type {
        EdgeType::Rising => println!("Rising edge at block {}", edge.block_index),
        EdgeType::Falling => println!("Falling edge at block {}", edge.block_index),
    }
}
```

## Todo

- Implement dynamic `BLOCK_SIZE` like binwalk (so identical output is produced without manually matching BLOCK_SIZE).
- Enable input via stdin

## License

MIT
