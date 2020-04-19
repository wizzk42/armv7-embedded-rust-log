use std::{
    env,
    error::Error,
    fs::File,
    io::Write,
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET")?;

    has_fpu(&target);

    let out = PathBuf::from(
        env::var_os("OUT_DIR")
            .unwrap()
    );

    println!("target: {}", target);
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");

    File::create(out.join("log.x"))?
        .write_all(include_bytes!("scripts/linker/log.x.in"))?;
    println!("cargo:rerun-if-changed=log.x.in");

    Ok(())
}

fn has_fpu(target: &str) {
    if target.ends_with("eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}
