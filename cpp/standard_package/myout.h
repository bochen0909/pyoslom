#if !defined(MYOUT_INCLUDED)
#define MYOUT_INCLUDED
#include <string>
#include <sstream>
#include <iostream>


class LogStream : public std::ostream
{

public:
    static bool verbose;

    LogStream(){}

    template <typename A>
    LogStream &operator<<(A &lhs)
    {
        auto &out = *this;
        std::stringstream ss;
        ss << lhs;
        out.buff += ss.str();
        size_t pos = 0;
        std::string token;
        std::string d = "\n";
        while ((pos = out.buff.find(d)) != std::string::npos)
        {
            token = out.buff.substr(0, pos);
            out.log(token);
            out.buff.erase(0, pos + d.length());
        }

        return out;
    }

    LogStream &operator<<(const char *lhs)
    {
        return this->operator<<<const char *>(lhs);
    }

    LogStream &operator<<(char *lhs)
    {
        return this->operator<<<char *>(lhs);
    }

   

    LogStream &operator<<(unsigned int lhs)
    {
        return this->operator<<<unsigned int>(lhs);
    }

    LogStream &operator<<(long int lhs)
    {
        return this->operator<<<long int>(lhs);
    }
    LogStream &operator<<(int lhs)
    {
        return this->operator<<<int>(lhs);
    }
    LogStream &operator<<(size_t lhs)
    {
        return this->operator<<<size_t>(lhs);
    }
    LogStream &operator<<(double lhs)
    {
        return this->operator<<<double>(lhs);
    }

    LogStream &operator<<(std::string &lhs)
    {
        return this->operator<<<std::string>(lhs);
    }

    LogStream &operator<<(char lhs)
    {
        return this->operator<<<char>(lhs);
    }

    LogStream &operator<<(unsigned char lhs)
    {
        return this->operator<<<unsigned char>(lhs);
    }

    LogStream &operator<<(signed char lhs)
    {
        return this->operator<<<signed char>(lhs);
    }


    void log(const std::string &msg)
    {
        if (LogStream::verbose) {
            
            std::stringstream ss;
            ss << "[info] " << msg;
            
            std::cout << ss.str() << std::endl;
        }
    }

    virtual ~LogStream()
    {
        #ifdef __APPLE__
            
        #else
            if (!buff.empty())
            {
                this->log(buff);
            }
            buff = "";
        #endif
    }

protected:
    std::string buff;
};

extern "C" void set_spdlog_verbose(bool b);
extern LogStream spdout;

#endif