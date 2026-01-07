# OSLOM C++ to Rust Migration Plan

## Overview
This document outlines the complete migration strategy from the existing C++ OSLOM implementation to a modern Rust implementation with Python bindings using PyO3.

## Current State Analysis

### Existing C++ Architecture Issues
- **Poor code organization**: Monolithic files with mixed responsibilities
- **Memory management**: Manual memory management with potential leaks
- **Build complexity**: Complex pybind11 setup with platform-specific compilation
- **Code quality**: Legacy C++ patterns, global variables, poor error handling
- **Maintainability**: Difficult to extend or modify safely

### Current Python API
- Scikit-learn compatible interface (`BaseEstimator`, `TransformerMixin`, `ClusterMixin`)
- Supports both directed and undirected graphs
- NetworkX integration
- Hierarchical clustering output
- Parameters: `r`, `hr`, `T`, `singlet`, `cp`, `random_state`, `verbose`

## Migration Strategy

### Phase 1: Project Structure Setup
1. **Create Rust workspace structure**
   ```
   rust/
   ├── Cargo.toml (workspace)
   ├── oslom-core/          # Core algorithm library
   ├── oslom-python/        # Python bindings
   └── oslom-cli/           # Optional CLI tool
   ```

2. **Update Python project configuration**
   - Replace pybind11 with PyO3/maturin
   - Update pyproject.toml for Rust builds
   - Maintain backward compatibility in Python API

### Phase 2: Core Algorithm Implementation
1. **Network representation**
   - Efficient graph data structures using `petgraph` or custom implementation
   - Support for weighted/unweighted, directed/undirected graphs
   - Memory-efficient adjacency representations

2. **Module detection algorithms**
   - Louvain algorithm implementation
   - Statistical significance testing (CUP test)
   - Hierarchical community detection

3. **Set operations and overlap detection**
   - Efficient set intersection/union operations
   - Module overlap analysis
   - Community merging logic

### Phase 3: Python Integration
1. **PyO3 bindings**
   - Expose core functionality to Python
   - Handle NetworkX graph conversion
   - Maintain existing API compatibility

2. **Error handling**
   - Proper Rust error types
   - Python exception mapping
   - Graceful failure modes

### Phase 4: Testing and Validation
1. **Unit tests** for all Rust components
2. **Integration tests** comparing C++ vs Rust outputs
3. **Performance benchmarks**
4. **Python API compatibility tests**

## Implementation Details

### Rust Dependencies
```toml
[dependencies]
petgraph = "0.6"           # Graph data structures
ndarray = "0.15"           # Numerical arrays
rand = "0.8"               # Random number generation
rayon = "1.7"              # Parallel processing
thiserror = "1.0"          # Error handling
serde = { version = "1.0", features = ["derive"] }

[dependencies.pyo3]
version = "0.20"
features = ["extension-module"]
```

### Key Rust Modules

#### 1. Graph Representation (`graph.rs`)
```rust
pub struct Network {
    nodes: Vec<NodeId>,
    edges: Vec<Edge>,
    adjacency: HashMap<NodeId, Vec<(NodeId, f64)>>,
    directed: bool,
}

pub trait NetworkOps {
    fn add_edge(&mut self, from: NodeId, to: NodeId, weight: f64);
    fn neighbors(&self, node: NodeId) -> &[(NodeId, f64)];
    fn degree(&self, node: NodeId) -> usize;
    fn subgraph(&self, nodes: &[NodeId]) -> Network;
}
```

#### 2. Module Collection (`modules.rs`)
```rust
pub struct ModuleCollection {
    modules: HashMap<ModuleId, Vec<NodeId>>,
    memberships: HashMap<NodeId, HashSet<ModuleId>>,
    scores: HashMap<ModuleId, f64>,
}

impl ModuleCollection {
    pub fn insert_module(&mut self, nodes: Vec<NodeId>, score: f64) -> ModuleId;
    pub fn check_overlap(&self, m1: ModuleId, m2: ModuleId) -> f64;
    pub fn merge_modules(&mut self, modules: &[ModuleId]) -> ModuleId;
}
```

#### 3. OSLOM Algorithm (`oslom.rs`)
```rust
pub struct OslomConfig {
    pub r: usize,
    pub hr: usize,
    pub threshold: f64,
    pub cp: f64,
    pub find_singletons: bool,
    pub random_seed: Option<u64>,
}

pub struct OslomResult {
    pub hierarchical_modules: Vec<ModuleCollection>,
    pub statistics: ClusteringStats,
}

pub fn run_oslom(network: &Network, config: &OslomConfig) -> Result<OslomResult, OslomError>;
```

