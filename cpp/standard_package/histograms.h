


#if !defined(HISTOGRAMS_INCLUDED)
#define HISTOGRAMS_INCLUDED



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



int intlog_binning(deque<int> c, int number_of_bins, deque<double> & Xs, deque<double> & Ys, deque<double> & var);



template <typename type>
int xybinning(deque<type> &c, deque<type> &d, int number_of_bins, deque<double> & xs, deque<double> & ys, deque<double> & var, deque<int> & nums) {		
	
	
	// so, this function takes two datasets (c and d) and gathers the data in bin, takes xs and ys as the average in each bin, var is the variance of the y average 
	// the difference with the same stuff called not_norm_histogram is that the other one averages x with y weights.
	
	xs.clear();
	ys.clear();
	var.clear();
	nums.clear();
	
	double min=double(c[0]);
	double max=double(c[0]);
	
	for (int i=0; i<int(c.size()); i++) {
		
		if (min>double(c[i]))
			min=double(c[i]);
		
		if (max<double(c[i]))
			max=double(c[i]);
		
	}
	
	
	
	min-=1e-6;
	max+=1e-6;
	
	
	
			
	if (max==min)
		max+=1e-3;
	
	
	
	deque<deque<double> > hist_x;		// x values in the bin
	deque<deque<double> >  hist_y;		// y values in the bin
		
	double step=min;
	double bin=(max-min)/number_of_bins;		// bin width
	
	deque<double> f;
	while (step<=max+2*bin) {
	
		hist_x.push_back(f);			
		hist_y.push_back(f);			
		step+=bin;
	}
	

	
		
	
	
	for (int i=0; i<int(c.size()); i++) {
		
		
		
		double data=double(c[i]);
		
		if (data>=min && data<=max) {
			
			
			
			int index=int((data-min)/bin);		
			//cout<<data<<" "<<exp(data)<<" "<<index<<endl;
				
			hist_x[index].push_back(double(c[i]));
			hist_y[index].push_back(double(d[i]));
		
		}
		
	}
	
	for (int i=0; i<hist_x.size()-1; i++) {
		
		
		
				
		double x=average_func(hist_x[i]);
		double y=average_func(hist_y[i]);
		
		
		
		//cout<<x<<" "<<exp(x)<<" "<<y<<endl;
		
		if (hist_y[i].size()>0) {
			xs.push_back(x);
			ys.push_back(y);
			var.push_back(variance_func(hist_y[i])/double(hist_y[i].size()));
			nums.push_back(hist_y[i].size());
			
			
			
			//cout<<x<<" "<<exp(x)<<" "<<y<<" "<<(hist_y[i].size())<<" "<<variance_func(hist_y[i])<<endl;

			
		}
		
		
	
	}
	
	
		
	
	for(int i=0; i<var.size(); i++)
		if(var[i]<1e-8)
			var[i]=1e-8;
	
	



	return 0;

}






template <typename type>
int xybinning(deque<type> &c, deque<type> &d, int number_of_bins, deque<double> & xs, deque<double> & ys, deque<double> & var) {

	
	deque<int> nums;
	return xybinning(c, d, number_of_bins, xs, ys, var, nums);

}



void compute_quantiles(double q, deque<double> & y, deque<double> & qs);

