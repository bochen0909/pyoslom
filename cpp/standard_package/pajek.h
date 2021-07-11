


#if !defined(PAJEK_INCLUDED)
#define PAJEK_INCLUDED


#include <deque>
#include <set>
#include <vector>
#include <map>
#include <math.h>
#include <algorithm>
#include <iostream>

#include "combinatorics.h"
#include "cast.h"

using namespace std;

namespace oslom{



int number_together(deque<int> & a);


int get_partition_from_file_list_pajek(string s, deque<deque<int> > & ten, deque<int> & oldlabels);



void set_partition_from_list(deque<int> & mems, deque<deque<int> > & ten);

int get_partition_from_file_list_pajek_tree(string s, deque<deque<deque<int> > > & Ten);


int pajek_format(string filename, bool directed);

}

#endif //PAJEK_INCLUDED
