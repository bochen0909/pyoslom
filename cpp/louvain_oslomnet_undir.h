#ifndef louvain_oslomnet_undir_INCLUDED
#define louvain_oslomnet_undir_INCLUDED
#include "standard_package/standard_include.h"

#include "undirected_network.h"

namespace oslom {
namespace undir {





class oslom_module {

	
public:
	
	
	oslom_module(int a);
	~oslom_module();
	
	int nc;
	int kout;
	int ktot;
	
};






typedef map<int, pair<int, double> > mapip;
typedef map<int, oslom_module> map_int_om;



void prints(map_int_om & M);



class oslomnet_louvain : public static_network {
	
public:
	
		
	oslomnet_louvain();
	~oslomnet_louvain();
	
	int collect_raw_groups_once(deque<deque<int> > & );
	
private:

	void weighted_favorite_of(const int & node, int & fi, int & kp, int & kop);
	void unweighted_favorite_of(const int & node, int & fi, int & kp, int & kop);

	void single_pass_unweighted();
	void single_pass_weighted();
	inline void update_modules(const int & i, const int & fi, const int & kp, const int & kpo);
	
		
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
