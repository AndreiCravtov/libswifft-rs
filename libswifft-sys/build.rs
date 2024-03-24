fn main() {
    // build and link
    let dst = cmake::build("src");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=swifft");

    // bindgen
    let bindings = bindgen::Builder::default()
        .header("src/include/swifft.h")
        // .header(dst.clone().join("include/swifft_api.inl").to_str().unwrap())
        .generate()
        .expect("Binding generation failed!");
    bindings.write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}