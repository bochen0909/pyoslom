


#include "log_table.h"

namespace oslom{
#define log_table_pr 1e-5



		log_fact_table::log_fact_table(right_cumulative_function_type fp){
			this->right_cumulative_function_pointer=fp;
		}
		log_fact_table::~log_fact_table(){};
		
		double log_fact_table::log_factorial(int a) { return lnf[a]; };
		 double log_fact_table::log_hyper(int kin_node, int kout_g, int tm, int degree_node) {   return log_choose(kout_g, kin_node) + log_choose(tm - kout_g, degree_node - kin_node) - log_choose(tm, degree_node);   };
		 double log_fact_table::hyper(int kin_node, int kout_g, int tm, int degree_node) { return max(0., exp(log_hyper(kin_node, kout_g, tm, degree_node)));  };
		 double log_fact_table::binom(int x, int N, double p) { return exp(  log_choose(N, x) + x*log(p) + (N-x) * log(1-p)    ); };
		 double log_fact_table::log_choose(int tm, int degree_node) {  return lnf[tm] - lnf[tm - degree_node] - lnf[degree_node];  };
		
		 double log_fact_table::log_symmetric_eq(int k1, int k2, int H, int x) {	return -x * lnf[2] - lnf[k1-x] - lnf[k2 -x] - lnf[x+H] -lnf[x];  };
	
