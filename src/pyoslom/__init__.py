"""
PyOSLOM - Python wrapper for OSLOM algorithm

This package provides a Rust implementation of the OSLOM
(Order Statistics Local Optimization Method) algorithm for community detection.
"""

from .rust_oslom import RustOSLOM as OSLOM

__version__ = "0.3.0"
__all__ = ["OSLOM"]

def get_implementation():
    """Return the currently active implementation."""
    return "rust"
