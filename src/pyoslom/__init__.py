"""
PyOSLOM - Python wrapper for OSLOM algorithm

This package provides both C++ (legacy) and Rust implementations of the OSLOM
(Order Statistics Local Optimization Method) algorithm for community detection.

The Rust implementation is the default and recommended version.
"""

# Try to import Rust implementation first (preferred)
try:
    from .rust_oslom import RustOSLOM as OSLOM
    IMPLEMENTATION = "rust"
except ImportError:
    # Fall back to C++ implementation if Rust is not available
    try:
        from .oslom_imp import OSLOM
        IMPLEMENTATION = "cpp"
    except ImportError:
        raise ImportError(
            "Neither Rust nor C++ implementation of OSLOM is available. "
            "Please install with: pip install pyoslom"
        )

__version__ = "0.3.0"
__all__ = ["OSLOM"]

def get_implementation():
    """Return the currently active implementation ('rust' or 'cpp')."""
    return IMPLEMENTATION
