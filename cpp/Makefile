# Compiler settings
CXX = g++
CPPFLAGS = -Wall -std=c++17 -I.

# Executable name
TARGET = program

# Find all cpp files in current directory, excluding specified files
SRCS = $(filter-out oslom_python_dir.cpp oslom_python_undir.cpp, $(wildcard *.cpp))
# Generate .o file names from .cpp files
OBJS = $(SRCS:.cpp=.o)

# Default target - creates the executable
all: $(OBJS)

# Rule to compile .cpp files into .o files
%.o: %.cpp
	$(CXX) $(CPPFLAGS) -c $< -o $@

# Clean target to remove all object files and executable
clean:
	rm -f $(OBJS)

.PHONY: all clean 