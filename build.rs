
fn main() -> miette::Result<()> {

    println!("cargo:rustc-link-search=lib");
    println!("cargo:rustc-link-lib=dylib=open3d_wrapper");
    //return Ok(());
    let b = autocxx_build::Builder::new("src/binds.rs", ["include"]).build()?;

    b.compile("bridge-open3d");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.h");

    Ok(())
}