## 25. June 2020
* Packages:
    - mpi-sys: simple C-bindings to mpi using bindgen for automatic generation
    - qmpi-sys: simple C-Bindings to qmpi, source code can be automatically downloaded and compiled
    - qmpi-tool-creator: consists of the trait `QmpiLayer` and the macro `install_layer!`, which, when both are implemented and used exactly once creates a QMPI-Tool automatically
    - qmpi-test-tool: example QMPI-Tool
    - mpi-test-app: example mpi application, using the reexported version of mpi-sys in qmpi-sys

## TODO:
    - seperate crates in pmpi and qmpi versions that include!() the same code with different #[cfg()] configurations
    - see if tool-creator maybe works with both features at the moment
