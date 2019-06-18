use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET")?;
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let name = env::var("CARGO_PKG_NAME")?;

    if target == "thumbv8m.main-none-eabi" {
        fs::copy(
            format!("bin/{}.a", target),
            out_dir.join(format!("lib{}.a", name)),
        )?;

        println!("cargo:rustc-link-lib=static={}", name);
        println!("cargo:rustc-link-search={}", out_dir.display());
    }

    Ok(())
}
