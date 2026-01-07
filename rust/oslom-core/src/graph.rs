use std::collections::{HashMap, HashSet};
use crate::error::{OslomError, Result};

pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct Network {
    nodes: HashSet<NodeId>,
    adjacency: HashMap<NodeId, Vec<(NodeId, f64)>>,
    directed: bool,
    total_weight: f64,
}

impl Network {
    pub fn new(directed: bool) -> Self {
        Self {
            nodes: HashSet::new(),
            adjacency: HashMap::new(),
            directed,
            total_weight: 0.0,
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        if self.directed {
            self.adjacency.values().map(|v| v.len()).sum()
        } else {
            self.adjacency.values().map(|v| v.len()).sum::<usize>() / 2
        }
    }

    pub fn is_directed(&self) -> bool {
        self.directed
    }

    pub fn total_weight(&self) -> f64 {
        self.total_weight
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.iter().copied()
    }

    pub fn neighbors(&self, node: NodeId) -> Option<&[(NodeId, f64)]> {
        self.adjacency.get(&node).map(|v| v.as_slice())
    }

    pub fn degree(&self, node: NodeId) -> usize {
        self.adjacency.get(&node).map(|v| v.len()).unwrap_or(0)
    }

    pub fn weighted_degree(&self, node: NodeId) -> f64 {
        self.adjacency
            .get(&node)
            .map(|neighbors| neighbors.iter().map(|(_, w)| w).sum())
            .unwrap_or(0.0)
    }

    pub fn subgraph(&self, nodes: &[NodeId]) -> Network {
        let node_set: HashSet<_> = nodes.iter().copied().collect();
        let mut subgraph = Network::new(self.directed);
        
        for &node in nodes {
            if let Some(neighbors) = self.adjacency.get(&node) {
                for &(neighbor, weight) in neighbors {
                    if node_set.contains(&neighbor) {
                        subgraph.add_edge_internal(node, neighbor, weight);
                    }
                }
            }
        }
        
        subgraph
    }

    fn add_edge_internal(&mut self, from: NodeId, to: NodeId, weight: f64) {
        self.nodes.insert(from);
        self.nodes.insert(to);
        
        self.adjacency.entry(from).or_default().push((to, weight));
        
        if !self.directed && from != to {
            self.adjacency.entry(to).or_default().push((from, weight));
        }
        
        self.total_weight += weight;
    }
}

pub struct NetworkBuilder {
    network: Network,
}

impl NetworkBuilder {
    pub fn new(directed: bool) -> Self {
        Self {
            network: Network::new(directed),
        }
    }

    pub fn add_edge(&mut self, from: NodeId, to: NodeId, weight: f64) -> &mut Self {
        if weight <= 0.0 {
            // Skip invalid weights
            return self;
        }
        
        self.network.add_edge_internal(from, to, weight);
        self
    }

    pub fn build(self) -> Network {
        self.network
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undirected_network() {
        let mut builder = NetworkBuilder::new(false);
        builder.add_edge(0, 1, 1.0)
               .add_edge(1, 2, 2.0)
               .add_edge(2, 0, 1.5);
        
        let network = builder.build();
        
        assert_eq!(network.node_count(), 3);
        assert_eq!(network.edge_count(), 3);
        assert!(!network.is_directed());
        
        // Check degrees
        assert_eq!(network.degree(0), 2);
        assert_eq!(network.degree(1), 2);
        assert_eq!(network.degree(2), 2);
        
        // Check weighted degrees
        assert_eq!(network.weighted_degree(0), 2.5);
        assert_eq!(network.weighted_degree(1), 3.0);
        assert_eq!(network.weighted_degree(2), 3.5);
    }

    #[test]
    fn test_directed_network() {
        let mut builder = NetworkBuilder::new(true);
        builder.add_edge(0, 1, 1.0)
               .add_edge(1, 2, 2.0);
        
        let network = builder.build();
        
        assert_eq!(network.node_count(), 3);
        assert_eq!(network.edge_count(), 2);
        assert!(network.is_directed());
        
        assert_eq!(network.degree(0), 1);
        assert_eq!(network.degree(1), 1);
        assert_eq!(network.degree(2), 0);
    }

    #[test]
    fn test_subgraph() {
        let mut builder = NetworkBuilder::new(false);
        builder.add_edge(0, 1, 1.0)
               .add_edge(1, 2, 1.0)
               .add_edge(2, 3, 1.0)
               .add_edge(0, 3, 1.0);
        
        let network = builder.build();
        let subgraph = network.subgraph(&[0, 1, 2]);
        
        assert_eq!(subgraph.node_count(), 3);
        assert_eq!(subgraph.edge_count(), 2); // Only edges within the subgraph
    }
}