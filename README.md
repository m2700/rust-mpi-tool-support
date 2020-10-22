# rust-mpi-tool-support

## How to build the example tools (with debug symbols):
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
+ the [original qmpi](https://github.com/caps-tum/qmpi) and [my own branch](https://github.com/m2700/qmpi) are both compatible

## Compile with benchmark settings (zero optimization for all tools):
```
cd tool
cargo build --release
```
- the result will now appear in tool/target/release/ instead

## Compile and open library Documentations
There is not much actual Documentation there right now, but it might still help to get a good overwiew over the existing types without searching through the entire source code.
### for MPI libraries
```
cd sys
cargo doc --all-features --open
```
### for Tool libraries
```
cd tool
cargo doc --all-features --open
```
