

#include <string.h>
//#include <experimental/filesystem>
//namespace fs = std::experimental::filesystem;
#include "filesystem.hpp"
namespace fs = std::filesystem;

int clean(int retcode)
{
	if (paras != 0)
	{
		delete paras;
	}
	if (LOG_TABLE != 0)
	{
		delete LOG_TABLE;
	}
	return retcode;
}

int main_function(const std::vector<std::string> &args)
{
	int argc = (int)args.size();

	#ifdef _WIN32
		char array[128][4096] = {{0}};
	#elif defined(__APPLE__)
		char array[128][4096] = {{0}};
	#else
		char array[argc][4096] = {{0}};
	#endif


	for (int i = 0; i < argc; i++)
	{
		strcpy(array[i], args.at(i).c_str());
	}

	char *argv[20];
	for (int i = 0; i < argc; i++)
	{
		argv[i] = array[i];
	}

	paras = new Parameters();
	LOG_TABLE = new log_fact_table(prog_right_cumulative_function);

	if (argc < 2)
	{
		program_statement(argv[0]);
		return clean(-1);
	}

	if (paras->_set_(argc, argv) == false)
		return clean(-1);

	paras->print();

	string netfile = paras->file1;

	{ /* check if file_name exists */

		#ifdef _WIN32
		        char b[4096+1];
			#else
			        char b[netfile.size()+1];
				#endif

		cast_string_to_char(netfile, b);
		ifstream inb(b);
		if (inb.is_open() == false)
		{

			spdout << "File " << netfile << " not found"
				   << "\n";
			return clean(-1);
		}
	} /* check if file_name exists */

	oslom_net_global luca(netfile);

	if (luca.size() == 0 || luca.stubs() == 0)
	{
		cerr << "network empty"
			 << "\n";
		return clean(-1);
	}

	LOG_TABLE->_set_(cast_int(luca.stubs()));

	char directory_char[1000];
	cast_string_to_char(paras->file1, directory_char);
	char char_to_use[1000];
	sprintf(char_to_use, "%s_oslo_files", directory_char);

	if (fs::exists(char_to_use))
	{
		fs::remove_all(char_to_use);
	}
	fs::create_directories(char_to_use);
	spdout << "output files will be written in directory: " << directory_char << "_oslo_files"
		   << "\n";

	//luca.draw_with_weight_probability("prob");
	oslom_level(luca, directory_char);

	return clean(0);
}
