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
    println!("cargo:rustc-link-lib=m");

    if !cnum_bindings_path.exists() || mtime!(&cnum_bindings_path) < mtime!("build.rs") {
        bindgen::builder()
            .header_contents("cnum.h", "#include <complex.h>")
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: true,
            })
            .generate_comments(true)
            .derive_eq(true)
            .whitelist_function("cacosf")
            .whitelist_function("cacos")
            .whitelist_function("casinf")
            .whitelist_function("casin")
            .whitelist_function("catanf")
            .whitelist_function("catan")
            .whitelist_function("ccosf")
            .whitelist_function("ccos")
            .whitelist_function("csinf")
            .whitelist_function("csin")
            .whitelist_function("ctanf")
            .whitelist_function("ctan")
            .whitelist_function("cacoshf")
            .whitelist_function("cacosh")
            .whitelist_function("casinhf")
            .whitelist_function("casinh")
            .whitelist_function("catanhf")
            .whitelist_function("catanh")
            .whitelist_function("ccoshf")
            .whitelist_function("ccosh")
            .whitelist_function("csinhf")
            .whitelist_function("csinh")
            .whitelist_function("ctanhf")
            .whitelist_function("ctanh")
            .whitelist_function("clogf")
            .whitelist_function("clog")
            .whitelist_function("cabsf")
            .whitelist_function("cabs")
            .whitelist_function("cpowf")
            .whitelist_function("cpow")
            .whitelist_function("csqrtf")
            .whitelist_function("csqrt")
            .whitelist_function("cexpf")
            .whitelist_function("cexp")
            .whitelist_function("cimagf")
            .whitelist_function("cimag")
            .whitelist_function("crealf")
            .whitelist_function("creal")
            .whitelist_function("conjf")
            .whitelist_function("conj")
            .whitelist_function("cprojf")
            .whitelist_function("cproj")
            .whitelist_function("cargf")
            .whitelist_function("carg")
            .generate()
            .unwrap()
            .write_to_file(cnum_bindings_path)
            .unwrap();
    }
}