		 double log_fact_table::sym_ratio(int & k1, int & k2, int & H, double i) { return 0.5 * (k1 - i +1) / ((i+H)*i)   * (k2 - i +1); };




void log_fact_table::_set_(int size) {

	spdout<<"allocating "<<size<<" factorials..."<<"\n";
	lnf.clear();
	
	lnf.reserve(size+1);
	
	double f=0;
	lnf.push_back(0);
	
	for(int i=1; i<=size; i++) {
		
		f+=log(i);
		lnf.push_back(f);
	}
	
	spdout<<"done"<<"\n";
	//prints(lnf);
	


}




double log_fact_table::cum_hyper_right(int kin_node, int kout_g, int tm, int degree_node) {
	
	//spdout<<"kin_node... "<<kin_node<<" "<<kout_g<<" "<<tm<<" "<<degree_node<<"\n"; 
	// this is bigger  or equal p(x >= kin_node)   *** EQUAL ***
	
	if(kin_node>min(degree_node, kout_g))
		return 0;
	
	
	
	if(tm - kout_g - degree_node + kin_node <=0)
		return 1;
	
	if(kin_node<=0)
		return 1;
	
	
	if(kin_node<double(kout_g+1)/double(tm +2)*double(degree_node+1))
		return (1. - cum_hyper_left(kin_node, kout_g, tm, degree_node));
	
	
	
	
	int x=kin_node;
	double pzero= hyper(x, kout_g, tm, degree_node);
	
	
	//*
	if(pzero<=1e-40)
		return 0;
	//*/
	
	
	
	
	double ga= tm - kout_g - degree_node;
	int kout_g_p = kout_g +1;
	double degree_node_p = degree_node +1;
	double z_zero= 1.;
	double sum= z_zero;
	
	
	
	while(true) {
	
		
		++x;
		
		z_zero *= double(kout_g_p -x) / (x * (ga + x)) * (degree_node_p - x);

		
		if(z_zero < log_table_pr * sum)
			break;
		
		if(pzero * sum>1)
			return pzero;
		
		sum+=z_zero;	
	}
	
	
	return pzero * sum;
	
}



double log_fact_table::cum_hyper_left(int kin_node, int kout_g, int tm, int degree_node) {
	
	
	// this is strictly less  p(x < kin_node)   *** NOT EQUAL ***
	//spdout<<kin_node<<" node: "<<degree_node<<" group: "<<tm<<" "<<degree_node<<"\n";
	
	
	if(kin_node<=0)
		return 0;

	if(tm - kout_g - degree_node + kin_node <=0)
		return 0;	
	
	if(kin_node>min(degree_node, kout_g))
		return 1;	

	
	
	if(kin_node>double(kout_g+1)/double(tm +2)*double(degree_node+1))
		return (1. - cum_hyper_right(kin_node, kout_g, tm, degree_node));
	
	
		
	
	int x=kin_node-1;
	double pzero= hyper(x, kout_g, tm, degree_node);
	
	//spdout<<"pzero: "<<pzero<<" "<<log_hyper(x, kout_g, tm, degree_node)<<" gsl: "<<(gsl_ran_hypergeometric_pdf(x, kout_g, tm - kout_g,  degree_node))<<"\n";
	
	
	
	//*
	if(pzero<=1e-40)
		return 0;
	//*/
	
	
	
	
	double ga= tm - kout_g - degree_node;
	int kout_g_p = kout_g +1;
	double degree_node_p = degree_node +1;
	double z_zero= 1.;
	double sum= z_zero;
	
	//spdout<<"pzero "<<pzero<<" "<<z_zero<<" "<<kin_node<<"\n";
	
	while(true) {
	
		
		
		
		z_zero *= (ga + x) / ((degree_node_p - x) *(kout_g_p -x))  * x;
		--x;
		//spdout<<"zzero sum "<<z_zero<<" "<<sum<<" "<<(ga + x)<<"\n";
		
		if(z_zero< log_table_pr *sum)
			break;
		
		if(pzero * sum>1)
			return pzero;
		
		sum+=z_zero;	
	}
	
	
	return pzero * sum;
	
}


double log_fact_table::cum_binomial_right(int x, int N, double prob) {

	// this is bigger  or equal p(x >= kin_node)   *** EQUAL ***
	
	//spdout<<"x "<<x<<" N "<<N <<"  prob "<<prob<<"\n";
	
	
	if(x<=0) 
		return 1;
	
	if(x>N)
		return 0;
	
	
	if(prob-1> - 1e-11)
		return 1;
	
	if(x<N*prob)
		return 1-cum_binomial_left(x, N, prob);
	
	
	double pzero= binom(x, N, prob);
	
	
	if(pzero<=1e-40)
		return 0;
	
	
	double z_zero= 1.;
	double sum= z_zero;
	
	
	while(true) {
	
		
		
		
		z_zero *=  prob * double(N-x) / ((x+1)*(1-prob));
		x++;
		//spdout<<"zzero sum "<<z_zero<<" "<<sum<<" "<<"\n";
		
		if(z_zero< log_table_pr * sum)
			break;
		
		sum+=z_zero;	
	}
	
	
	return pzero * sum;




}





double log_fact_table::cum_binomial_left(int x, int N, double prob) {

	// this is less strictly p(x < kin_node)   *** NOT EQUAL ***
	
	
	if(x<=0)
		return 0;
	
	if(x>N)
		return 1;
	
	
	if(prob<1e-11)
		return 1;
	
	if(x>N*prob)
		return 1-cum_binomial_right(x, N, prob);
	
	--x;
	double pzero= binom(x, N, prob);
	
	
	if(pzero<=1e-40)
		return 0;
	
	
	double z_zero= 1.;
	double sum= z_zero;
	
	while(true) {
	
		
		
		--x;
		z_zero *=  (1-prob) * double(x+1) / ((N-x) *prob);
		
		//spdout<<"zzero sum "<<z_zero<<" "<<sum<<" "<<(ga + x)<<"\n";
		
		if(z_zero< log_table_pr * sum)
			break;
		
		sum+=z_zero;	
	}
	
	
	return pzero * sum;
}






double log_fact_table::slow_symmetric_eq(int k1, int k2, int H, int x) {
	
	// k1, k2 and k3 are the three colors
	
	//spdout<<"k3: "<<k3<<"\n";
	

	int l1=max(0, -H);
	int l2=min(k1, k2);
	
	//spdout<<"l1: "<<l1<<" l2: "<<l2<<"\n";
	
	
	if(x<l1)
		return 0;
	
	if(x>l2)
		return 0;
	
	
	double p=0;
	for(int ix=l1; ix<=l2; ++ix) {
		
		//spdout<<ix<<" "<<(log_symmetric_eq(k1, k2, H, ix))<<"\n";
		p+=exp(log_symmetric_eq(k1, k2, H, ix));
	
	}
	
	//spdout<<"p: "<<p<<"\n";
	
	
	
	return exp(log_symmetric_eq(k1, k2, H, x))/p;

}


double log_fact_table::right_cumulative_function(int k1, int k2, int k3, int x){
	return this->right_cumulative_function_pointer(this, k1,k2,k3,x);

}


