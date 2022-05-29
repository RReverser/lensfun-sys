fn main() {
    let mut build_for_sse = "OFF";
    let mut build_for_sse2 = "OFF";
    for target_feature in std::env::var("CARGO_CFG_TARGET_FEATURE")
        .unwrap_or_default()
        .split(',')
    {
        match target_feature {
            "sse" => {
                build_for_sse = "ON";
            }
            "sse2" => {
                build_for_sse2 = "ON";
            }
            _ => {}
        }
    }

    let dst = cmake::Config::new("lensfun")
        .define("INSTALL_HELPER_SCRIPTS", "OFF")
        .define("BUILD_STATIC", "ON")
        .define("BUILD_FOR_SSE", build_for_sse)
        .define("BUILD_FOR_SSE2", build_for_sse2)
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=lensfun");

    autocxx_build::Builder::new("src/lib.rs", &[&dst.join("include/lensfun")])
        .build()
        .unwrap()
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-deprecated-declarations")
        .compile("bindings");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
