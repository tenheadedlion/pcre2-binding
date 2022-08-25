fn main() {
    println!("cargo:rustc-link-search=target");
    println!("cargo:rustc-link-lib=static=search");
    println!("cargo:rustc-link-lib=static=pcre2-8");
}
