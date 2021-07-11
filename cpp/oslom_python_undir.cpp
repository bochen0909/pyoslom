/*
 * feaca_main.cpp
 *
 *  Created on: Jun 24, 2021
 *      Author: Bo
 */

#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include "main_undirected.cpp"

namespace py = pybind11;



PYBIND11_MODULE(coslomundir, m)
{
    m.doc() = R"pbdoc(
        OSLOM undirected python wrapper
        -----------------------
    )pbdoc";

    
    m.def(
        "run", &main_function,
        py::arg("args"));

    m.def(
        "set_verbose", &set_spdlog_verbose,
        py::arg("b"));        

}
