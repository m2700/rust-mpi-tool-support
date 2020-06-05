// #[crate_type = "cdylib"]

use qmpi_tool_creator::{install_layer, QmpiLayer};

struct MyQmpiLayer;
impl QmpiLayer for MyQmpiLayer {
    fn pre_test() {
        println!("pre")
    }
    fn post_test() {
        println!("post")
    }
}

install_layer!(MyQmpiLayer);
