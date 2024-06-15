# -*- coding: utf-8 -*-
from setuptools import setup

package_dir = \
{'': 'src'}

packages = \
['pyoslom']

package_data = \
{'': ['*']}

install_requires = \
['networkx>=3.3,<4.0',
 'pybind11>=2.12.0,<3.0.0',
 'scikit-learn>=1.5.0,<2.0.0',
 'wheel>=0.43.0,<0.44.0']

entry_points = \
{'console_scripts': ['test = scripts:test']}

setup_kwargs = {
    'name': 'pyoslom',
    'version': '0.1.5',
    'description': 'Python Wrapper for OSLOM',
    'long_description': '\n# Python binding for OSLOM graph clustering algorithm   \n\n\n## Summary\n\nPyolsom is a python binding for [OSLOM](http://www.oslom.org/) (Order Statistics Local Optimization Method) graph clustering algorithm.\n\nIt works with directed/undirected weighted and unweighted graph. \nThe algorithm performs usually good but slow, so it is better to be applied to medium graph size. \n\nThe orginal C++ code is really hard to be refactored. I tried the best to make it work with python.\n\n### Known issues\n\n* The lib is not thread safe. So use mutliprocess  when parallel is required. \n* Only works on Linux\n\n\n## Requirements\n* C++ 17 \n* Python >= 3.10\n* scikit-learn>=0.24\n* pybind11>=2.6\n* networkx>=2.5\n\nThe versions are what I worked on. Lower versions may work also.  \n\n## Install\n\n### Use setup.py\n```bash\ngit clone https://bochen0909@github.com/bochen0909/pyoslom.git\ncd pyoslom \npip install -r requirements.txt\npython setup.py install\n```\n\n### Use Poetry\n```bash\ngit clone https://bochen0909@github.com/bochen0909/pyoslom.git\ncd pyoslom \npoetry install --no-root\npoetry build\npoetry install\n```\n\n### or use pip\n```bash\npip install pyoslom\n```\n\n## How to use\n\nExample:\n\n```python\nimport networkx as nx\nfrom pyoslom import OSLOM\n\nG = nx.read_pajek("example.pajek") # networkx graph or adjacency matrix\nalg = OSLOM(random_state=123)\nresults = alg.fit_transform(G)\n\ndef print_clus(clus):\n    for k, v in clus.items():\n        if k != \'clusters\':\n            print(str(k) + "=" + str(v))\n    for k, l in clus[\'clusters\'].items():\n        print("Level:" + str(k) + ", #clu=" + str(len(l)))\n\nprint_clus(results)\n\n```\n\nFor more complete examples please see the notebook [example.ipynb](example/example.ipynb).\n\n![example_clu0.png](example/example_clu0.png)\n![example_clu1.png](example/example_clu1.png)\n\n## License\nThe original c++ code is published at [OSLOM](http://www.oslom.org/) following a research publication. However there is no license attached with it. \nThe python wrapping work is licensed under the GPLv2.\n',
    'author': 'Bo Chen',
    'author_email': 'bochen0909@gmail.com',
    'maintainer': 'Bo Chen',
    'maintainer_email': 'bochen0909@gmail.com',
    'url': 'https://github.com/bochen0909/pyoslom',
    'package_dir': package_dir,
    'packages': packages,
    'package_data': package_data,
    'install_requires': install_requires,
    'entry_points': entry_points,
    'python_requires': '>=3.10,<4.0',
}
from build import *
build(setup_kwargs)

setup(**setup_kwargs)
