C++ Generated Code Guide
========================

Usage
-----

.. sourcecode:: bash

    usage: generate_cxx_backend.py [-h] [--input INPUT] [--output OUTPUT] [--namespace NAMESPACE] [--include-header INCLUDE_HEADER] [--using-namespace USING_NAMESPACE]

    options:
      -h, --help            show this help message and exit
      --input INPUT         Input PDL-JSON source
      --output OUTPUT       Output C++ file
      --namespace NAMESPACE
                            Generated module namespace
      --include-header INCLUDE_HEADER
                            Added include directives
      --using-namespace USING_NAMESPACE
                            Added using namespace statements

Example invocation:

.. sourcecode:: bash

    cargo run my-protocol.pdl --output-format json | \
        ./scripts/generate_cxx_backend.py > my-protocol.h