 double log_fact_table::fast_right_cum_symmetric_eq(int k1, int k2, int H, int x, int mode, int tm) {
	
	// I want k1 to be the smaller between the two
		
	
	
	//spdout<<"k1 "<<k1<<" "<<k2<<" "<<H<<" "<<x<<" "<<mode<<" "<<2*H+k1+k2<<"\n";
	
	
	if(k1>k2)
		return fast_right_cum_symmetric_eq(k2, k1, H, x, mode, tm);
	

	
	double ri=1;
	double q1=0;
	double q2=0;
	double ratio;
	
	if(x==mode)
		++q2;
	else
		++q1;

	int l1=max(0, -H);
	
	double ii=mode-1;
	
	
	while(ii>=l1) {
		
		ratio = sym_ratio(k1, k2, H, ii+1);
		ri /= ratio;
		
		q1+=ri;
		if(q1> 1e280)
			return cum_hyper_right(x, k2, tm, k1);
		

		
		
		if(ri<log_table_pr*q1)
			break;
		
		--ii;
		

	}
	
	
	
	/*double cum1=exp(log_symmetric_eq(k1, k2, H, l1));
	double lg0=log_symmetric_eq(k1, k2, H, l1);
	spdout<<"x: "<<l1<<" "<<exp(log_symmetric_eq(k1, k2, H, l1))<<" "<<exp(lg0)<<"\n";*/
	ri=1;
	ii=mode+1;
	//for(double i=mode+1; i<x; i++) 
	while(ii<x) {
		
		
		ratio = sym_ratio(k1, k2, H, ii);
		ri *= ratio;
		
		q1+=ri;
		if(q1> 1e280)
			return cum_hyper_right(x, k2, tm, k1);
		
		if(ri<log_table_pr*q1)
			break;
		//spdout<<ii<<" "<<ratio<<" "<<ri/q1<<" b"<<"\n";;
		++ii;
		
		//spdout<<"dx-->: "<<ii<<" "<<exp(log_symmetric_eq(k1, k2, H, ii))<<" "<<exp(lg0) * ri<<" "<<sym_ratio(k1, k2, H, ii+1)<<"\n";

	}
	
	
	
	ii=max(x, mode+1);
	ri=exp(log_symmetric_eq(k1, k2, H, cast_int(ii-1)) - log_symmetric_eq(k1, k2, H, mode));
	
	//for(double i=max(x, mode+1); i<=k1; i++) 
	
	while(ii<=k1) {
		
		ratio = sym_ratio(k1, k2, H, ii);
		ri *= ratio;
		q2+=ri;
		if(q2> 1e280)
			return cum_hyper_right(x, k2, tm, k1);
		//spdout<<ii<<" "<<ratio<<" "<<ri/q2<<" c "<<x<<" "<<q2<<"\n";;
		++ii;
		if(ri<log_table_pr*q2)
			break;

		
		//spdout<<"ddx-->: "<<i<<" "<<exp(log_symmetric_eq(k1, k2, H, i))<<" "<<exp(lg0) * ri<<" "<<sym_ratio(k1, k2, H, i+1)<<"\n";

	}


	
	/* spdout<<"fast q12: "<<q1<<" "<<q2<<"\n";*/
	
	return max(q2/(q1+q2), 1e-100);
	

}









}