template <typename type>
int xybinning_quantiles(deque<type> &c, deque<type> &d, int number_of_bins, deque<double> & xs, deque<double> & ys, deque<double> & var, deque<int> & nums, deque<deque<double> > & Mq, double qa, double qb) {		
	
	
	// so, this function takes two datasets (c and d) and gathers the data in bin, takes xs and ys as the average in each bin, var is the variance of the y average 
	// the difference with the same stuff called not_norm_histogram is that the other one averages x with y weights.
	
	xs.clear();
	ys.clear();
	var.clear();
	nums.clear();
	Mq.clear();
	
	
	double min=double(c[0]);
	double max=double(c[0]);
	
	for (int i=0; i<int(c.size()); i++) {
		
		if (min>double(c[i]))
			min=double(c[i]);
		
		if (max<double(c[i]))
			max=double(c[i]);
		
	}
	
	
	
	min-=1e-6;
	max+=1e-6;
	
	
	
			
	if (max==min)
		max+=1e-3;
	
	
	
	deque<deque<double> > hist_x;		// x values in the bin
	deque<deque<double> >  hist_y;		// y values in the bin
		
	double step=min;
	double bin=(max-min)/number_of_bins;		// bin width
	
	deque<double> f;
	while (step<=max+2*bin) {
	
		hist_x.push_back(f);			
		hist_y.push_back(f);			
		step+=bin;
	}
	

	
		
	
	
	for (int i=0; i<int(c.size()); i++) {
		
		
		
		double data=double(c[i]);
		
		if (data>=min && data<=max) {
			
			
			
			int index=int((data-min)/bin);		
			//cout<<data<<" "<<exp(data)<<" "<<index<<endl;
				
			hist_x[index].push_back(double(c[i]));
			hist_y[index].push_back(double(d[i]));
		
		}
		
	}
	
	for (int i=0; i<hist_x.size()-1; i++) {
		
		
		
				
		double x=average_func(hist_x[i]);
		double y=average_func(hist_y[i]);
		
		
		
		//cout<<x<<" "<<exp(x)<<" "<<y<<endl;
		
		if (hist_y[i].size()>0) {
			xs.push_back(x);
			ys.push_back(y);
			var.push_back(variance_func(hist_y[i])/double(hist_y[i].size()));
			nums.push_back(hist_y[i].size());
			sort(hist_y[i].begin(), hist_y[i].end());
			
			deque<double> qs;
			compute_quantiles(qa, hist_y[i], qs);
			compute_quantiles(qb, hist_y[i], qs);
			
			Mq.push_back(qs);
			//cout<<x<<" "<<exp(x)<<" "<<y<<" "<<(hist_y[i].size())<<" "<<variance_func(hist_y[i])<<endl;

			
		}
		
		
	
	}
	
	
		
	
	for(int i=0; i<var.size(); i++)
		if(var[i]<1e-8)
			var[i]=1e-8;
	
	



	return 0;

}

template <typename type>
int log_histogram(deque<type> &c, ostream & out, int number_of_bins) {		// c is the set od data, min is the lower bound, max is the upper one
	
	
	
	deque <type> d;
	for(int i=0; i<int(c.size()); i++) if (c[i]>0)
		d.push_back(c[i]);
	
	c.clear();
	c=d;
	
	double min=double(c[0]);
	double max=double(c[0]);
	
	for (int i=0; i<int(c.size()); i++) {
		
		if (min>double(c[i]))
			min=double(c[i]);
		
		if (max<double(c[i]))
			max=double(c[i]);
		
	}
	
	
	
	
	deque <int> hist;
	deque <double> hist2;
	deque <double> bins;
	double step=log(min);
	if (max==min)
		max++;
	
	double bin=(log(max)-log(min))/number_of_bins;		// bin width
	
		

	while (step<=log(max)+2*bin) {
		
		
		bins.push_back(exp(step));
		hist.push_back(0);			
		hist2.push_back(0);			
		step+=bin;
	}
	
	
	for (int i=0; i<int(c.size()); i++) {
		
		
		int index=bins.size()-1;
		for (int j=0; j<int(bins.size())-1; j++) if(	(fabs(double(c[i])-bins[j])<1e-7) || (	double(c[i])>bins[j]	&&	double(c[i])<bins[j+1]	)	) { 
		// this could be done in a more efficient way
			
			index=j;
			break;
		
		}
		
		//cout<<hist[index]<<" "<<index<<endl;
		
				
		hist[index]++;
		hist2[index]+=double(c[i]);
		
	}
	
	
	
	
	for (int i=0; i<int(hist.size())-1; i++) {
		
		double h1= bins[i];
		double h2= bins[i+1];
		
		
		double x=hist2[i]/hist[i];
		double y=double(hist[i])/(c.size()*(h2-h1));
		
		if (fabs(y)>1e-10)
			out<<x<<"\t"<<y<<endl;		
		
	
	
	}
	
	
	
	return 0;

}





int log_histogram(deque<double> &c, deque<double> &c2, ostream & out, int number_of_bins);




