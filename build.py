import glob
import os
import platform
from pybind11.setup_helpers import Pybind11Extension, build_ext

__version__ = "0.1.6"

is_windows = platform.system() == "Windows"

is_macos = platform.system() == "Darwin"

is_linux = platform.system() == "Linux"

with open("README.md", "r") as fh:
    long_description = fh.read()


def get_cpp_files():
    cppfiles = glob.glob("cpp/*.cpp") + glob.glob("cpp/*/*.cpp")
    
    undircppfiles = [
        u
        for u in cppfiles
        if "oslom_python_dir.cpp" not in u
        and "_dir" not in u
        and not os.path.split(u)[-1].startswith("dir")
        and ("dir_weighted_tabdeg.cpp" not in u or "undir_weighted_tabdeg.cpp" in u)
    ]

    dircppfiles = [
        u
        for u in cppfiles
        if "oslom_python_undir.cpp" not in u
        and "_undir" not in u
        and not os.path.split(u)[-1].startswith("undir")
        and "undir_weighted_tabdeg.cpp" not in u
    ]

    print("undir", " ".join(undircppfiles))
    print("dir", " ".join(dircppfiles))
    return undircppfiles, dircppfiles


def build(setup_kwargs):
    undircppfiles, dircppfiles = get_cpp_files()
    
    extra_compile_args = (
        ["/std:c++17", f"/MP4"]  # /MP flag for MSVC parallel builds
        if is_windows
        else (
            ["-O3", "-std=c++17", "-mmacosx-version-min=10.15"]
            if is_macos
            else ["-O0", "-std=c++17"]
        )
    )

    ext_modules = [
        Pybind11Extension(
            "coslomundir",
            undircppfiles,
            define_macros=[("VERSION_INFO", __version__)],
            include_dirs=["cpp"],
            extra_compile_args=extra_compile_args,
            extra_link_args=["-lstdc++fs"] if is_linux else [],
            language="c++",
        ),
        Pybind11Extension(
            "coslomdir",
            dircppfiles,
            define_macros=[("VERSION_INFO", __version__)],
            include_dirs=["cpp"],
            extra_compile_args=extra_compile_args,
            extra_link_args=["-lstdc++fs"] if is_linux else [],
            language="c++",
        ),
    ]
    
    # Enable parallel build
    setup_kwargs.update(
        {
            "description": "OSLOM graph clustering algorithm",
            "long_description": long_description,
            "long_description_content_type": "text/markdown",
            "ext_modules": ext_modules,
            "cmdclass": {"build_ext": build_ext},
            "zip_safe": False,
            "include_package_data": True,
            "options": {"build": {"parallel": 4}},  # Enable parallel build
        }
    )
