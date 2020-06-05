fn main() {
    println!("cargo:rustc-link-lib=qmpi");
    println!("cargo:rustc-link-lib=pmpi");
    println!("cargo:rustc-link-lib=mpi");
}
