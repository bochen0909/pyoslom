'''
Created on Jul 9, 2021

@author: Bo Chen
'''
import unittest
import numpy as np
from oslom import OSLOM  
from scipy.sparse import csr_matrix, lil_matrix, csc_matrix, coo_matrix
import oslom


class Test(unittest.TestCase):

    def print_clu(self, clu):
        for k, v in clu.items():
            if k != 'clusters':
                print(str(k) + "=" + str(v))
        for l in clu['clusters']:
            print("#clu=" + str(len(l)) + "\t" + str(l))
            
    def atest_fit_dense(self):
        clustering = OSLOM (random_state=123)
        clus = clustering.fit_transform((100 * np.random.random(size=(100, 100))).astype(np.int))
        clus = clustering.fit_transform(np.random.random(size=(100, 100)))
        clus = clustering.fit_transform(np.random.random(size=(100, 100)).astype(np.float32))
        self.print_clu(clus)

    def atest_sparse_matrix(self):
        
        for cls in [csr_matrix, lil_matrix, csc_matrix, coo_matrix ]:
            X = np.random.random(size=(100, 100));
            X[X < 0.5] = 0
            A = cls(X)
            obj = OSLOM ()
            clus = obj.fit_transform(A)
            self.print_clu(clus)
            
    def test_sparse_matrix_directed(self):
        
        for cls in [csr_matrix]:
            X = np.random.random(size=(100, 100));
            X[X < 0.3] = 0
            A = cls(X)
            obj = OSLOM (directed=True, verbose=True)
            clus = obj.fit_transform(A)
            self.print_clu(clus)            

        
if __name__ == "__main__":
    # import sys;sys.argv = ['', 'Test.testName']
    unittest.main()
