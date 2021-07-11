
#include <math.h>
#include <fstream>

#include "print.h"

namespace oslom {

void cherr() {
	
	cerr<<"the check failed"<<endl;
	int e;
	cin>>e;
	
}


void cherr(double a) {
	
	cerr<<"the check failed because of "<<a<<endl;
	int e;
	cin>>e;
	
}


void cherr(double a, double ee) {
	
	if(fabs(a)>ee) {
	
		cerr<<"the check failed because of "<<a<<endl;
		int e;
		cin>>e;
	}
	
	
}

 


void get_data_from_file_string(string s, deque<string> & a1, int col) {

	
	// default will be col=1
	
	
	char b[1000];
	cast_string_to_char(s, b);
	ifstream lin(b);
	
	
	a1.clear();
	col--;
	
	string sas;
	while(getline(lin, sas)) {
		
		deque<string> v;
		separate_strings(sas,  v);
		 
		 //prints(v);
		 
		if(int(s.size())>col) {
			a1.push_back(v[col]);
		}
	
	}
	
	

}

}


