use std::{
    env,
    error::Error,
    fs::File,
    io::Write,
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let out = PathBuf::from(
        env::var_os("OUT_DIR").unwrap_or_default()
    );

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");

    File::create(out.join("log.x"))?
        .write_all(include_bytes!("linker/log.x.in"))
        .expect("cannot write log.x linker script");

    println!("cargo:rerun-if-changed=log.x.in");
    Ok(())
}
