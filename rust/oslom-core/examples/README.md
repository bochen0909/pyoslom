# OSLOM Examples

This directory contains examples demonstrating various features of the OSLOM implementation.

## Examples

### `threading_demo.rs`
Demonstrates the threading configuration options:
- Default single-threaded behavior
- Enabling multi-threading with all cores
- Enabling multi-threading with specific thread count
- Benefits of parallel processing with multiple runs

```bash
cargo run --example threading_demo --release
```

### `performance_comparison.rs`
Compares single-threaded vs multi-threaded performance on the same network:

```bash
# Run single-threaded test
cargo run --example performance_comparison --release single

# Run multi-threaded test  
cargo run --example performance_comparison --release multi
```

### `large_network_benchmark.rs`
Tests performance on progressively larger networks:

```bash
cargo run --example large_network_benchmark --release
```

### `scaling_benchmark.rs`
Tests performance scaling across different network sizes:

```bash
cargo run --example scaling_benchmark --release <size> <mode>
# where size = small|medium|large|huge
# where mode = single|multi
```

### `parallel_benchmark.rs`
Original benchmark comparing single vs multi-threaded performance:

```bash
cargo run --example parallel_benchmark --release
```

## Automated Scripts

### `benchmark.sh`
Automated performance comparison script:

```bash
./benchmark.sh
```

### `scaling_test.sh`
Automated scaling test across multiple network sizes:

```bash
./scaling_test.sh
```

## Key Findings

- **Default behavior**: Single-threaded for backward compatibility
- **Enable parallel processing**: Set `num_threads: None` or `num_threads: Some(n > 1)`
- **Best speedups**: Multiple algorithm runs with parallel processing enabled
- **Typical speedup**: 1.1x to 1.8x depending on network size and configuration
- **No overhead**: Single-threaded mode has no performance penalty

## Configuration Examples

```rust
// Default - single-threaded
let config = OslomConfig::default();

// Multi-threaded with all cores
let config = OslomConfig {
    num_threads: None,
    ..Default::default()
};

// Multi-threaded with specific thread count
let config = OslomConfig {
    num_threads: Some(4),
    ..Default::default()
};

// Multi-threaded with multiple runs (best for parallel speedup)
let config = OslomConfig {
    r: 10,           // 10 runs at first level
    hr: 20,          // 20 runs at higher levels  
    num_threads: None, // Use all available threads
    ..Default::default()
};
```