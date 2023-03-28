fn main() {
    println!("cargo:rustc-link-lib=dylib=systemd");
    println!("cargo:rustc-link-search=native=/usr/lib/");
    tauri_build::build()
}
