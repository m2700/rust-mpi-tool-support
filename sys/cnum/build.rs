use std::{env, fs::metadata, path::PathBuf};

macro_rules! mtime {
    ($fp:expr) => {
        metadata($fp).unwrap().modified().unwrap()
    };
}

fn main() {
    let out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();

    let cnum_bindings_path = out_dir.join("cnum_bindings.rs");

    println!(
        "cargo:rustc-env=C_NUM_BINDINGS={}",
        cnum_bindings_path.display()
    );

    if !cnum_bindings_path.exists() || mtime!(&cnum_bindings_path) < mtime!("build.rs") {
        bindgen::builder()
            .header_contents("cnum.h", "#include <complex.h>")
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: true,
            })
            .generate_comments(true)
            .derive_eq(true)
            .blacklist_function("cacosl")
            .blacklist_function("casinl")
            .blacklist_function("catanl")
            .blacklist_function("ccosl")
            .blacklist_function("csinl")
            .blacklist_function("ctanl")
            .blacklist_function("cacoshl")
            .blacklist_function("casinhl")
            .blacklist_function("catanhl")
            .blacklist_function("ccoshl")
            .blacklist_function("csinhl")
            .blacklist_function("ctanhl")
            .blacklist_function("clogl")
            .blacklist_function("cabsl")
            .blacklist_function("cpowl")
            .blacklist_function("csqrtl")
            .blacklist_function("cexpl")
            .blacklist_function("cimagl")
            .blacklist_function("creall")
            .blacklist_function("conjl")
            .blacklist_function("cprojl")
            .blacklist_function("cargl")
            .generate()
            .unwrap()
            .write_to_file(cnum_bindings_path)
            .unwrap();
    }
}
