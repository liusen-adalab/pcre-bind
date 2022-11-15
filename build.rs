fn main() {
    println!(r"cargo:rustc-link-search=native=/home/sen/rust/pcre");

    // cc::Build::new()
    //     .file("src/double.c")
    //     .compile("libdouble2.a");
}
