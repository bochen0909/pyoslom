"""
Rust-based OSLOM implementation with scikit-learn compatible interface.
This module provides a drop-in replacement for the C++ implementation.
"""

import numpy as np
from sklearn.base import BaseEstimator, TransformerMixin, ClusterMixin
from networkx import Graph, DiGraph
import networkx as nx

try:
    from pyoslom._rust import (
        PyOslom as RustOslom, 
        run_oslom_direct, 
        set_verbose
    )
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False
    RustOslom = None
    run_oslom_direct = None
    set_verbose = None


class RustOSLOM(ClusterMixin, BaseEstimator):
    """
    Rust-based OSLOM implementation with scikit-learn compatible interface.
    
    This is a drop-in replacement for the C++ OSLOM implementation with
    improved performance, memory safety, and maintainability.
    
    Parameters
    ----------
    directed : bool, default=False
        Whether the graph is directed
    r : int, default=10
        Number of runs for the first hierarchical level
    hr : int, default=50
        Number of runs for higher hierarchical levels
    T : float, default=0.1
        Statistical significance threshold
    cp : float, default=0.5
        Coverage parameter for module unions
    singlet : bool, default=False
        Whether to find singleton nodes
    random_state : int or None, default=None
        Random seed for reproducibility
    verbose : bool, default=False
        Verbosity mode
    
    Attributes
    ----------
    cluster_ : dict
        Clustering result with hierarchical modules and statistics.
        Available after calling fit().
    labels_ : ndarray of shape (n_samples,)
        Cluster labels for each point. Available after calling fit().
    n_clusters_ : int
        Number of clusters found. Available after calling fit().
    """

    def __init__(
        self,
        directed=False,
        r=None,
        hr=None,
        T=None,
        singlet=False,
        cp=None,
        random_state=None,
        verbose=False,
    ):
        if not RUST_AVAILABLE:
            raise ImportError(
                "Rust OSLOM implementation not available. "
                "Please install with: pip install pyoslom or build from source."
            )

        self.directed = directed
        self.r = r if r is not None else 10
        self.hr = hr if hr is not None else 50
        self.T = T if T is not None else 0.1
        self.singlet = singlet
        self.cp = cp if cp is not None else 0.5
        self.random_state = random_state
        self.verbose = verbose
        
        # sklearn-compatible attributes
        self.cluster_ = None
        self.labels_ = None
        self.n_clusters_ = None
        self._is_fitted = False
        
        # Initialize Rust OSLOM instance
        self._rust_oslom = None
        self._create_rust_instance()

    def _create_rust_instance(self):
        """Create or recreate the Rust OSLOM instance."""
        self._rust_oslom = RustOslom(
            directed=self.directed,
            r=self.r,
            hr=self.hr,
            threshold=self.T,
            cp=self.cp,
            find_singletons=self.singlet,
            random_seed=self.random_state,
            verbose=self.verbose,
        )

    def fit(self, X, y=None):
        """
        Compute OSLOM clustering.
        
        Parameters
        ----------
        X : {networkx Graph, networkx DiGraph, ndarray, sparse matrix} 
            of shape (n_samples, n_samples)
            Training instances to cluster.
        y : Ignored
            Not used, present here for API consistency by convention.
            
        Returns
        -------
        self
            Fitted estimator.
        """
        # Reset state
        self.cluster_ = None
        self.labels_ = None
        self.n_clusters_ = None
        self._is_fitted = False
        
        # Convert input to NetworkX graph if needed
        if not isinstance(X, (Graph, DiGraph)):
            X = self._convert_to_networkx(X)

        # Validate graph type
        self._validate_graph_type(X)

        # Convert NetworkX graph to edge list
        edges, node_mapping = self._extract_edges(X)

        if self.verbose:
            print(f"Processing graph with {X.number_of_nodes()} nodes "
                  f"and {X.number_of_edges()} edges")

        # Run Rust OSLOM
        try:
            self._rust_oslom.fit(edges)
            clusters = self._rust_oslom.get_clusters()
            statistics = self._rust_oslom.get_statistics()
            
            # Convert to format compatible with original implementation
            self.cluster_ = {
                "multilevel": True,
                "num_level": int(statistics["num_levels"]),
                "max_level": max(clusters.keys()) if clusters else 0,
                "params": self._get_params_list(),
                "clusters": clusters,
                "statistics": statistics,
            }
            
            # Create sklearn-compatible labels
            self.labels_ = self._create_labels(clusters, node_mapping, X)
            self.n_clusters_ = len(set(self.labels_)) - (1 if -1 in self.labels_ else 0)
            
            self._is_fitted = True
            
            if self.verbose:
                print(f"Found {statistics['num_modules']} modules at base level")
                print(f"Modularity: {statistics['modularity']:.4f}")
                print(f"Coverage: {statistics['coverage']}/"
                      f"{statistics['total_nodes']} nodes")
                
        except Exception as e:
            raise RuntimeError(f"OSLOM clustering failed: {e}")

        return self

    def _convert_to_networkx(self, X):
        """Convert matrix input to NetworkX graph."""
        if len(X.shape) != 2 or X.shape[0] != X.shape[1]:
            raise ValueError("Input must be a symmetric matrix")
            
        if isinstance(X, np.ndarray):
            if self.directed:
                return nx.convert_matrix.from_numpy_array(
                    X, create_using=nx.DiGraph
                )
            else:
                return nx.convert_matrix.from_numpy_array(
                    X, create_using=nx.Graph
                )
        else:
            if self.directed:
                return nx.convert_matrix.from_scipy_sparse_array(
                    X, create_using=nx.DiGraph
                )
            else:
                return nx.convert_matrix.from_scipy_sparse_array(
                    X, create_using=nx.Graph
                )

    def _validate_graph_type(self, X):
        """Validate that graph type matches directed parameter."""
        if isinstance(X, Graph) and not isinstance(X, DiGraph):
            if self.directed:
                raise ValueError("Undirected graph provided but directed=True")
        elif isinstance(X, DiGraph):
            if not self.directed:
                raise ValueError("Directed graph provided but directed=False")
        else:
            raise ValueError("Invalid graph type")

    def _create_node_mapping(self, X):
        """Create mapping from node IDs to integer indices."""
        nodes = list(X.nodes())
        return {node: idx for idx, node in enumerate(nodes)}

    def _extract_edges(self, X):
        """Extract edges from NetworkX graph with integer node mapping."""
        node_mapping = self._create_node_mapping(X)
        edges = []
        for u, v, data in X.edges(data=True):
            weight = data.get('weight', 1.0)
            u_idx = node_mapping[u]
            v_idx = node_mapping[v]
            edges.append((u_idx, v_idx, weight))
        return edges, node_mapping

    def _create_labels(self, clusters, node_mapping, X):
        """Create sklearn-compatible cluster labels."""
        if not clusters:
            return np.full(X.number_of_nodes(), -1)
            
        # Create reverse mapping from index to original node ID
        idx_to_node = {idx: node for node, idx in node_mapping.items()}
        
        # Use base level (level 0) clusters
        base_clusters = clusters.get(0, {})
        labels = np.full(X.number_of_nodes(), -1)
        
        for cluster_id, node_indices in base_clusters.items():
            for node_idx in node_indices:
                if node_idx < len(labels):
                    labels[node_idx] = cluster_id
                    
        return labels

    def fit_predict(self, X, y=None):
        """
        Compute cluster centers and predict cluster index for each sample.
        
        Parameters
        ----------
        X : {networkx Graph, networkx DiGraph, ndarray, sparse matrix}
            Input data.
        y : Ignored
            Not used, present here for API consistency by convention.
            
        Returns
        -------
        labels : ndarray of shape (n_samples,)
            Index of the cluster each sample belongs to.
        """
        return self.fit(X, y).labels_

    def transform(self, X=None):
        """
        Return the clustering result.
        
        For OSLOM, transform doesn't process new data but returns the 
        clustering result from the fitted data. This maintains backward
        compatibility with the original implementation.
        
        Parameters
        ----------
        X : None
            Must be None. OSLOM doesn't transform new data.
            
        Returns
        -------
        dict
            Clustering result with hierarchical modules and statistics.
            
        Raises
        ------
        ValueError
            If X is not None or if the model hasn't been fitted yet.
        """
        if X is not None:
            raise ValueError(
                "OSLOM.transform() does not accept new data. "
                "It returns the clustering result from fitted data. "
                "Call with X=None or no arguments."
            )
            
        if not self._is_fitted:
            raise ValueError(
                "This RustOSLOM instance is not fitted yet. Call 'fit' first."
            )
        return self.cluster_

    def fit_transform(self, X, y=None):
        """
        Fit the model and return the clustering result.
        
        Parameters
        ----------
        X : {networkx Graph, networkx DiGraph, ndarray, sparse matrix}
            Input data.
        y : Ignored
            Not used, present here for API consistency by convention.
            
        Returns
        -------
        dict
            Clustering result with hierarchical modules and statistics.
        """
        return self.fit(X, y).transform()

    def _get_params_list(self):
        """Get parameters in list format for compatibility."""
        params = ["oslom_dir" if self.directed else "oslom_undir", "-w"]
        params.extend(["-r", str(self.r)])
        params.extend(["-hr", str(self.hr)])
        params.extend(["-T", str(self.T)])
        params.extend(["-cp", str(self.cp)])
        
        if self.random_state is not None:
            params.extend(["-seed", str(self.random_state)])
        if self.singlet:
            params.append("-singlet")
            
        return params

    def get_params(self, deep=True):
        """Get parameters for this estimator."""
        return {
            'directed': self.directed,
            'r': self.r,
            'hr': self.hr,
            'T': self.T,
            'singlet': self.singlet,
            'cp': self.cp,
            'random_state': self.random_state,
            'verbose': self.verbose,
        }

    def set_params(self, **params):
        """Set the parameters of this estimator."""
        valid_params = {
            'directed', 'r', 'hr', 'T', 'singlet', 'cp', 
            'random_state', 'verbose'
        }
        
        for key, value in params.items():
            if key not in valid_params:
                raise ValueError(f"Invalid parameter {key}")
            setattr(self, key, value)
        
        # Apply defaults for None values
        self.r = self.r if self.r is not None else 10
        self.hr = self.hr if self.hr is not None else 50
        self.T = self.T if self.T is not None else 0.1
        self.cp = self.cp if self.cp is not None else 0.5
        
        # Recreate Rust instance with new parameters
        self._create_rust_instance()
        
        return self

    def __repr__(self):
        return (
            f"RustOSLOM(directed={self.directed}, r={self.r}, hr={self.hr}, "
            f"T={self.T}, cp={self.cp}, singlet={self.singlet}, "
            f"random_state={self.random_state}, verbose={self.verbose})"
        )


