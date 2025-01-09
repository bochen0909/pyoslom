import networkx as nx
from pyoslom import OSLOM
G = nx.read_pajek("example.pajek")
alg = OSLOM(random_state=123, verbose=True)
results = alg.fit_transform(G)


def print_clus(clus):
    for k, v in clus.items():
        if k != 'clusters':
            print(str(k) + "=" + str(v))
    for k, l in clus['clusters'].items():
        print("Level:" + str(k) + ", #clu=" + str(len(l)))


print_clus(results)
