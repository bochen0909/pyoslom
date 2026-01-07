use crate::error::{OslomError, Result};
use crate::graph::{Network, NodeId};
use crate::modules::{ModuleCollection, Module};
use std::collections::HashMap;

pub struct StatisticalTester<'a> {
    network: &'a Network,
    threshold: f64,
    log_table: LogFactorialTable,
}

impl<'a> StatisticalTester<'a> {
    pub fn new(network: &'a Network, threshold: f64) -> Self {
        Self {
            network,
            threshold,
            log_table: LogFactorialTable::new(10000), // Pre-compute up to 10k
        }
    }

    pub fn evaluate_modules(&mut self, modules: &ModuleCollection) -> Result<ModuleCollection> {
        let mut evaluated = ModuleCollection::new();

        for module in modules.modules() {
            let score = self.evaluate_module(&module.nodes)?;
            if score >= self.threshold {
                evaluated.insert_module(module.nodes.clone(), score);
            }
        }

        Ok(evaluated)
    }

    pub fn evaluate_module(&mut self, nodes: &[NodeId]) -> Result<f64> {
        if nodes.is_empty() {
            return Ok(0.0);
        }

        // Calculate internal and external edges
        let (internal_edges, external_edges, total_degree) = self.calculate_edge_statistics(nodes)?;
        
        // Perform CUP (Clustering with Uncorrelated Pairs) test
        self.cup_test(internal_edges, external_edges, total_degree, nodes.len())
    }

    fn calculate_edge_statistics(&self, nodes: &[NodeId]) -> Result<(usize, usize, usize)> {
        let node_set: std::collections::HashSet<_> = nodes.iter().copied().collect();
        let mut internal_edges = 0;
        let mut external_edges = 0;
        let mut total_degree = 0;

        for &node in nodes {
            if let Some(neighbors) = self.network.neighbors(node) {
                total_degree += neighbors.len();
                
                for &(neighbor, _weight) in neighbors {
                    if node_set.contains(&neighbor) {
                        if node <= neighbor { // Avoid double counting
                            internal_edges += 1;
                        }
                    } else {
                        external_edges += 1;
                    }
                }
            }
        }

        Ok((internal_edges, external_edges, total_degree))
    }

    fn cup_test(&mut self, internal: usize, external: usize, total_degree: usize, module_size: usize) -> Result<f64> {
        if total_degree == 0 {
            return Ok(0.0);
        }

        let total_edges = self.network.edge_count();
        let total_nodes = self.network.node_count();
        
        if total_edges == 0 || total_nodes <= 1 {
            return Ok(0.0);
        }

        // Expected number of internal edges under null hypothesis
        let expected_internal = self.expected_internal_edges(module_size, total_degree, total_edges, total_nodes)?;
        
        // Calculate p-value using hypergeometric distribution
        let p_value = self.hypergeometric_tail_probability(internal, total_degree, expected_internal)?;
        
        // Convert p-value to significance score (negative log p-value)
        if p_value > 0.0 {
            Ok(-p_value.ln())
        } else {
            Ok(f64::INFINITY)
        }
    }

    fn expected_internal_edges(&self, module_size: usize, total_degree: usize, total_edges: usize, total_nodes: usize) -> Result<f64> {
        if total_nodes <= 1 {
            return Ok(0.0);
        }

        // Expected internal edges based on random null model
        let p_internal = (module_size * (module_size - 1)) as f64 / (total_nodes * (total_nodes - 1)) as f64;
        Ok(total_degree as f64 * p_internal)
    }

    fn hypergeometric_tail_probability(&mut self, observed: usize, total_degree: usize, expected: f64) -> Result<f64> {
        if expected <= 0.0 || total_degree == 0 {
            return Ok(1.0);
        }

        // Approximate using normal distribution for large numbers
        if total_degree > 100 {
            return self.normal_approximation(observed, expected, total_degree);
        }

        // Exact hypergeometric calculation for smaller numbers
        let mut p_value = 0.0;
        let max_possible = total_degree.min(observed + 50); // Limit computation

        for k in observed..=max_possible {
            let prob = self.hypergeometric_probability(k, total_degree, expected)?;
            p_value += prob;
            
            // Early termination if probability becomes negligible
            if prob < 1e-10 {
                break;
            }
        }

        Ok(p_value.min(1.0))
    }