# Convenience function for direct usage
def run_rust_oslom(
    edges,
    directed=False,
    r=10,
    hr=50,
    T=0.1,
    cp=0.5,
    singlet=False,
    random_state=None,
    verbose=False,
):
    """
    Run OSLOM directly on an edge list.
    
    Parameters
    ----------
    edges : list of tuples
        List of (source, target, weight) tuples
    directed : bool, default=False
        Whether the graph is directed
    r : int, default=10
        Number of runs for the first hierarchical level
    hr : int, default=50
        Number of runs for higher hierarchical levels
    T : float, default=0.1
        Statistical significance threshold
    cp : float, default=0.5
        Coverage parameter for module unions
    singlet : bool, default=False
        Whether to find singleton nodes
    random_state : int or None, default=None
        Random seed for reproducibility
    verbose : bool, default=False
        Verbosity mode
        
    Returns
    -------
    dict
        Hierarchical clustering result
    """
    if not RUST_AVAILABLE:
        raise ImportError("Rust OSLOM implementation not available")
        
    return run_oslom_direct(
        edges=edges,
        directed=directed,
        r=r,
        hr=hr,
        threshold=T,
        cp=cp,
        find_singletons=singlet,
        random_seed=random_state,
        verbose=verbose,
    )


# Alias for backward compatibility
OSLOM = RustOSLOM