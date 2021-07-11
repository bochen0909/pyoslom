

#include <string.h>
#include <experimental/filesystem>
namespace fs = std::experimental::filesystem;


int clean(int retcode){
	if (paras!=0){
		delete paras;
	}
	if(LOG_TABLE!=0){
		delete LOG_TABLE;
	}
	return retcode;
}


int main_function(const std::vector<std::string> &args) {
    int argc = (int) args.size();
    char array[argc][4096]= {{0}};;
    for (int i=0;i<argc;i++){
        strcpy(array[i],args.at(i).c_str());
    }
	
	char* argv[20];
    for ( int i = 0; i < argc; i++ ){
        argv[i] = array[i];
    }
        
	paras = new Parameters();
	LOG_TABLE = new log_fact_table(prog_right_cumulative_function);
		
		
	if(argc<2) {
		program_statement(argv[0]);
		return clean(-1);
	}

	
	if(paras->_set_(argc, argv)==false)
		return clean(-1);

	paras->print();
	
	string netfile=paras->file1;
	
	
	{	/* check if file_name exists */
		char b[netfile.size()+1];
		cast_string_to_char(netfile, b);
		ifstream inb(b);
		if(inb.is_open()==false) {
			
			cout<<"File "<<netfile<<" not found"<<endl;
			return clean(-1);
		
		}	
	}	/* check if file_name exists */

	
	oslom_net_global luca(netfile);
	
	
	
	
	
	if(luca.size()==0 || luca.stubs()==0) {
		cerr<<"network empty"<<endl;
		return clean(-1);
	}
	
	
	
	LOG_TABLE->_set_(cast_int(luca.stubs()));
	
	
	
	char directory_char[1000];
	cast_string_to_char(paras->file1, directory_char);
	if (fs::exists(directory_char)){
	    fs::remove_all(directory_char);
	}
	fs::create_directories(directory_char);
	cout<<"output files will be written in directory: "<<directory_char<<"_oslo_files"<<endl;
	
	//luca.draw_with_weight_probability("prob");
	oslom_level(luca, directory_char);
	
	
	return clean(0);
	
	

}


