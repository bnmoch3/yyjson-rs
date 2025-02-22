use std::env;
use std::path::PathBuf;

fn main() {
    // TODO: currently cannot build for windows
    // let target = env::var("TARGET").unwrap();
    // if target.contains("windows") {
    //     panic!("cannot build for windows");
    // }

    let build_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()); // current dir
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let yyjson_src_dir_path = PathBuf::from("yyjson")
        .canonicalize()
        .expect("cannot canonicalize path")
        .join("src");

    println!("cargo:rerun-if-changed={}", yyjson_src_dir_path.display());

    let yyjson_cflags = vec![
        "-DYYJSON_DISABLE_UTILS=1",
        "-DYYJSON_DISABLE_NON_STANDARD=1",
        "-DYYJSON_DISABLE_WRITER=1",
    ];

    let mut build = cc::Build::new();
    #[allow(clippy::redundant_closure_call)]
    (|build: &mut cc::Build| {
        for flag in yyjson_cflags {
            let (var, val) = flag.split_once('=').unwrap();
            build.define(&var[2..], val);
        }
    })(&mut build);
    build
        .include(".")
        .flag("-Wno-deprecated-declarations")
        .file(build_dir.join("extern.c"))
        .file(yyjson_src_dir_path.join("yyjson.c"))
        .out_dir(out_dir.clone())
        .compile("yyjson");

    // ========================================================================
    // LINK YYJSON
    // ========================================================================

    // where to look for non-standard shared libraries (libyyjson.a)
    println!("cargo:rustc-link-search={}", out_dir.display());

    // link to libyyjson.a
    println!("cargo:rustc-link-lib=static=yyjson");
}
