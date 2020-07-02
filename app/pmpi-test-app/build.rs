

fn main() {

        use std::path::Path;

        println!(
            "cargo:rustc-link-search={}",
            Path::new("..")
                .join("..")
                .join("tool")
                .join("target")
                .join("debug")
                .canonicalize()
                .unwrap()
                .display()
        );
        println!("cargo:rustc-link-lib=mpi_test_tool");

}
