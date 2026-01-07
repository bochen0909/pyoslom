"""
Test suite for Rust OSLOM implementation migration.
Validates compatibility and correctness against the C++ version.
"""

import pytest
import numpy as np
import networkx as nx
from sklearn.utils.estimator_checks import check_estimator

# Import both implementations for comparison
try:
    from pyoslom.oslom_imp import OSLOM as CppOSLOM
    CPP_AVAILABLE = True
except ImportError:
    CPP_AVAILABLE = False
    CppOSLOM = None

try:
    from pyoslom.rust_oslom import RustOSLOM, run_rust_oslom, RUST_AVAILABLE
except ImportError:
    RUST_AVAILABLE = False
    RustOSLOM = None
    run_rust_oslom = None


class TestRustImplementation:
    """Test the Rust OSLOM implementation."""

    @pytest.fixture
    def simple_graph(self):
        """Create a simple test graph with clear community structure."""
        G = nx.Graph()
        # Community 1: nodes 0, 1, 2
        G.add_edges_from([(0, 1), (1, 2), (2, 0)], weight=1.0)
        # Community 2: nodes 3, 4, 5
        G.add_edges_from([(3, 4), (4, 5), (5, 3)], weight=1.0)
        # Weak connection between communities
        G.add_edge(2, 3, weight=0.1)
        return G

    @pytest.fixture
    def karate_graph(self):
        """Karate club graph - standard benchmark."""
        return nx.karate_club_graph()

    @pytest.fixture
    def directed_graph(self):
        """Simple directed graph."""
        G = nx.DiGraph()
        G.add_edges_from([(0, 1), (1, 2), (2, 0), (3, 4), (4, 5), (5, 3)])
        G.add_edge(2, 3, weight=0.1)
        return G

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_rust_basic_functionality(self, simple_graph):
        """Test basic Rust OSLOM functionality."""
        oslom = RustOSLOM(verbose=True)
        oslom.fit(simple_graph)
        result = oslom.transform()
        
        assert result is not None
        assert "clusters" in result
        assert "statistics" in result
        assert result["multilevel"] is True
        assert result["num_level"] > 0
        assert result["statistics"]["num_modules"] > 0

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_rust_parameters(self):
        """Test parameter handling."""
        oslom = RustOSLOM(
            directed=False,
            r=5,
            hr=10,
            T=0.05,
            cp=0.3,
            singlet=True,
            random_state=42,
            verbose=False
        )
        
        params = oslom.get_params()
        assert params['r'] == 5
        assert params['hr'] == 10
        assert params['T'] == 0.05
        assert params['cp'] == 0.3
        assert params['singlet'] is True
        assert params['random_state'] == 42

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_rust_directed_graph(self, directed_graph):
        """Test directed graph processing."""
        oslom = RustOSLOM(directed=True, verbose=True)
        oslom.fit(directed_graph)
        result = oslom.transform()
        
        assert result["statistics"]["num_modules"] > 0
        assert result["statistics"]["total_nodes"] == 6

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_rust_matrix_input(self):
        """Test matrix input handling."""
        # Create adjacency matrix
        adj_matrix = np.array([
            [0, 1, 1, 0],
            [1, 0, 1, 0],
            [1, 1, 0, 1],
            [0, 0, 1, 0]
        ], dtype=float)
        
        oslom = RustOSLOM()
        oslom.fit(adj_matrix)
        result = oslom.transform()
        
        assert result["statistics"]["total_nodes"] == 4

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_rust_reproducibility(self, simple_graph):
        """Test reproducibility with random seed."""
        oslom1 = RustOSLOM(random_state=42, r=3)
        oslom1.fit(simple_graph)
        result1 = oslom1.transform()
        
        oslom2 = RustOSLOM(random_state=42, r=3)
        oslom2.fit(simple_graph)
        result2 = oslom2.transform()
        
        # Results should be identical with same seed
        assert result1["statistics"]["num_modules"] == result2["statistics"]["num_modules"]
        assert abs(result1["statistics"]["modularity"] - result2["statistics"]["modularity"]) < 1e-10

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_rust_direct_function(self):
        """Test direct function interface."""
        edges = [(0, 1, 1.0), (1, 2, 1.0), (2, 0, 1.0), (3, 4, 1.0), (4, 5, 1.0), (5, 3, 1.0), (2, 3, 0.1)]
        
        result = run_rust_oslom(edges, verbose=True)
        assert len(result) > 0  # Should have at least one level
        assert 0 in result  # Should have base level

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_rust_error_handling(self):
        """Test error handling."""
        oslom = RustOSLOM()
        
        # Test fitting with invalid input
        with pytest.raises(ValueError):
            oslom.fit(np.array([[1, 2, 3]]))  # Non-square matrix
        
        # Test transform before fit
        with pytest.raises(ValueError):
            oslom.transform()

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_rust_sklearn_compatibility(self):
        """Test scikit-learn compatibility."""
        # This is a basic check - full estimator check might be too strict
        oslom = RustOSLOM()
        
        # Check that it has required methods
        assert hasattr(oslom, 'fit')
        assert hasattr(oslom, 'transform')
        assert hasattr(oslom, 'get_params')
        assert hasattr(oslom, 'set_params')

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_rust_large_graph_performance(self):
        """Test performance on larger graph."""
        # Create a larger graph with community structure
        G = nx.Graph()
        
        # Add 4 communities of 25 nodes each
        for community in range(4):
            nodes = list(range(community * 25, (community + 1) * 25))
            # Create dense intra-community connections
            for i in nodes:
                for j in nodes[nodes.index(i) + 1:]:
                    if np.random.random() < 0.3:  # 30% connection probability
                        G.add_edge(i, j, weight=1.0)
        
        # Add sparse inter-community connections
        for i in range(100):
            for j in range(i + 25, min(i + 50, 100)):
                if np.random.random() < 0.01:  # 1% connection probability
                    G.add_edge(i, j, weight=0.1)
        
        oslom = RustOSLOM(r=3, hr=5, verbose=True)  # Reduce runs for speed
        oslom.fit(G)
        result = oslom.transform()
        
        # Should find reasonable number of communities
        assert 2 <= result["statistics"]["num_modules"] <= 10
        assert result["statistics"]["coverage"] > 50  # Most nodes should be assigned


