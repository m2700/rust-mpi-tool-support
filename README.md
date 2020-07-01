# rust-mpi-tool-support

## How to build everything: run
```
cargo build
```

## How to run the mpi-test-app with the mpi-test-tool in the qmpi stack:
```
export TOOLS=`realpath tool/target/debug/libmpi_test_tool.dylib`
cd tool/mpi-test-tool
cargo build --features qmpi_mode
cd ../app/mpi-test-app
cargo clean // make sure only one feature is used
cargo run --features link_qmpi
```
(it might be libmpi_test_tool.so for linux)

## How to run the mpi-test-app directly linked to mpi-test-tool as pmpi tool:
```
cd tool/mpi-test-tool
cargo build --features pmpi_mode
cd ../app/mpi-test-app
cargo clean // make sure only one feature is used
cargo run --features link_mpi_test_tool
```
