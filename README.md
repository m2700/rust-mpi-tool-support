# rust-mpi-tool-support

## How to build the example tools:
```
cd tool
cargo build
```
- the result will appear in tool/target/debug/*
- \*\_pmpi.dylib can be used by directly linking the application to it
- \*\_qmpi.dylib can be used through qmpi

### The compiler needs to be able to find qmpi in your system:
+ make sure libqmpi.a and qmpi.h are both in a directory where the compiler can find them
+ setting the following variables can help, depending on your system: CPATH, DYLD_LIBRARY_PATH, LD_LIBRARY_PATH, LIBRARY_PATH
