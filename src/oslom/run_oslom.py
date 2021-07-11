#!/usr/bin/env python

import tempfile
import os 
import shutil
import time
import glob 

script_dir= (os.path.dirname(os.path.realpath(__file__)))
current_dir = os.getcwd()
def shell_run_and_wait(command, working_dir=None, env=None):
    curr_dir = os.getcwd()
    if working_dir is not None:
        os.chdir(working_dir)
    command = command.split(" ")
    import subprocess
    
    # process = subprocess.Popen(command, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    process = subprocess.Popen(command, env=env)
    process.wait()
    if working_dir is not None:
        os.chdir(curr_dir)
    return process.returncode


def timeit(fun):
    t0 = time.time()
    ret = fun()
    t1 = time.time()
    return t1 - t0, ret

def remove_if_file_exit(fname, is_dir=False):
    if os.path.exists(fname):
        if is_dir: 
            shutil.rmtree(fname) 
        else:
            os.remove(fname)
            
class TempDir():

    def __init__(self):
        self.dirpath = None  
        
    def __enter__(self):
        self.dirpath = tempfile.mkdtemp()
        return self.dirpath

    def __exit__(self, type, value, traceback):
        if self.dirpath is not None:
            shutil.rmtree(self.dirpath)

        

class OSLOM():
    '''
    A wrapper of *OSLOM (Order Statistics Local Optimization Method)* collected from `OSLOM <http://www.oslom.org/index.html>`_
    
    Arguments
    
      [-r R]:                       sets the number of runs for the first hierarchical level, bigger this value, more accurate the output (of course, it takes more). Default value is 10.
    
      [-hr R]:                      sets the number of runs  for higher hierarchical levels. Default value is 50 (the method should be faster since the aggregated network is usually much smaller).
    
      [-seed m]:                    sets m equal to the seed for the random number generator. (instead of reading from time_seed.dat)
    
      [-hint filename]:             takes a partition from filename. The file is expected to have the nodes belonging to the same cluster on the same line.
    
      [-load filename]:             takes modules from a tp file you already got in a previous run.
    
      [-t T]:                       sets the threshold equal to T, default value is 0.1
    
      [-singlet]:                    finds singletons. If you use this flag, the program generally finds a number of nodes which are not assigned to any module.
                                    the program will assign each node with at least one not homeless neighbor. This only applies to the lowest hierarchical level.
    
      [-cp P]:                      sets a kind of resolution parameter equal to P. This parameter is used to decide if it is better to take some modules or their union.
                                    Default value is 0.5. Bigger value leads to bigger clusters. P must be in the interval (0, 1).
    
      [-fast]:                      is equivalent to "-r 1 -hr 1" (the fastest possible execution).
    
      [-infomap runs]:              calls infomap and uses its output as a starting point. runs is the number of times you want to call infomap.
    
      [-copra runs]:                same as above using copra.
    
      [-louvain runs]:              same as above using louvain method.


    Reference 
        A. Lancichinetti, F. Radicchi, J.J. Ramasco and S. Fortunato Finding statistically sig-
        nificant communities in networks PloS One 6, e18961 (2011)
    '''

    def __init__(self, prog, edges, options):
        self.prog = prog
        self.options= options
        self.edges= edges

    def get_meta(self):
        return {'lib':"OSLOM", "name": 'oslom' }
    
    def run(self):
        
        with TempDir() as tmp_dir:
            #tmp_dir = "/tmp/abc"
            def link_file(path, destname=None):
                if destname is None :
                    destname = path.split("/")[-1]
                destpath = os.path.join(tmp_dir, destname)
                remove_if_file_exit(destpath)
                os.symlink(path, destpath)
                return destpath

            _ = link_file(os.path.join(script_dir,self.prog))
            _ = link_file(os.path.join(current_dir,self.edges), "edges.txt")
            
            cmd = "{} -f edges.txt {}".format(self.prog,self.options)            
            print("Running " + cmd) 
            
            timecost, status = timeit(lambda: shell_run_and_wait(cmd, tmp_dir))
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
                    
            max_level = max(list(clusters.keys()))
                    
        print("Made %d levels of clusters with #clusters %s in %f seconds" % (len(clusters), str([len(u) for u in clusters.values()]), timecost))
        
        result = {}
        result['multilevel'] = True
        result['num_level'] = len(clusters)
        result['max_level'] = max_level
        result['params'] = self.options
        result['meta'] = self.get_meta()
        result['timecost'] = timecost
        result['clusters'] = clusters
        return result; 


if __name__ == "__main__":
    import sys
    if not(len(sys.argv)>=5):
        print("usage: cmd result_file prog edges options")
    else:
        os.environ["OSLOM_ONLY_LEVEL0"]="1"
        options = " ".join(sys.argv[4:])
        result_file=sys.argv[1]
        alg = OSLOM(sys.argv[2], sys.argv[3], options)
        ret = alg.run()
        with open(result_file, 'wt') as fo:
            fo.write("#{}\t{}\n".format("node","clu"))
            for k,c in ret['clusters'][0].items():
                for n in c:
                    fo.write("{}\t{}\n".format(n,k))
    
