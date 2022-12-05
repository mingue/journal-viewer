fn main() {
    println!("cargo:rustc-link-lib=systemd");
    println!("cargo:rustc-link-search=native=/usr/lib/");
    tauri_build::build()
}
