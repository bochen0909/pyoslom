#if !defined(MYOUT_INCLUDED)
#define MYOUT_INCLUDED
#include <string>
#include <sstream>
#include "../spdlog/spdlog.h"

template <spdlog::level::level_enum LEVEL>
class SpdlogStream : public std::ostream
{
    template <typename A>
    friend SpdlogStream &operator<<(SpdlogStream &out, A &lhs);

public:
#ifdef _WIN32
    SpdlogStream():std::ostream(std::cout.rdbuf()){}
#endif

    void log(const std::string &msg)
    {
        spdlog::log(LEVEL, msg);
    }

    virtual ~SpdlogStream()
    {
        if (!buff.empty())
        {
            this->log(buff);
        }
        buff = "";
    }

    template <typename A>
    SpdlogStream &operator<<(A &lhs)
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

    SpdlogStream &operator<<(const char *lhs)
    {
        return this->operator<<<const char *>(lhs);
    }

    SpdlogStream &operator<<(char *lhs)
    {
        return this->operator<<<char *>(lhs);
    }

    SpdlogStream &operator<<(int lhs)
    {
        return this->operator<<<int>(lhs);
    }

    SpdlogStream &operator<<(unsigned int lhs)
    {
        return this->operator<<<unsigned int>(lhs);
    }

    SpdlogStream &operator<<(long int lhs)
    {
        return this->operator<<<long int>(lhs);
    }
    SpdlogStream &operator<<(size_t lhs)
    {
        return this->operator<<<size_t>(lhs);
    }
    SpdlogStream &operator<<(double lhs)
    {
        return this->operator<<<double>(lhs);
    }

    SpdlogStream &operator<<(std::string &lhs)
    {
        return this->operator<<<std::string>(lhs);
    }

    SpdlogStream &operator<<(char lhs)
    {
        return this->operator<<<char>(lhs);
    }

    SpdlogStream &operator<<(unsigned char lhs)
    {
        return this->operator<<<unsigned char>(lhs);
    }

    SpdlogStream &operator<<(signed char lhs)
    {
        return this->operator<<<signed char>(lhs);
    }

protected:
    std::string buff;
};

extern "C" void set_spdlog_verbose(bool b);
extern SpdlogStream<spdlog::level::info> spdout;
extern SpdlogStream<spdlog::level::err> spderr;

#endif
