#!/usr/bin/env python3
"""
Build script for PyOSLOM Rust implementation.
This script helps build the Rust extension using maturin.
"""

import subprocess
import sys
import os
from pathlib import Path

def run_command(cmd, cwd=None):
    """Run a command and return the result."""
    print(f"Running: {' '.join(cmd)}")
    result = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Error: {result.stderr}")
        sys.exit(1)
    return result.stdout

def main():
    """Main build function."""
    root_dir = Path(__file__).parent
    rust_dir = root_dir / "rust"
    
    print("Building PyOSLOM Rust implementation...")
    
    # Check if Rust is installed
    try:
        run_command(["rustc", "--version"])
        print("✓ Rust is installed")
    except FileNotFoundError:
        print("❌ Rust is not installed. Please install from https://rustup.rs/")
        sys.exit(1)
    
    # Check if maturin is installed
    try:
        run_command(["maturin", "--version"])
        print("✓ Maturin is installed")
    except FileNotFoundError:
        print("Installing maturin...")
        run_command([sys.executable, "-m", "pip", "install", "maturin"])
    
    # Build the Rust extension
    print("Building Rust extension...")
    os.chdir(root_dir)
    
    if len(sys.argv) > 1 and sys.argv[1] == "develop":
        # Development build
        run_command(["maturin", "develop", "--release"])
        print("✓ Development build complete")
    elif len(sys.argv) > 1 and sys.argv[1] == "wheel":
        # Build wheel
        run_command(["maturin", "build", "--release"])
        print("✓ Wheel build complete")
    else:
        # Default: development build
        run_command(["maturin", "develop", "--release"])
        print("✓ Development build complete")
    
    print("\nBuild successful! You can now use:")
    print("  python -c 'import pyoslom; print(pyoslom.get_implementation())'")

if __name__ == "__main__":
    main()