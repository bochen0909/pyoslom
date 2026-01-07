# OSLOM Rust Implementation

This directory contains the Rust implementation of the OSLOM (Order Statistics Local Optimization Method) algorithm for community detection in networks.

## Structure

- `oslom-core/` - Core algorithm implementation
- `oslom-python/` - Python bindings using PyO3
- `Cargo.toml` - Workspace configuration

## Features

- **Memory Safety**: No segfaults or memory leaks thanks to Rust's ownership system
- **Performance**: Optimized implementation with parallel processing support
- **Modern Architecture**: Clean separation of concerns and modular design
- **Python Integration**: Drop-in replacement for the C++ implementation
- **Comprehensive Testing**: Unit tests and benchmarks included

## Building

### Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- Python 3.8+
- maturin (install with `pip install maturin`)

### Development Build

```bash
# Build the core library
cd oslom-core
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Python Extension

```bash
# Build Python extension
cd oslom-python
maturin develop --release

# Or build wheel
maturin build --release
```

## Usage

### Rust API

```rust
use oslom_core::{NetworkBuilder, OslomConfig, run_oslom};

let mut builder = NetworkBuilder::new(false); // undirected
builder.add_edge(0, 1, 1.0)
       .add_edge(1, 2, 1.0)
       .add_edge(2, 0, 1.0);

let network = builder.build();
let config = OslomConfig::default();
let result = run_oslom(&network, &config)?;

println!("Found {} communities", result.statistics.num_modules);
```

### Python API

```python
from pyoslom.rust_oslom import RustOSLOM
import networkx as nx

# Create a test graph
G = nx.karate_club_graph()

# Run OSLOM
oslom = RustOSLOM(verbose=True)
oslom.fit(G)
result = oslom.transform()

print(f"Found {result['statistics']['num_modules']} communities")
```

## Algorithm Overview

The Rust implementation follows the same algorithmic approach as the original C++ version:

1. **Initialization**: Use Louvain algorithm for initial community detection
2. **Statistical Testing**: Evaluate communities using CUP (Clustering with Uncorrelated Pairs) test
3. **Overlap Detection**: Find and resolve overlapping communities
4. **Hierarchical Processing**: Build hierarchical community structure
5. **Singleton Handling**: Optionally assign isolated nodes

## Performance

The Rust implementation provides several performance improvements:

- **Parallel Processing**: Uses rayon for parallel computation
- **Memory Efficiency**: Better memory layout and reduced allocations
- **Optimized Data Structures**: Custom graph representations for better cache locality
- **SIMD Operations**: Vectorized mathematical operations where applicable

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_basic_oslom

# Run benchmarks
cargo bench
```

## Migration from C++

The Rust implementation is designed as a drop-in replacement:

1. **API Compatibility**: Python API remains identical
2. **Result Format**: Output format matches the C++ version
3. **Parameter Compatibility**: All parameters work the same way
4. **Performance**: Equal or better performance in most cases

## Contributing

1. Follow Rust conventions (use `cargo fmt` and `cargo clippy`)
2. Add tests for new functionality
3. Update benchmarks for performance-critical changes
4. Maintain API compatibility with the C++ version

## License

MIT License - same as the original OSLOM implementation.