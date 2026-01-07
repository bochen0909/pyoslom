"""
Rust-based OSLOM implementation with scikit-learn compatible interface.
This module provides a drop-in replacement for the C++ implementation.
"""

import numpy as np
from sklearn.base import BaseEstimator, TransformerMixin, ClusterMixin
from networkx import Graph, DiGraph
import networkx as nx
import tempfile
import shutil
import os

try:
    from pyoslom._rust import PyOslom as RustOslom, run_oslom_direct, set_verbose
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False
    RustOslom = None
    run_oslom_direct = None
    set_verbose = None


class TempDir:
    def __init__(self):
        self.dirpath = None

    def __enter__(self):
        self.dirpath = tempfile.mkdtemp()
        return self.dirpath

    def __exit__(self, type, value, traceback):
        if self.dirpath is not None:
            shutil.rmtree(self.dirpath)


class RustOSLOM(TransformerMixin, ClusterMixin, BaseEstimator):
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
                "Please install with: pip install pyoslom[rust] or build from source."
            )

        self.directed = directed
        self.r = r if r is not None else 10
        self.hr = hr if hr is not None else 50
        self.T = T if T is not None else 0.1
        self.singlet = singlet
        self.cp = cp if cp is not None else 0.5
        self.random_state = random_state
        self.verbose = verbose
        
        self.cluster_ = None
        self._is_fitted = False
        
        # Initialize Rust OSLOM instance
        self._rust_oslom = RustOslom(
            directed=directed,
            r=self.r,
            hr=self.hr,
            threshold=self.T,
            cp=self.cp,
            find_singletons=singlet,
            random_seed=random_state,
            verbose=verbose,
        )

    def fit(self, X, y=None):
        """
        Compute OSLOM clustering.
        
        Parameters
        ----------
        X : {networkx Graph, networkx DiGraph, ndarray, sparse matrix} of shape (n_samples, n_samples)
            Training instances to cluster.
        y : Ignored
            Not used, present here for API consistency by convention.
            
        Returns
        -------
        self
            Fitted estimator.
        """
        self.cluster_ = None
        self._is_fitted = False
        
        # Convert input to NetworkX graph if needed
        if not isinstance(X, Graph) and not isinstance(X, DiGraph):
            if len(X.shape) != 2 or X.shape[0] != X.shape[1]:
                raise ValueError("Input must be a symmetric matrix")
                
            if isinstance(X, np.ndarray):
                if self.directed:
                    X = nx.convert_matrix.from_numpy_array(X, create_using=nx.DiGraph)
                else:
                    X = nx.convert_matrix.from_numpy_array(X, create_using=nx.Graph)
            else:
                if self.directed:
                    X = nx.convert_matrix.from_scipy_sparse_array(X, create_using=nx.DiGraph)
                else:
                    X = nx.convert_matrix.from_scipy_sparse_array(X, create_using=nx.Graph)

        # Validate graph type
        if isinstance(X, Graph) and not isinstance(X, DiGraph):
            if self.directed:
                raise ValueError("Undirected graph provided but directed=True")
        elif isinstance(X, DiGraph):
            if not self.directed:
                raise ValueError("Directed graph provided but directed=False")
        else:
            raise ValueError("Invalid graph type")

        # Convert NetworkX graph to edge list
        edges = []
        for u, v, data in X.edges(data=True):
            weight = data.get('weight', 1.0)
            edges.append((u, v, weight))

        if self.verbose:
            print(f"Processing graph with {X.number_of_nodes()} nodes and {X.number_of_edges()} edges")

        # Run Rust OSLOM
        try:
            self._rust_oslom.fit(edges)
            clusters = self._rust_oslom.get_clusters()
            statistics = self._rust_oslom.get_statistics()
            
            # Convert to format compatible with original implementation
            result = {
                "multilevel": True,
                "num_level": int(statistics["num_levels"]),
                "max_level": max(clusters.keys()) if clusters else 0,
                "params": self._get_params_list(),
                "clusters": clusters,
                "statistics": statistics,
            }
            
            self.cluster_ = result
            self._is_fitted = True
            
            if self.verbose:
                print(f"Found {statistics['num_modules']} modules at base level")
                print(f"Modularity: {statistics['modularity']:.4f}")
                print(f"Coverage: {statistics['coverage']}/{statistics['total_nodes']} nodes")
                
        except Exception as e:
            raise RuntimeError(f"OSLOM clustering failed: {e}")

        return self

    def transform(self, X=None):
        """
        Return the clustering result.
        
        Parameters
        ----------
        X : Ignored
            Not used, present here for API consistency by convention.
            
        Returns
        -------
        dict
            Clustering result with hierarchical modules and statistics.
        """
        if not self._is_fitted:
            raise ValueError("This RustOSLOM instance is not fitted yet. Call 'fit' first.")
        return self.cluster_

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
        for key, value in params.items():
            if hasattr(self, key):
                setattr(self, key, value)
            else:
                raise ValueError(f"Invalid parameter {key}")
        
        # Recreate Rust instance with new parameters
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
        
        return self

    def __repr__(self):
        return (f"RustOSLOM(directed={self.directed}, r={self.r}, hr={self.hr}, "
                f"T={self.T}, cp={self.cp}, singlet={self.singlet}, "
                f"random_state={self.random_state}, verbose={self.verbose})")


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