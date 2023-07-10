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
        ./pdl-compiler/scripts/generate_cxx_backend.py > my-protocol.h

Language bindings
-----------------

Enum declarations
^^^^^^^^^^^^^^^^^

+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: c++                                           |
|                                       |                                                               |
|     enum TestEnum : 8 {               |     enum TestEnum : int8_t {                                  |
|         A = 1,                        |         A = 1,                                                |
|         B = 2..3,                     |         B_MIN = 2,                                            |
|         C = 4,                        |         B_MAX = 3,                                            |
|         OTHER = ..,                   |         C = 4,                                                |
|     }                                 |     }                                                         |
+---------------------------------------+---------------------------------------------------------------+

.. note::
    C++ enums are open by construction, default cases in enum declarations are ignored.
