extern crate cmake;
extern crate bindgen;

use std::env;
use std::path::Path;

fn out() -> String {
    String::from(env::var("OUT_DIR").unwrap())
}

fn fetch() -> String {
    match env::var("WIIUSE_PATH") {
        Ok(root) => root,
        Err(..) => {
            let repo = env::var("WIIUSE_REPO")
                .unwrap_or(String::from("https://github.com/wiiuse/wiiuse.git"));
 
            let root = format!("{}/wiiuse", out());
            if !Path::new(&root).exists() {

                let success = std::process::Command::new("git")
                    .args(&["-C", &out(), "clone", &repo])
                    .status()
                .expect("Failed to execute git")
                .success();

                if !success {
                    panic!("Failed to fetch wiiuse repository: {}", repo);
                }
            }

            root
        }
    }
}

fn compile(root: &str) {
    let dst = cmake::Config::new(root)
        .define("BUILD_WIIUSE_SHARED_LIB", "NO")
        .build_target("wiiuse")
        .build();

    println!("cargo:rustc-link-search=native={}/build/src", dst.display());
    println!("cargo:rustc-link-lib=static=wiiuse");
}

fn gen_bindings(root: &str) {
    let header = format!("{}/src/wiiuse.h", root);
    
    let bindings = bindgen::builder()
        .header(header)
        .generate()
        .expect("Failed to generate bindings!");

    bindings
        .write_to_file(format!("{}/bindings.rs", out()))
        .expect("Failed to write bindings");
}

fn main() {
    let root = fetch();
    compile(&root);
    gen_bindings(&root);
}