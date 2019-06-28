use std::env;
use std::path;

const ARM64_TARGET: &'static str = "aarch64-linux-android";
const ARM32_TARGET: &'static str = "armv7-linux-androideabi";
const X86_TARGET: &'static str = "i686-linux-android";

fn get_link_dir_name(target: &str) -> &'static str {
    match target {
        ARM64_TARGET => "arm64-v8a",
        ARM32_TARGET => "armeabi-v7a",
        X86_TARGET => "x86",
        _ => panic!(
            "Unknown target: {}. Supported: {:?}",
            target,
            [ARM64_TARGET, ARM32_TARGET, X86_TARGET]
        ),
    }
}

fn main() {
    let target = env::var("TARGET").expect("Cannot get 'TARGET' env var");
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("Cannot get 'CARGO_MANIFEST_DIR' env var");

    let dir_name = get_link_dir_name(&target);

    let dir_path: path::PathBuf = [&manifest_dir, "..", "jniLibs", dir_name].iter().collect();
    let dir_path = dir_path.as_path();
    if !dir_path.is_dir() {
        panic!("{:?} is not a dir", dir_path);
    }

    println!("cargo:rustc-flags=-L {}", dir_path.to_str().unwrap());
}
