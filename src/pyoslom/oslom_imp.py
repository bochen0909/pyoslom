import numpy as np
from sklearn.base import BaseEstimator, TransformerMixin, ClusterMixin
from networkx import Graph, DiGraph
import networkx as nx 
from coslomundir import run as run_undir
from coslomdir import run as run_dir
import coslomdir, coslomundir
import os
import glob
import tempfile
import shutil


class TempDir():

    def __init__(self):
        self.dirpath = None  
        
    def __enter__(self):
        self.dirpath = tempfile.mkdtemp()
        return self.dirpath

    def __exit__(self, type, value, traceback):
        if self.dirpath is not None:
            shutil.rmtree(self.dirpath)
            
 
class OSLOM(TransformerMixin, ClusterMixin, BaseEstimator):
    
    '''Apply graph clustering by Order Statistics Local Optimization Method

    A wrapper of *OSLOM (Order Statistics Local Optimization Method)* collected from `OSLOM <http://www.pyoslom.org/index.html>`_
    
    Parameters
    ----------
    r : int, default=10
        sets the number of runs for the first hierarchical level, bigger this value, more accurate the output (of course, it takes more). Default value is 10.
    
    R : int, default=50
        sets the number of runs  for higher hierarchical levels. Default value is 50 (the method should be faster since the aggregated network is usually much smaller).

    random_state : int or None, default=None
        sets the seed for the random number generator. (instead of reading from time_seed.dat)
    
    T : float, default=0.1                   
        sets the threshold equal to T, default value is 0.1 
    
    singlet : bool, default=False
        finds singletons. If you use this flag, the program generally finds a number of nodes which are not assigned to any module.
        the program will assign each node with at least one not homeless neighbor. This only applies to the lowest hierarchical level.
    
    verbose : bool, default=False
        Verbosity mode.

    cp : float, default=0.5
        sets a kind of resolution parameter equal to P. This parameter is used to decide if it is better to take some modules or their union.
        Default value is 0.5. Bigger value leads to bigger clusters. P must be in the interval (0, 1).
    '''

    def __init__(self, directed=False, r=None, hr=None, T=None, singlet=False, cp=None, random_state=None, verbose=False):
        self.directed = directed
        self.r = r
        self.hr = hr
        self.T = T
        self.singlet = singlet
        self.cp = cp
        self.cluster_ = None 
        self._is_fitted = False
        self.random_state = random_state
        self.verbose = verbose
        options = ['oslom_dir' if directed else 'oslom_undir', '-w']
        if r is not None:
            options += ['-r', r]
        if hr is not None:
            options += ['-hr', hr]
        if random_state is not None:
            options += ['-seed', random_state]            
        if T is not None:
            options += ['-T', T]  
        if cp is not None:
            options += ['-cp', cp]
        if singlet:
            options += ['-singlet']
        self.options = options 

    def _set_verbose(self):
        coslomdir.set_verbose(self.verbose)
        coslomundir.set_verbose(self.verbose)
        
    def fit(self, X, y=None):
        """Compute k-means clustering.
        Parameters
        ----------
        X : {nextworkx Graph, networkx Digraph, ndarray, sparse matrix} of shape (n_samples, n_samples)
            Training instances to cluster.
            
        y : Ignored
            Not used, present here for API consistency by convention.

        Returns
        -------
        self
            Fitted estimator.
        """
        
        '''
            X is either networkx graph or matrix (dense or sparse, mostly like sparse)
        '''
        
        self.cluster_ = None 
        self._is_fitted = False
        if not isinstance(X, Graph) and not isinstance(X, DiGraph):
            if len(X.shape) != 2 or X.shape[0] != X.shape[1]:
                raise Exception("must be symmetric matrix")
            if isinstance(X, np.ndarray):
                if(self.directed):
                    X = nx.convert_matrix.from_numpy_matrix(X, create_using=nx.DiGraph) 
                else:
                    X = nx.convert_matrix.from_numpy_matrix(X, create_using=nx.Graph)
            else:
                if(self.directed):
                    X = nx.convert_matrix.from_scipy_sparse_matrix(X, create_using=nx.DiGraph) 
                else:
                    X = nx.convert_matrix.from_scipy_sparse_matrix(X, create_using=nx.Graph)
                            
        if isinstance(X, Graph) and not isinstance(X, DiGraph):
            assert not self.directed
        elif isinstance(X, DiGraph):
            assert self.directed
        else:
            raise Exception("never be here")
            
        method = run_dir if self.directed else run_undir
        cwd = os.getcwd()
        with TempDir() as tmp_dir:
            tmp_dir="/tmp/a"
            edgefile = os.path.join(tmp_dir, "edges.txt")
            nx.write_edgelist(X, edgefile, data=["weight"])
            cmd = self.options + ["-f", "edges.txt"] 
            cmd = [str(u) for u in cmd]  
            if self.verbose:         
                print("Running " + str(cmd)) 
            
            os.chdir(tmp_dir)
            self._set_verbose() 
            status = method(cmd)
            os.chdir(cwd)
            
            if status != 0:
                raise Exception("Run command with error status code {}".format(status))
            outputfiles = glob.glob(os.path.join(tmp_dir, "edges.txt_oslo_files", "tp*"))
            clusters = {}
            for tp in outputfiles:
                fname = tp.split("/")[-1]
                if fname == 'tp':
                    level = 0 
                else:
                    level = int(fname[2:])
                with open(tp) as f:
                    lines = [u.strip() for u in f if not u.startswith('#')]
                    lines = [[int(v) for v in u.split(" ")] for u in lines]
                    clusters[level] = dict(enumerate (lines))
                    
            max_level = max(list(clusters.keys())) if clusters else 0  
                    
        result = {}
        result['multilevel'] = True
        result['num_level'] = len(clusters)
        result['max_level'] = max_level
        result['params'] = self.options
        result['clusters'] = clusters
        
        self.cluster_ = result
        
        self._is_fitted = True
        return self

    def transform(self, X=None):
        if not self._is_fitted:
            raise Exception("fit first")
        return self.cluster_


if __name__ == "__main__":
    pass
    
