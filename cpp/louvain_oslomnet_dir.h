
#ifndef louvain_oslomnet_dir_INCLUDED
#define louvain_oslomnet_dir_INCLUDED
#include "standard_package/standard_include.h"

#include "directed_network.h"

namespace oslom {
namespace dir {




class oslom_module {

	
public:
	
	
	oslom_module(int a, int b);
	~oslom_module();
	
	int nc;
	int kout_in;
	int kout_out;
	int ktot_in;
	int ktot_out;
	
};



class internal_links_weights {

public:
	
	internal_links_weights(int a, double b, int c, double d);
	~internal_links_weights();
	
	int k_in;
	int k_out;
	double win;
	double wout;
	
};







typedef map<int, internal_links_weights> mapip;
typedef map<int, oslom_module> map_int_om;



void int_histogram(const int & c, mapip  & hist, const int & w1, const double & w2,  const int & w1_out, const double & w2_out);



void prints(map_int_om & M);





class oslomnet_louvain : public static_network {
	
public:
	
		
	oslomnet_louvain();
	~oslomnet_louvain();
	
	int collect_raw_groups_once(deque<deque<int> > & );
	
private:

	void weighted_favorite_of(const int & node, int & fi, int & kp_in, int & kop_in, int & kp_out, int & kop_out);
	void unweighted_favorite_of(const int & node, int & fi, int & kp_in, int & kop_in, int & kp_out, int & kop_out);

	void single_pass_unweighted();
	void single_pass_weighted();
	inline void update_modules(const int & i, const int & fi, const int & kp_in, const int & kop_in, const int & kp_out, const int & kop_out);
	
		
	void module_initializing();
	void set_partition_collected(deque<deque<int> > & M);

	
	//int check_all();



	map<int, oslom_module> label_module;
	deque<int> vertex_label;
	deque<int> vertex_order;
	deque<bool> vertex_to_check;
	deque<bool> vertex_to_check_next;
	int nodes_changed;
	

};






}}
#endif
