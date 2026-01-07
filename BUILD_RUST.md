# Building PyOSLOM with Rust

This document explains how to build the Rust implementation of PyOSLOM.

## Prerequisites

1. **Rust**: Install from https://rustup.rs/
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Python 3.8+**: Make sure you have Python installed

3. **Maturin**: Will be installed automatically by the build script

## Building

### Option 1: Using the build script (Recommended)

```bash
# Development build (installs in current environment)
python build_rust.py develop

# Build wheel
python build_rust.py wheel
```

### Option 2: Using maturin directly

```bash
# Install maturin if not already installed
pip install maturin

# Development build
maturin develop --release

# Build wheel
maturin build --release
```

### Option 3: Using pip (if configured properly)

```bash
pip install -e .
```

## Testing the Build

After building, test that the Rust implementation is working:

```python
import pyoslom
print(f"Using implementation: {pyoslom.get_implementation()}")

# Should print "rust" if successful
```

## Troubleshooting

### "Rust is not installed"
- Install Rust from https://rustup.rs/
- Make sure `rustc` is in your PATH

### "maturin not found"
- Install with: `pip install maturin`

### Build errors
- Make sure you have the latest Rust version: `rustup update`
- Try cleaning the build: `cargo clean` in the `rust/` directory

### Import errors
- Make sure the build completed successfully
- Check that the extension was installed in the right environment
- Try: `python -c "import pyoslom._rust; print('Rust extension loaded')"`

## Development

For development work on the Rust code:

```bash
cd rust/oslom-core
cargo test          # Run tests
cargo bench         # Run benchmarks
cargo clippy        # Linting
cargo fmt           # Format code
```

## Performance

The Rust implementation should provide:
- Better memory safety (no segfaults)
- Improved performance through better optimization
- Parallel processing capabilities
- More reliable error handling

Compare performance with:
```python
import time
import networkx as nx
from pyoslom import OSLOM

G = nx.karate_club_graph()
oslom = OSLOM(verbose=True)

start = time.time()
oslom.fit(G)
end = time.time()

print(f"Time: {end - start:.3f}s")
result = oslom.transform()
print(f"Found {result['statistics']['num_modules']} communities")
```