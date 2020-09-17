use std::{
    env,
    fs::{create_dir, metadata},
    io::ErrorKind as IOErrorKind,
    path::{Path, PathBuf},
    process::Command,
    str::from_utf8,
};
#[cfg(any(feature = "use_git_header", feature = "bundled"))]
use std::{fs::File, io::Write};

#[cfg(any(feature = "use_git_header", feature = "bundled"))]
use hyper::{body::HttpBody, Client, Uri};
#[cfg(any(feature = "use_git_header", feature = "bundled"))]
use hyper_rustls::HttpsConnector;
#[cfg(feature = "bundled")]
use tokio::join as tokio_join; // fixes highlightning issue

macro_rules! mtime {
    ($fp:expr) => {
        metadata($fp).unwrap().modified().unwrap()
    };
}

fn prepare_dir(path: &Path) {
    create_dir(path)
        .or_else(|e| match e.kind() {
            IOErrorKind::AlreadyExists => Ok(()),
            _ => Err(e),
        })
        .unwrap();
}

#[cfg(any(feature = "use_git_header", feature = "bundled"))]
async fn download_file(uri: Uri, file_path: &Path, patch: &[[&str; 2]]) {
    if !file_path.exists() || mtime!(file_path) < mtime!("build.rs") {
        let https_client: Client<_, hyper::Body> = Client::builder().build(HttpsConnector::new());

        let mut resp = https_client.get(uri).await.unwrap();
        println!("Response: {}", resp.status());

        let mut file = File::create(file_path).unwrap();
        let mut data = vec![];
        while let Some(chunk) = resp.body_mut().data().await {
            data.extend(chunk.unwrap());
        }
        let mut data = String::from_utf8(data).unwrap();
        for &[pat, repl] in patch {
            data = data.replace(pat, repl);
        }
        file.write_all(data.as_bytes()).unwrap();
    }
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "use_git_header")]
    let qmpi_h_uri = "https://raw.githubusercontent.com/caps-tum/qmpi/master/qmpi.h"
        .parse()
        .unwrap();
    #[cfg(feature = "bundled")]
    let qmpi_c_uri = "https://raw.githubusercontent.com/caps-tum/qmpi/master/qmpi.c"
        .parse()
        .unwrap();
    #[cfg(feature = "bundled")]
    let qmpi_arrays_h_uri = "https://raw.githubusercontent.com/caps-tum/qmpi/master/arrays.h"
        .parse()
        .unwrap();

    let out_dir: PathBuf = env::var("OUT_DIR").unwrap().into();
    let qmpi_root_path = Path::new("qmpi");

    #[cfg(feature = "use_git_header")]
    let qmpi_h_path = qmpi_root_path.join("qmpi.h");
    #[cfg(feature = "bundled")]
    let qmpi_c_path = qmpi_root_path.join("qmpi.c");
    #[cfg(feature = "bundled")]
    let qmpi_arrays_h_path = qmpi_root_path.join("arrays.h");

    let qmpi_bindings_path = out_dir.join("qmpi_bindings.rs");

    println!("cargo:rerun-if-changed={}", qmpi_root_path.display());

    #[cfg(feature = "use_git_header")]
    println!("cargo:rerun-if-changed={}", qmpi_h_path.display());
    #[cfg(feature = "bundled")]
    println!("cargo:rerun-if-changed={}", qmpi_c_path.display());
    #[cfg(feature = "bundled")]
    println!("cargo:rerun-if-changed={}", qmpi_arrays_h_path.display());

    println!(
        "cargo:rustc-env=QMPI_BINDGEN_BINDINGS={}",
        qmpi_bindings_path.display()
    );

    prepare_dir(&qmpi_root_path);

    #[cfg(feature = "bundled")]
    tokio_join!(
        download_file(qmpi_h_uri, &qmpi_h_path, &[]),
        download_file(qmpi_c_uri, &qmpi_c_path, &[]),
        download_file(qmpi_arrays_h_uri, &qmpi_arrays_h_path, &[])
    );
    #[cfg(all(not(feature = "bundled"), feature = "use_git_header"))]
    download_file(qmpi_h_uri, &qmpi_h_path, &[]).await;

    #[cfg(feature = "bundled")]
    cc::Build::new()
        .compiler("mpicc")
        .archiver("ar")
        // .ar_flag("rsv") // doesn't work (??)
        .flag("-std=c99")
        .flag("-g")
        .flag("-Wswitch")
        .file(qmpi_c_path)
        .warnings(false)
        .extra_warnings(false)
        .compile("qmpi");
    #[cfg(all(not(feature = "bundled"), feature = "link_qmpi"))]
    println!("cargo:rustc-link-lib=qmpi");

    if !qmpi_bindings_path.exists() || mtime!(&qmpi_bindings_path) < mtime!("build.rs") {
        let mpicc_output = Command::new("mpicc")
            .arg("-show")
            .output()
            .expect("failed to execute process 'mpicc'");
        let mpicc_args = from_utf8(if mpicc_output.stdout.starts_with(b"clang") {
            &mpicc_output.stdout[b"clang ".len()..]
        } else if mpicc_output.stdout.starts_with(b"gcc") {
            &mpicc_output.stdout[b"gcc ".len()..]
        } else {
            panic!(from_utf8(Box::leak(mpicc_output.stdout.into_boxed_slice())))
        })
        .unwrap();

        #[cfg(feature = "use_git_header")]
        let bindgen_builder = bindgen::builder().header(qmpi_h_path.to_str().unwrap());
        #[cfg(not(feature = "use_git_header"))]
        let bindgen_builder =
            bindgen::builder().header_contents("qmpi_include.h", "#include <qmpi.h>");

        let qmpi_bindings = bindgen_builder
            .clang_args(mpicc_args.split(' '))
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: true,
            })
            .whitelist_var("NUM_MPI_FUNCS")
            .whitelist_type("vector")
            .whitelist_type("_MPI_funcs")
            .whitelist_type("mpi_func");
        #[cfg(feature = "link_qmpi")]
        let qmpi_bindings = qmpi_bindings
            .whitelist_function("vector_get")
            .whitelist_function("QMPI_Table_query");
        qmpi_bindings
            .generate()
            .unwrap()
            .write_to_file(qmpi_bindings_path)
            .unwrap();
    }
}
