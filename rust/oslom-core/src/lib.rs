//! OSLOM (Order Statistics Local Optimization Method) Core Library
//! 
//! This crate provides a modern Rust implementation of the OSLOM algorithm
//! for community detection in networks.

pub mod error;
pub mod graph;
pub mod modules;
pub mod oslom;
pub mod statistics;
pub mod louvain;

pub use error::{OslomError, Result};
pub use graph::{Network, NetworkBuilder, NodeId, Edge};
pub use modules::{ModuleCollection, ModuleId};
pub use oslom::{OslomConfig, OslomResult, run_oslom};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let mut builder = NetworkBuilder::new(false);
        builder.add_edge(0, 1, 1.0);
        builder.add_edge(1, 2, 1.0);
        builder.add_edge(2, 3, 1.0);
        
        let network = builder.build();
        assert_eq!(network.node_count(), 4);
        assert_eq!(network.edge_count(), 3);
    }
}