#include "iostream"
#include "myout.h"

 bool LogStream::verbose = false;
LogStream spdout;


extern "C"
void set_spdlog_verbose(bool b)
{
    LogStream::verbose = b;
}