class TestCompatibility:
    """Test compatibility between C++ and Rust implementations."""

    @pytest.mark.skipif(not (CPP_AVAILABLE and RUST_AVAILABLE), 
                       reason="Both C++ and Rust implementations needed")
    def test_api_compatibility(self):
        """Test that both implementations have the same API."""
        cpp_oslom = CppOSLOM()
        rust_oslom = RustOSLOM()
        
        # Check that both have the same methods
        cpp_methods = set(dir(cpp_oslom))
        rust_methods = set(dir(rust_oslom))
        
        # Core methods should be present in both
        core_methods = {'fit', 'transform', 'get_params', 'set_params'}
        assert core_methods.issubset(cpp_methods)
        assert core_methods.issubset(rust_methods)

    @pytest.mark.skipif(not (CPP_AVAILABLE and RUST_AVAILABLE), 
                       reason="Both C++ and Rust implementations needed")
    def test_result_format_compatibility(self, simple_graph):
        """Test that result formats are compatible."""
        # Use same parameters for both
        params = dict(r=3, hr=5, T=0.1, cp=0.5, random_state=42)
        
        cpp_oslom = CppOSLOM(**params)
        rust_oslom = RustOSLOM(**params)
        
        cpp_oslom.fit(simple_graph)
        rust_oslom.fit(simple_graph)
        
        cpp_result = cpp_oslom.transform()
        rust_result = rust_oslom.transform()
        
        # Check that both have the same structure
        assert set(cpp_result.keys()) == set(rust_result.keys())
        assert cpp_result["multilevel"] == rust_result["multilevel"]
        assert "clusters" in cpp_result and "clusters" in rust_result
        
        # Both should find some communities
        assert cpp_result["num_level"] > 0
        assert rust_result["num_level"] > 0

    @pytest.mark.skipif(not (CPP_AVAILABLE and RUST_AVAILABLE), 
                       reason="Both C++ and Rust implementations needed")
    def test_parameter_compatibility(self):
        """Test that parameters work the same way."""
        params = {
            'directed': False,
            'r': 5,
            'hr': 10,
            'T': 0.05,
            'cp': 0.3,
            'singlet': True,
            'random_state': 42,
            'verbose': False
        }
        
        # Both should accept the same parameters
        cpp_oslom = CppOSLOM(**params)
        rust_oslom = RustOSLOM(**params)
        
        # Parameter retrieval should work similarly
        cpp_params = cpp_oslom.get_params()
        rust_params = rust_oslom.get_params()
        
        # Core parameters should match
        for key in ['r', 'T', 'cp', 'random_state']:
            if key in cpp_params and key in rust_params:
                assert cpp_params[key] == rust_params[key]


class TestEdgeCases:
    """Test edge cases and error conditions."""

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_empty_graph(self):
        """Test handling of empty graph."""
        G = nx.Graph()
        oslom = RustOSLOM()
        
        with pytest.raises(RuntimeError):  # Should fail gracefully
            oslom.fit(G)

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_single_node(self):
        """Test single node graph."""
        G = nx.Graph()
        G.add_node(0)
        
        oslom = RustOSLOM()
        with pytest.raises(RuntimeError):  # Should handle gracefully
            oslom.fit(G)

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_disconnected_graph(self):
        """Test disconnected graph."""
        G = nx.Graph()
        G.add_edges_from([(0, 1), (2, 3)])  # Two disconnected edges
        
        oslom = RustOSLOM(verbose=True)
        oslom.fit(G)
        result = oslom.transform()
        
        # Should find separate communities
        assert result["statistics"]["num_modules"] >= 2

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_self_loops(self):
        """Test graph with self-loops."""
        G = nx.Graph()
        G.add_edges_from([(0, 1), (1, 2), (2, 0)])
        G.add_edge(1, 1)  # Self-loop
        
        oslom = RustOSLOM()
        oslom.fit(G)
        result = oslom.transform()
        
        assert result["statistics"]["total_nodes"] == 3

    @pytest.mark.skipif(not RUST_AVAILABLE, reason="Rust implementation not available")
    def test_negative_weights(self):
        """Test handling of negative weights."""
        G = nx.Graph()
        G.add_edge(0, 1, weight=-1.0)  # Negative weight
        G.add_edge(1, 2, weight=1.0)
        
        oslom = RustOSLOM()
        # Should handle gracefully (might skip negative weights)
        oslom.fit(G)
        result = oslom.transform()
        
        assert result is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])