#### 4. Python Bindings (`python.rs`)
```rust
#[pyclass]
pub struct PyOslom {
    config: OslomConfig,
    result: Option<OslomResult>,
}

#[pymethods]
impl PyOslom {
    #[new]
    pub fn new(/* parameters */) -> Self;
    
    pub fn fit(&mut self, edges: Vec<(usize, usize, f64)>) -> PyResult<()>;
    
    pub fn get_clusters(&self) -> PyResult<HashMap<usize, Vec<Vec<usize>>>>;
}
```

### Build System Migration

#### New pyproject.toml
```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "pyoslom"
requires-python = ">=3.8"
dependencies = [
    "scikit-learn>=1.0",
    "networkx>=2.5",
    "numpy>=1.20",
]

[tool.maturin]
python-source = "python"
module-name = "pyoslom._rust"
```

## Migration Timeline

### Week 1-2: Setup and Planning
- [ ] Create Rust workspace structure
- [ ] Set up development environment
- [ ] Create initial Cargo.toml files
- [ ] Set up CI/CD for Rust builds

### Week 3-4: Core Data Structures
- [ ] Implement Network struct and basic operations
- [ ] Implement ModuleCollection
- [ ] Add comprehensive unit tests
- [ ] Benchmark against C++ version

### Week 5-6: Louvain Algorithm
- [ ] Port Louvain optimization algorithm
- [ ] Implement modularity calculations
- [ ] Add parallel processing support
- [ ] Validate against C++ implementation

### Week 7-8: Statistical Testing
- [ ] Implement CUP significance testing
- [ ] Port hypergeometric distribution calculations
- [ ] Add statistical validation functions
- [ ] Performance optimization

### Week 9-10: Hierarchical Processing
- [ ] Implement hierarchical community detection
- [ ] Add module merging and overlap detection
- [ ] Implement union checking algorithms
- [ ] Integration testing

### Week 11-12: Python Bindings
- [ ] Create PyO3 bindings
- [ ] Implement NetworkX integration
- [ ] Maintain API compatibility
- [ ] Add Python-side error handling

### Week 13-14: Testing and Optimization
- [ ] Comprehensive test suite
- [ ] Performance benchmarking
- [ ] Memory usage optimization
- [ ] Documentation updates

## Success Criteria

### Functional Requirements
- [ ] 100% API compatibility with existing Python interface
- [ ] Identical or better clustering results compared to C++ version
- [ ] Support for all existing parameters and options
- [ ] Proper error handling and edge cases

### Performance Requirements
- [ ] At least equivalent performance to C++ version
- [ ] Better memory efficiency
- [ ] Improved parallel processing capabilities
- [ ] Faster build times

### Quality Requirements
- [ ] >90% test coverage
- [ ] Memory safety (no segfaults)
- [ ] Proper error propagation
- [ ] Clean, maintainable code structure

## Risk Mitigation

### Technical Risks
1. **Performance regression**: Continuous benchmarking against C++ version
2. **API compatibility**: Comprehensive integration tests
3. **Complex algorithm porting**: Incremental implementation with validation
4. **Build system issues**: Early testing on multiple platforms

### Timeline Risks
1. **Underestimated complexity**: Buffer time in schedule
2. **Dependency issues**: Early evaluation of Rust ecosystem
3. **Integration challenges**: Parallel development of tests

## Post-Migration Benefits

### Developer Experience
- **Memory safety**: Eliminate segfaults and memory leaks
- **Better error handling**: Rust's Result type for proper error propagation
- **Improved maintainability**: Clear module boundaries and ownership
- **Modern tooling**: Cargo, rustfmt, clippy for better development workflow

### Performance
- **Zero-cost abstractions**: Better optimization opportunities
- **Parallel processing**: Built-in support with rayon
- **Memory efficiency**: Better control over allocations
- **Cross-platform**: Consistent behavior across platforms

### Ecosystem
- **Better packaging**: Simplified build process with maturin
- **Documentation**: Built-in documentation with rustdoc
- **Testing**: Integrated testing framework
- **Community**: Access to Rust's growing scientific computing ecosystem

## Conclusion

This migration will modernize the OSLOM implementation, improve maintainability, and provide a solid foundation for future enhancements while maintaining full backward compatibility with the existing Python API.