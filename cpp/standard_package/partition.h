#if !defined(PARTITION_INCLUDED)
#define PARTITION_INCLUDED	
	


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



int get_partition_from_file_tp_format(string S, deque<deque<int> > & M, deque<double> & bss);



int get_partition_from_file_tp_format_with_homeless(string S, deque<deque<int> > & M, deque<double> & bss);






int get_partition_from_file_tp_format(string S, map<int, deque<int> > & M);



int get_partition_from_file_tp_format(string S, deque<deque<int> > & M, deque<int> & homel);





int get_partition_from_file_tp_format(string S, deque<deque<int> > & M, bool anyway) ;


int get_partition_from_file_tp_format(string S, deque<deque<int> > & M);

int get_partition_from_file(string s, deque<deque<int> > & M, int min);


int get_partition_from_file(string s, deque<deque<int> > & M);

int get_partition_from_file_list(string s, deque<deque<int> > & ten);
}



#endif


