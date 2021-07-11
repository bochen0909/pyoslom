
#ifndef log_table_INCLUDED
#define log_table_INCLUDED

#include "standard_package/standard_include.h"


namespace oslom{
class log_fact_table;

typedef double (*right_cumulative_function_type)(log_fact_table* self, int k1, int k2, int k3, int x);

class log_fact_table {



	public:
		log_fact_table(right_cumulative_function_type fp);
		~log_fact_table();
		
		double log_factorial(int a);

		//void set_small_tab_right_hyper(int, int);
		
		 double log_hyper(int kin_node, int kout_g, int tm, int degree_node);
		 double hyper(int kin_node, int kout_g, int tm, int degree_node);
		 double binom(int x, int N, double p);
		 double log_choose(int tm, int degree_node);
		
		double cum_binomial_right(int x, int N, double prob);
		double cum_binomial_left(int x, int N, double prob);
		
		 double log_symmetric_eq(int k1, int k2, int H, int x);
		double slow_symmetric_eq(int k1, int k2, int H, int x);
		//vector<vector<vector<vector<double> > > > small_rh;		/*********/// small_rh[tm][kout][k][kin]		tm>=kout>=k>=kin
		//double right_cum_symmetric_eq(int k1, int k2, int H, int x);		
		double fast_right_cum_symmetric_eq(int k1, int k2, int H, int x, int mode, int tm);


		double right_cumulative_function(int k1, int k2, int k3, int x);
	
		double cum_hyper_right(int kin_node, int kout_g, int tm, int degree_node);
		void _set_(int);
		
	private:
	
		right_cumulative_function_type right_cumulative_function_pointer;
		vector<double> lnf;

		double loghyper1(int tm, int degree_node, int kout_g);
		double hyper2(int tm, int degree_node, int kout_g, int x, double constlog);
		 double sym_ratio(int & k1, int & k2, int & H, double i);


		double cum_hyper_left(int kin_node, int kout_g, int tm, int degree_node);


};


}

#endif
