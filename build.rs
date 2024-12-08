use std::fs;
use std::path::Path;

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I/usr/local/include/metal_irconverter")
        .clang_arg("-I/usr/local/include/metal_irconverter_runtime")
        .header("/usr/local/include/metal_irconverter/metal_irconverter.h")
        .size_t_is_usize(true)
        .allowlist_function("IR.*")
        .allowlist_type("IR.*")
        .layout_tests(false)
        .generate()
        .expect("Failed to generate bindings");

    fs::create_dir_all("gen").unwrap();
    fs::write(Path::new("gen/bindings.rs"), bindings.to_string())
        .expect("Failed to write bindings to file");
}

fn main() {
    generate_bindings();

    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=dylib=metalirconverter");
}
