# rust-mpi-tool-support

## How to build everything: run
```
cargo build
```

## How to run the mpi-test-app with the mpi-test-tool in the qmpi stack:
```
# does not exist at the moment (use another tool)
export TOOLS=`realpath tool/target/debug/libmpi_test_tool.dylib`
cd tool/mpi-test-tool
cargo build --features qmpi_mode

cd ../app/qmpi-test-app
cargo run
```
(it might be libmpi_test_tool.so for linux)

## How to run the mpi-test-app directly linked to mpi-test-tool as pmpi tool:
```
cd tool/mpi-test-tool
cargo build
cd ../app/pmpi-test-app
cargo run
```
