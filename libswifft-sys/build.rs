fn main() {
    // build and link
    let dst = cmake::Config::new("src").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=swifft");

    // bindgen
    let bindings = bindgen::Builder::default()
        .header("src/include/swifft.h")
        .generate()
        .expect("Binding generation failed!");
    bindings.write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}