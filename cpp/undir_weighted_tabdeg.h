#ifndef undir_weighted_tabdeg_INCLUDED
#define undir_weighted_tabdeg_INCLUDED


#include "standard_package/standard_include.h"


namespace oslom {
namespace undir {


typedef multimap<double, pair<int, double> > cup_data_struct;
# define sqrt_two 1.41421356237
# define num_up_to 5
# define bisection_precision 1e-2





double compare_r_variables(double a, double b, double c, double d);





double right_error_function(double x);


double log_together(double minus_log_total, int number);


double fitted_exponent(int N);


  double order_statistics_left_cumulative(int N, int pos, double x);

double inverse_order_statistics(int sample_dim, int pos, const double  & zerof, double lo, double hi);



  double pron_min_exp(int N, double xi);



  double compute_probability_to_stop(const double & a, const double & b, const double & critical_xi, int Nstar, int pos);



bool equivalent_check(int pos_first, int pos_last, double & A_average, double & B_average, int equivalents, int Nstar, const double & critical_xi);



bool equivalent_check_gather(cup_data_struct & a, int & until, const double & probability_a, const double & probability_b, int Nstar, const double & critical_xi);


 double hyper_table(int kin_node, int kout_g, int tm, int degree_node) ;





 double topological_05(int kin_node, int kout_g, int tm, int degree_node);

double compute_global_fitness(int kin_node, int kout_g, int tm, int degree_node, double minus_log_total, int number_of_neighs, int Nstar, double & boot_interval);
	



double compute_global_fitness_step(int kin_node, int kout_g, int tm, int degree_node, double minus_log_total, int number_of_neighs, int Nstar, double _step_) ;
	



 double compute_global_fitness_ofive(int kin_node, int kout_g, int tm, int degree_node, double minus_log_total, int number_of_neighs, int Nstar);

 double compute_global_fitness_randomized(int kin_node, int kout_g, int tm, int degree_node, double minus_log_total, int number_of_neighs, int Nstar);


double compute_global_fitness_randomized_short(int kin_node, int kout_g, int tm, int degree_node, double minus_log_total);





class facts {

	public:
	
		
		facts(int a, double b, multimap<double, int>::iterator c, int d)  ;
		~facts() ;
		
		
		int degree;
		int internal_degree;						
		double minus_log_total_wr;								// wr is the right part of the exponential for the weights, this is the sum over the internal stubs of that
		multimap<double, int>::iterator fitness_iterator;
		
		
};




class weighted_tabdeg {


	public:
				
		weighted_tabdeg() ;
		~weighted_tabdeg() ;
		
		void _set_(weighted_tabdeg &);
		
		
		void clear();
		void edinsert(int a, int kp, int kt, double mtlw, double fit);
		bool erase(int a);
		void set_deque(deque<int> & );


		int size();
		
		
		void print_nodes(ostream &, deque<int> & );
		
		void set_and_update_group(int nstar, int nn, int kout_g, int tm, weighted_tabdeg & one);
		void set_and_update_neighs(int nstar, int nn, int kout_g, int tm, weighted_tabdeg & one);
		bool update_group(int a, int delta_degree, double delta_mtlw, int nstar, int nn, int kout_g, int tm, int kt, deque<int> & to_be_erased);
		bool update_neighs(int a, int delta_degree, double delta_mtlw, int nstar, int kout_g, int tm, int kt);
		

		
		int best_node(int & lab, double & best_fitness, int kout_g, int Nstar, int nneighs, int tm);
		int worst_node(int & lab, double & worst_fitness, int kout_g, int Nstar, int nneighs, int tm);
		bool is_internal(int a);
		
		
		
		
		map<int, facts> lab_facts;					// maps the label into the facts
		multimap<double, int> fitness_lab;			// maps the fitness into the label  (this can be optimized)
	
	
	

};







}}

#endif
