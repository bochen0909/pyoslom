#if !defined(CAST_INCLUDED)
#define CAST_INCLUDED	
#include <string>
#include <deque>

namespace oslom{



bool cast_string_to_double (std::string &b, double &h);

double cast_string_to_double(std::string &b);
int cast_int(double u);


int cast_string_to_char(std::string file_name, char *b);


bool cast_string_to_doubles(std::string &b, std::deque<double> & v);

bool cast_string_to_doubles(std::string &b, std::deque<int> & v);
bool separate_strings(std::string &b, std::deque<std::string> & v);

double approx(double a, int digits);


}//end namespace


#endif
