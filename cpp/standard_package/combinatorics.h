#if !defined(COMBINATORICS_INCLUDED)
#define COMBINATORICS_INCLUDED	

#include <deque>
#include <set>

#include "random.h"

namespace oslom{

template <typename Seq>
double average_func(Seq &sq) {
	
	if (sq.empty())
		return 0;
	
	double av=0;
	typename Seq::iterator it = sq.begin(); 
	while(it != sq.end())
		av+=*(it++);
	
	av=av/sq.size();
	
	return av;
	
}



template <typename Seq>
double variance_func(Seq &sq) {
	
	if (sq.empty())
		return 0;
	
	double av=0;
	double var=0;
	
	
	typename Seq::iterator it = sq.begin(); 
	while(it != sq.end()) {
		
		av+=*(it);
		var+=(*(it))*(*(it));
		it++;
		
	}
	
	
	av=av/sq.size();
	var=var/sq.size();
	var-=av*av;
	
	if(var<1e-7)
		return 0;
	
	return var;
	
}



// this returns the average of the discrete probability function stored in Seq
template <typename Seq>
double average_pf(Seq &sq) {
	
	
	double av=0;
	int h=0;
	
	typename Seq::iterator it = sq.begin(); 
	while(it != sq.end()) {
		
		av+=*(it)*h;
		it++;
		h++;
	
	}
	
	return av;
	
}



template <typename Seq>
double variance_pf(Seq &sq) {
	
	
	double av=0;
	double var=0;
	int h=0;
	
	typename Seq::iterator it = sq.begin(); 
	while(it != sq.end()) {
		
		av+=*(it) * h;
		var+=(*(it)) * h * h ;
		it++;
		h++;
	}
	
	
	var-=av*av;
	
	if(var<1e-7)
		return 0;
	
	return var;
	
}








double log_factorial (int num);





double log_combination (int n, int k);



double binomial(int n, int x, double p);

int binomial_cumulative(int n, double p, std::deque<double> &cum);



int powerlaw (int n, int min, double tau, std::deque<double> &cumulative);


int distribution_from_cumulative(const std::deque<double> &cum, std::deque<double> &distr);
int cumulative_from_distribution (std::deque<double> &cum, const std::deque<double> &distr);


double poisson (int x, double mu);



int shuffle_and_set(int *due, int dim);

template<typename T>
int shuffle_s(std::deque<T> &sq) {
	
	int siz=sq.size();
	if(siz==0)
		return -1;
	
	for (int i=0; i<int(sq.size()); i++) {
		
		int random_pos=irand(siz-1);
	
		T random_card_=sq[random_pos];
	
		sq[random_pos]=sq[siz-1];
		sq[siz-1]=random_card_;
		siz--;
		
	
	}
	
	
	return 0;
	
	
}



template <typename type_>
int shuffle_s(type_ *a, int b) {
	
		
	
	int siz=b;
	if(siz==0)
		return -1;
	
	for (int i=0; i<b; i++) {
		
		int random_pos=irand(siz-1);
	
		type_ random_card_=a[random_pos];
	
		a[random_pos]=a[siz-1];
		a[siz-1]=random_card_;
		siz--;
		
	
	}
	
	return 0;
}





double compute_r(int x, int k, int kout, int m);

int add_factors (std::deque<double> & num, std::deque<double> &den, int  n, int k);

double compute_hypergeometric(int i, int k, int kout, int m);

int random_from_set(std::set<int> & s);


}

#endif
