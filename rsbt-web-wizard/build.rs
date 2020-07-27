use actix_web_static_files::NpmBuild;

fn main() {
    NpmBuild::new("./web")
        .install()
        .unwrap()
        .run("build")
        .unwrap()
        .target("./web/dist")
        .to_resource_dir()
        .with_generated_fn("generated_web_wizard")
        .build()
        .unwrap();
    println!("cargo:rerun-if-changed=web/package-lock.json");
    println!("cargo:rerun-if-changed=web/package.json");
    println!("cargo:rerun-if-changed=web/index.html");
    println!("cargo:rerun-if-changed=web/index.ts");
    println!("cargo:rerun-if-changed=build.rs");
}
