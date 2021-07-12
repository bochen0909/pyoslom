
# Python binding for OSLOM graph clustering algorithm   


## Summary

Pyolsom is a python binding for [OSLOM](http://www.oslom.org/) (Order Statistics Local Optimization Method) graph clustering algorithm.

It works with directed/undirected weighted and unweighted graph. 
The algorithm performs usually good but slow, so it is better to be applied to medium graph size. 

The orginal C++ code is really hard to be refactored. I tried the best to make it work with python.

### Known issues

* The lib is not thread safe. So use mutliprocess  when parallel is required. 


## Requirements
* C++ 17 
* Python 3
* scikit-learn>=0.24
* pybind11>=2.6
* networkx>=2.5

The versions are what I worked on. Lower versions may work also.  

## Install

Install pybind11 first because there is no binary release in pip repo and the *setup.py* depends on *pybind11*. 

```bash
pip install "pybind11>=2.6"
```
On Windows  install *Microsoft Visual C++ Build Tool* first (refer to [https://wiki.python.org/moin/WindowsCompilers](https://wiki.python.org/moin/WindowsCompilers)).

### build from source
```bash
git clone https://bochen0909@github.com/bochen0909/pyoslom.git && cd pyoslom && python setup.py install
```

### or use pip
```bash
pip install pyoslom
```

## How to use

Example:

```python
import networkx as nx
from pyoslom import OSLOM

G = nx.read_pajek("example.pajek") # networkx graph or adjacency matrix
alg = OSLOM(random_state=123)
results = alg.fit_transform(G)

def print_clus(clus):
    for k, v in clus.items():
        if k != 'clusters':
            print(str(k) + "=" + str(v))
    for k, l in clus['clusters'].items():
        print("Level:" + str(k) + ", #clu=" + str(len(l)))

print_clus(results)

```

For more complete examples please see the notebook [example.ipynb](example/example.ipynb).

![example_clu0.png](example/example_clu0.png)
![example_clu1.png](example/example_clu1.png)

## License
The original c++ code is published at [OSLOM](http://www.oslom.org/) following a research publication. However there is no license attached with it. 
The python wrapping work is licensed under the GPLv2.
