use std::{io, path::PathBuf, process::Command};

fn main() -> io::Result<()> {
    println!(r"cargo:rerun-if-changed=src/double.c");

    let manifest = format!("{}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = format!("{}", std::env::var("OUT_DIR").unwrap());
    let manifest = PathBuf::new().join(manifest);
    let out = PathBuf::new().join(out);

    let source_code = manifest.join("src/double.c");
    let source_code = source_code.to_str().unwrap();
    let _ = Command::new("gcc")
        .current_dir(&out)
        .args(["-Wall", "-c", source_code])
        .output()?;
    let obj_file = out.join("double.o");
    let obj_file = obj_file.to_str().unwrap();
    let _ = Command::new("ar")
        .current_dir(&out)
        .args(["-cvq", "libdouble.a", &obj_file])
        .output()?;

    println!(r"cargo:rustc-link-search=native={}", out.display());
    Ok(())
}
