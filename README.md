# rust-mpi-tool-support

## How to build everything: run
```
cargo build
```

## How to run the mpi-test-app with the qmpi-test-tool in the qmpi stack:
```
export TOOLS=`realpath target/debug/libqmpi_test_tool.dylib`
cd mpi-test-app
mpiexec cargo run
```
(it might be libqmpi_test_tool.so for linux)
