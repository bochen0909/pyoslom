#!/bin/bash

echo "OSLOM Parallel Processing Performance Test"
echo "=========================================="
echo

# Build in release mode
echo "Building in release mode..."
cargo build --example performance_comparison --release
echo

# Run single-threaded test
echo "Running single-threaded test..."
SINGLE_RESULT=$(cargo run --example performance_comparison --release single 2>/dev/null | grep "RESULT:" | cut -d' ' -f2)
echo "Single-threaded time: ${SINGLE_RESULT}s"
echo

# Run multi-threaded test  
echo "Running multi-threaded test..."
MULTI_RESULT=$(cargo run --example performance_comparison --release multi 2>/dev/null | grep "RESULT:" | cut -d' ' -f2)
echo "Multi-threaded time: ${MULTI_RESULT}s"
echo

# Calculate speedup using bc for floating point arithmetic
if command -v bc >/dev/null 2>&1; then
    SPEEDUP=$(echo "scale=2; $SINGLE_RESULT / $MULTI_RESULT" | bc)
    IMPROVEMENT=$(echo "scale=1; ($SPEEDUP - 1) * 100" | bc)
    echo "Performance Results:"
    echo "  Speedup: ${SPEEDUP}x"
    echo "  Improvement: ${IMPROVEMENT}%"
else
    # Fallback using awk if bc is not available
    SPEEDUP=$(awk "BEGIN {printf \"%.2f\", $SINGLE_RESULT / $MULTI_RESULT}")
    IMPROVEMENT=$(awk "BEGIN {printf \"%.1f\", ($SINGLE_RESULT / $MULTI_RESULT - 1) * 100}")
    echo "Performance Results:"
    echo "  Speedup: ${SPEEDUP}x"
    echo "  Improvement: ${IMPROVEMENT}%"
fi

echo
echo "Test network: 3000 nodes, ~125k edges, 25 communities"
echo "CPU cores available: $(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 'unknown')"