    fn hypergeometric_probability(&mut self, k: usize, n: usize, expected: f64) -> Result<f64> {
        // Approximate hypergeometric with binomial for simplicity
        let p = expected / n as f64;
        self.binomial_probability(k, n, p)
    }

    fn binomial_probability(&mut self, k: usize, n: usize, p: f64) -> Result<f64> {
        if p <= 0.0 || p >= 1.0 || k > n {
            return Ok(0.0);
        }

        // Use log space to avoid overflow
        let log_prob = self.log_table.log_binomial_coefficient(n, k)
            + k as f64 * p.ln()
            + (n - k) as f64 * (1.0 - p).ln();

        Ok(log_prob.exp())
    }

    fn normal_approximation(&self, observed: usize, expected: f64, total_degree: usize) -> Result<f64> {
        let variance = expected * (1.0 - expected / total_degree as f64);
        if variance <= 0.0 {
            return Ok(1.0);
        }

        let std_dev = variance.sqrt();
        let z_score = (observed as f64 - expected) / std_dev;

        // Complementary error function approximation
        Ok(0.5 * (1.0 - erf(z_score / std::f64::consts::SQRT_2)))
    }
}

struct LogFactorialTable {
    table: Vec<f64>,
}

impl LogFactorialTable {
    fn new(max_n: usize) -> Self {
        let mut table = vec![0.0; max_n + 1];
        
        for i in 1..=max_n {
            table[i] = table[i - 1] + (i as f64).ln();
        }

        Self { table }
    }

    fn log_factorial(&self, n: usize) -> f64 {
        if n < self.table.len() {
            self.table[n]
        } else {
            // Use Stirling's approximation for large n
            let n_f = n as f64;
            n_f * n_f.ln() - n_f + 0.5 * (2.0 * std::f64::consts::PI * n_f).ln()
        }
    }

    fn log_binomial_coefficient(&self, n: usize, k: usize) -> f64 {
        if k > n {
            return f64::NEG_INFINITY;
        }
        if k == 0 || k == n {
            return 0.0;
        }

        self.log_factorial(n) - self.log_factorial(k) - self.log_factorial(n - k)
    }
}

// Error function approximation
fn erf(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::NetworkBuilder;
    use crate::modules::ModuleCollection;

    #[test]
    fn test_statistical_evaluation() {
        let mut builder = NetworkBuilder::new(false);
        // Create a clear community structure
        builder.add_edge(0, 1, 1.0)
               .add_edge(1, 2, 1.0)
               .add_edge(2, 0, 1.0)
               .add_edge(3, 4, 1.0)
               .add_edge(4, 5, 1.0)
               .add_edge(5, 3, 1.0)
               .add_edge(2, 3, 0.1); // Weak inter-community edge

        let network = builder.build();
        let mut tester = StatisticalTester::new(&network, 0.1);

        // Test a good community
        let score1 = tester.evaluate_module(&[0, 1, 2]).unwrap();
        assert!(score1 > 0.0);

        // Test a bad community (random nodes)
        let score2 = tester.evaluate_module(&[0, 3]).unwrap();
        assert!(score1 > score2); // Good community should score higher
    }

    #[test]
    fn test_empty_module() {
        let network = NetworkBuilder::new(false).build();
        let mut tester = StatisticalTester::new(&network, 0.1);
        
        let score = tester.evaluate_module(&[]).unwrap();
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_log_factorial_table() {
        let table = LogFactorialTable::new(10);
        
        // Test known values
        assert!((table.log_factorial(0) - 0.0).abs() < 1e-10);
        assert!((table.log_factorial(1) - 0.0).abs() < 1e-10);
        assert!((table.log_factorial(2) - 2.0_f64.ln()).abs() < 1e-10);
        assert!((table.log_factorial(3) - 6.0_f64.ln()).abs() < 1e-10);
    }

    #[test]
    fn test_binomial_coefficient() {
        let table = LogFactorialTable::new(10);
        
        // C(5,2) = 10
        let log_coeff = table.log_binomial_coefficient(5, 2);
        assert!((log_coeff.exp() - 10.0).abs() < 1e-10);
        
        // C(4,0) = 1
        let log_coeff = table.log_binomial_coefficient(4, 0);
        assert!((log_coeff.exp() - 1.0).abs() < 1e-10);
    }
}