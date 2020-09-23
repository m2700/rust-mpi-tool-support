## 25. June 2020
* Packages:
    - mpi-sys: simple C-bindings to mpi using bindgen for automatic generation
    - qmpi-sys: simple C-Bindings to qmpi, source code can be automatically downloaded and compiled
    - qmpi-tool-creator: consists of the trait `QmpiLayer` and the macro `install_layer!`, which, when both are implemented and used exactly once creates a QMPI-Tool automatically
    - qmpi-test-tool: example QMPI-Tool
    - mpi-test-app: example mpi application, using the reexported version of mpi-sys in qmpi-sys

## 23. September 2020
* the workspace is now seperated into three categories:
    1. app: contains sample mpi applications (mostly for testing purposes)
    2. sys: contains libraries that help interact with the C-libraries MPICH and libm (complex numbers)
    3. tool: contains framework for creating MPI-Tools and a few sample tools
* additional Packages:
    - cnum: support for C's complex types and their operations supported by libm
    - mpi_func-id: contains MPI function id enumeration dereived from QMPI's MPI_funcs type (used for internal purposes)
    - rmpi: high-level MPI library implemented on top of mpi-sys
    - mpi-tool-layer: High-Level and Low-Level interface for creating a generic tool (was inside qmpi-tool-creator earlier)
    - pmpi-tool-creator: same purpose as qmpi-tool-creator for PMPI
* additional Tools:
    - bandwith_recorder: counts primitive elements that are sent or received in an aplication
    - function_counter: atomically counts how often a function has been called across all processes
    - empty-*: empty tool examples for either PMPI or QMPI using either the high-level or raw layer interface.
