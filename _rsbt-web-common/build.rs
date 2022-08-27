use actix_web_static_files::generate_resources;
use std::{env, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let generated_filename = Path::new(&out_dir).join("generated.rs");

    generate_resources("./web", None, generated_filename, "generated_web_common").unwrap();

    println!("cargo:rerun-if-changed=web/img/01.svg");
    println!("cargo:rerun-if-changed=web/img/02.svg");
    println!("cargo:rerun-if-changed=build.rs");
}
