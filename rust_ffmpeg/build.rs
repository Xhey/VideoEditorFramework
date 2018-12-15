extern crate dunce;
use std::{env, path::PathBuf};

fn main() {
    config_fflib();
}

fn config_fflib() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir = dunce::canonicalize(root.join("libs")).unwrap();
    println!("cargo:rustc-link-search=native={}", env::join_paths(&[lib_dir]).unwrap().to_str().unwrap());
    println!("cargo:rustc-link-lib=static={}", "avutil");
    println!("cargo:rustc-link-lib=static={}", "avcodec");
    println!("cargo:rustc-link-lib=static={}", "avformat");
    println!("cargo:rustc-link-lib=static={}", "avfilter");
    println!("cargo:rustc-link-lib=static={}", "avdevice");
    println!("cargo:rustc-link-lib=static={}", "swscale");
    println!("cargo:rustc-link-lib=static={}", "swresample");
    println!("cargo:rustc-link-lib=static={}", "postproc");
    println!("cargo:rustc-link-lib=static={}", "x264");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=m");
}