template <typename type>
int histogram (vector <type> &c, ostream & out, int number_of_bins, double b1, double b2) {		

	// this should be OK
	// c is the set of data, b1 is the lower bound, b2 is the upper one (if they are equal, default limits are used)
	
	
	
	double min=double(c[0]);
	double max=double(c[0]);
	
	for (int i=0; i<int(c.size()); i++) {
		
		if (min>double(c[i]))
			min=double(c[i]);
		
		if (max<double(c[i]))
			max=double(c[i]);
		
	}
	
	
	
	min-=1e-6;
	max+=1e-6;
	
	
	
	if (b1!=b2) {
		
		min=b1;
		max=b2;
	
	}
		
	if (max==min)
		max+=1e-3;
	
	
	
	deque <int> hist;
	deque <double> hist2;
		
	double step=min;
	double bin=(max-min)/number_of_bins;		// bin width

	while (step<=max+2*bin) {
	
		hist.push_back(0);			
		hist2.push_back(0);			
		step+=bin;
	}
	

	
		
	
	
	for (int i=0; i<int(c.size()); i++) {
		
		
		
		double data=double(c[i]);
		
		if (data>min && data<=max) {
			
			int index=int((data-min)/bin);		
			
				
			hist[index]++;
			hist2[index]+=double(c[i]);
		
		}
		
	}
	
	for (int i=0; i<int(hist.size())-1; i++) {
		
		
		
				
		double x=hist2[i]/hist[i];
		double y=double(hist[i])/(c.size()*bin);
		
		if (fabs(y)>1e-10)
			out<<x<<"\t"<<y<<endl;
		
	
	}
	
	
	
			
	return 0;

}





template <typename type>
int not_norm_histogram_correlated (deque<type> &c, deque<type> &d, ostream & out, int number_of_bins, double b1, double b2) {		
	
	
	// c is the x axis, d the y, b1 is the lower bound, b2 is the upper one (if they are equal, default limits are used)
	
	
	
	double min=double(c[0]);
	double max=double(c[0]);
	
	for (int i=0; i<int(c.size()); i++) {
		
		if (min>double(c[i]))
			min=double(c[i]);
		
		if (max<double(c[i]))
			max=double(c[i]);
		
	}
	
	
	
	min-=1e-6;
	max+=1e-6;
	
	
	
	if (b1!=b2) {
		
		min=b1;
		max=b2;
	
	}
		
	if (max==min)
		max+=1e-3;
	
	
	
	deque <int> hist;			// frequency in the bin
	deque <double> hist_x;		// x sum in the bin
	deque <double> hist_y;		// y sum in the bin
		
	double step=min;
	double bin=(max-min)/number_of_bins;		// bin width

	while (step<=max+2*bin) {
	
		hist.push_back(0);			
		hist_x.push_back(0);			
		hist_y.push_back(0);			
		step+=bin;
	}
	

	
		
	
	
	for (int i=0; i<int(c.size()); i++) {
		
		
		
		double data=double(c[i]);
		
		if (data>min && data<=max) {
			
			int index=int((data-min)/bin);		
			
				
			hist[index]++;
			hist_x[index]+=double(c[i]);
			hist_y[index]+=double(d[i]);
		
		}
		
	}
	
	for (int i=0; i<int(hist.size())-1; i++) {
		
		
		
				
		double x=hist_x[i]/hist[i];
		double y=hist_y[i]/hist[i];;
		
		if (fabs(y)>1e-10)
			out<<x<<"\t"<<y<<endl;
		
	
	}
	
	
	
			
	return 0;

}




template <typename type>
int histogram (deque <type> &c, ostream & out, int number_of_bins, double b1, double b2) {		

	// this should be OK
	// c is the set of data, b1 is the lower bound, b2 is the upper one (if they are equal, default limits are used)
	
	
	
	double min=double(c[0]);
	double max=double(c[0]);
	
	for (int i=0; i<int(c.size()); i++) {
		
		if (min>double(c[i]))
			min=double(c[i]);
		
		if (max<double(c[i]))
			max=double(c[i]);
		
	}
	
	
	
	min-=1e-6;
	max+=1e-6;
	
	
	
	if (b1!=b2) {
		
		min=b1;
		max=b2;
	
	}
		
	if (max==min)
		max+=1e-3;
	
	
	
	deque <int> hist;
	deque <double> hist2;
		
	double step=min;
	double bin=(max-min)/number_of_bins;		// bin width

	while (step<=max+2*bin) {
	
		hist.push_back(0);			
		hist2.push_back(0);			
		step+=bin;
	}
	

	
		
	
	
	for (int i=0; i<int(c.size()); i++) {
		
		
		
		double data=double(c[i]);
		
		if (data>min && data<=max) {
			
			int index=int((data-min)/bin);		
			
				
			hist[index]++;
			hist2[index]+=double(c[i]);
		
		}
		
	}
	
	for (int i=0; i<int(hist.size())-1; i++) {
		
		
		
				
		double x=hist2[i]/hist[i];
		double y=double(hist[i])/(c.size()*bin);
		
		if (fabs(y)>1e-10)
			out<<x<<"\t"<<y<<endl;
		
	
	}
	
	
	
			
	return 0;

}

//ofstream pout("hist.dat");
//not_norm_histogram(c, pout, 10, 0, 0);


