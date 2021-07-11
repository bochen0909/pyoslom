#include "iostream"
#include "myout.h"

SpdlogStream<spdlog::level::info> spdout;
SpdlogStream<spdlog::level::err> spderr;

extern "C"
void set_spdlog_verbose(bool b)
{
    if (b)
    {
        spdlog::set_level(spdlog::level::info);
    }
    else
    {
        spdlog::set_level(spdlog::level::err);
    }
}


int main_test(int argc, char *argv[])
{
    printf("log level: %d\n", spdlog::get_level());

    spderr << "this is a error"
           << "\n";

    std::string s(" this is a string ");
    spdout << "hello " << 1234 << " and " << 231.2 << s << "\n";

    return 0;
}