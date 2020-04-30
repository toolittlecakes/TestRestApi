fn main() {
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/transformation/opencv_resize.cpp");
    println!("cargo:rerun-if-changed=src/transformation/mod.rs");


    // cc:Build does not allow to create .so library.
    // Creating static library with static binding to OpenCV seems stupid.
    // So, there is some workaround here with Command::new and Makefile

    #[cfg(debug_assertions)]
    std::process::Command::new("make")
        .arg("install_debug")
        .output()
        .expect("failed make");

    #[cfg(not (debug_assertions))]
    std::process::Command::new("make")
        .arg("install_release")
        .output()
        .expect("failed make");

    std::process::Command::new("make")
        .arg("clean")
        .output()
        .expect("failed make");

}