template <typename type>
int not_norm_histogram (vector<type> &c, ostream & out, int number_of_bins, double b1, double b2) {		

	// this should be OK
	// c is the set of data, b1 is the lower bound, b2 is the upper one (if they are equal, default limits are used)
	
	
	
	double min=double(c[0]);
	double max=double(c[0]);
	
	for (int i=0; i<int(c.size()); i++) {
		
		if (min>double(c[i]))
			min=double(c[i]);
		
		if (max<double(c[i]))
			max=double(c[i]);
		
	}
	
	
	
	min-=1e-6;
	max+=1e-6;
	
	
	
	if (b1!=b2) {
		
		min=b1;
		max=b2;
	
	}
		
	if (max==min)
		max+=1e-3;
	
	
	
	deque <int> hist;
	deque <double> hist2;
		
	double step=min;
	double bin=(max-min)/number_of_bins;		// bin width

	while (step<=max+2*bin) {
	
		hist.push_back(0);			
		hist2.push_back(0);			
		step+=bin;
	}
	

	
		
	
	
	for (int i=0; i<int(c.size()); i++) {
		
		
		
		double data=double(c[i]);
		
		if (data>min && data<=max) {
			
			int index=int((data-min)/bin);		
			
				
			hist[index]++;
			hist2[index]+=double(c[i]);
		
		}
		
	}
	
	
	for (int i=0; i<int(hist.size())-1; i++) {
		
		
		
				
		double x=hist2[i]/hist[i];
		double y=double(hist[i])/(c.size());
		
		if (fabs(y)>1e-10)
			out<<x<<"\t"<<y<<endl;
		
	
	}
	
	
	
			
	return 0;

}



template <typename type>
int not_norm_histogram (deque<type> &c, ostream & out, int number_of_bins, double b1, double b2) {		

	// this should be OK
	// c is the set of data, b1 is the lower bound, b2 is the upper one (if they are equal, default limits are used)
	
	
	
	double min=double(c[0]);
	double max=double(c[0]);
	
	for (int i=0; i<int(c.size()); i++) {
		
		if (min>double(c[i]))
			min=double(c[i]);
		
		if (max<double(c[i]))
			max=double(c[i]);
		
	}
	
	
	
	min-=1e-6;
	max+=1e-6;
	
	
	
	if (b1!=b2) {
		
		min=b1;
		max=b2;
	
	}
		
	if (max==min)
		max+=1e-3;
	
	
	
	deque <int> hist;
	deque <double> hist2;
		
	double step=min;
	double bin=(max-min)/number_of_bins;		// bin width

	while (step<=max+2*bin) {
	
		hist.push_back(0);			
		hist2.push_back(0);			
		step+=bin;
	}
	

	
		
	
	
	for (int i=0; i<int(c.size()); i++) {
		
		
		
		double data=double(c[i]);
		
		if (data>min && data<=max) {
			
			int index=int((data-min)/bin);		
			
				
			hist[index]++;
			hist2[index]+=double(c[i]);
		
		}
		
	}
	
	
	for (int i=0; i<int(hist.size())-1; i++) {
		
		
		
				
		double x=hist2[i]/hist[i];
		double y=double(hist[i])/(c.size());
		
		if (fabs(y)>1e-10)
			out<<x<<"\t"<<y<<"\t"<<sqrt(hist[i])/c.size()<<endl;
		
	
	}
	
	
	
			
	return 0;

}


int histogram (deque<double> &c, deque<double> &c2, ostream & out, int number_of_bins, double b1, double b2);

int not_norm_histogram (deque<double> &c, deque<double> &c2, ostream & out, int number_of_bins, double b1, double b2);
void int_histogram (vector <int> &c, ostream & out);

void int_histogram (deque <int> &c, ostream & out);

void int_histogram(int c, map<int, int> & hist);

void int_histogram(int c, map<int, double> & hist, double w);

int print_cumulative (deque<double> & kws, string file, int number_of_step);

int print_cumulative (deque<int> & kws, string file, int number_of_step);


int print_cumulative (vector<double> & kws, string file, int number_of_step);


int print_cumulative (vector<int> & kws, string file, int number_of_step);





void int_histogram(string infile, string outfile);



void int_histogram(int c, map<int, int> & hist, int w);




void int_histogram(const int & c, map<int, pair<int, double> >  & hist, const int & w1, const double  & w2);


void int_histogram(int c, map<int, pair<double, double> > & hist, double w1, double w2);


void int_histogram(int c, map<int, pair<int, int> > & hist, int w1, int w2);



}

#endif


