/*
 * oslomnetglobalhandler.h
 *
 *  Created on: May 12, 2020
 *      Author: bo
 */

#ifndef SOURCES_2_5_OSLOM_REFACTOR_OSLOM_NET_GLOBAL_HANDLER_H_
#define SOURCES_2_5_OSLOM_REFACTOR_OSLOM_NET_GLOBAL_HANDLER_H_

#include <stdexcept>
#include "module_collection.h"
namespace oslom {

class oslom_net_global_handler {
public:
	virtual void hint(module_collection & minimal_modules,  string filename) {
		throw std::runtime_error("not implemented");
	};
	virtual int size() =0;
	virtual void print_modules(bool not_homeless, string tp, module_collection & Mcoll) {
		throw std::runtime_error("not implemented");
	};
	virtual void print_modules(bool not_homeless, ostream & out1, module_collection & Mcoll) {
		throw std::runtime_error("not implemented");
	};
	virtual int try_to_assign_homeless(module_collection & Mcoll, bool anyway)  {
		throw std::runtime_error("not implemented");
	};
	virtual void get_covers(string cover_file, int & soft_partitions_written, int gruns)  {
		throw std::runtime_error("not implemented");
	};
	virtual void ultimate_cover(string cover_file, int soft_partitions_written, string final_cover_file)  {
		throw std::runtime_error("not implemented");
	};
	virtual int translate(deque<int> & a)   {
		throw std::runtime_error("not implemented");
	};
	virtual int translate(deque<deque<int> > &)   {
		throw std::runtime_error("not implemented");
	};
	virtual void print_statistics(ostream & outt, module_collection & Mcoll)  {
		throw std::runtime_error("not implemented");
	};
	virtual int set_upper_network(map<int, map<int, pair<int, double> > > & neigh_weight_f, module_collection & module_coll)  {
		throw std::runtime_error("not implemented");
	};
	virtual void set_graph(map<int, map<int, pair<int, double> > > & A) =0;
	virtual int draw(string) =0;
	virtual double stubs() =0;
};

}

#endif /* SOURCES_2_5_OSLOM_REFACTOR_OSLOM_NET_GLOBAL_HANDLER_H_ */
