#!/bin/bash

echo "OSLOM Scaling Performance Test"
echo "=============================="
echo

# Build in release mode
cargo build --example scaling_benchmark --release

NETWORK_SIZES=("small" "medium" "large" "huge")

for size in "${NETWORK_SIZES[@]}"; do
    echo "Testing $size network:"
    echo "----------------------"
    
    # Run single-threaded
    echo "Single-threaded:"
    SINGLE_RESULT=$(cargo run --example scaling_benchmark --release $size single 2>/dev/null | grep "RESULT:" | cut -d' ' -f2)
    
    # Run multi-threaded
    echo "Multi-threaded:"
    MULTI_RESULT=$(cargo run --example scaling_benchmark --release $size multi 2>/dev/null | grep "RESULT:" | cut -d' ' -f2)
    
    # Calculate speedup
    if command -v bc >/dev/null 2>&1; then
        SPEEDUP=$(echo "scale=2; $SINGLE_RESULT / $MULTI_RESULT" | bc)
        IMPROVEMENT=$(echo "scale=1; ($SPEEDUP - 1) * 100" | bc)
    else
        SPEEDUP=$(awk "BEGIN {printf \"%.2f\", $SINGLE_RESULT / $MULTI_RESULT}")
        IMPROVEMENT=$(awk "BEGIN {printf \"%.1f\", ($SINGLE_RESULT / $MULTI_RESULT - 1) * 100}")
    fi
    
    echo "Results: ${SPEEDUP}x speedup (${IMPROVEMENT}% improvement)"
    echo
done

echo "CPU cores available: $(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 'unknown')"