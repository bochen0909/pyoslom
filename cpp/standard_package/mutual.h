	


#if !defined(MUTUAL_INCLUDED)
#define MUTUAL_INCLUDED	


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


int overlap_grouping(deque<deque<int> > ten, int unique);


double mutual (deque<deque<int> > en, deque<deque<int> > ten);

double H(double a);


double H(deque <double> &p);


double H_x_given_y(deque<deque<int> > &en, deque<deque<int> > &ten, int dim);

double mutual2(deque<deque<int> > en, deque<deque<int> > ten);
double H_x_given_y3(deque<deque<int> > &en, deque<deque<int> > &ten, int dim);


double mutual3(deque<deque<int> > en, deque<deque<int> > ten);

}

#endif

