use std::path::PathBuf;

use bindgen::Builder;

fn main() {
    let out_path = PathBuf::from(std::env::var("BINDGEN_OUT").expect("BINDGEN_OUT is not given"));

    let thirdparty_path = std::env::var("THIRDPARTY").unwrap();

    Builder::default()
        .header("src/wrapper.h")
        .clang_arg(format!("-I{}/rcl/rcl/include", thirdparty_path))
        .clang_arg(format!("-I{}/rcutils/rcutils/include", thirdparty_path))
        .clang_arg(format!("-I{}/rmw/rmw/include", thirdparty_path))
        .clang_arg(format!(
            "-I/opt/ros/{}/include",
            std::env::var("ROS_DISTRO").unwrap()
        ))
        .allowlist_type("rcl_.*")
        .allowlist_type("rmw_.*")
        .allowlist_type("rcutils_.*")
        .allowlist_type("RCUTILS_.*")
        .allowlist_function("rcl_.*")
        .allowlist_function("rmw_.*")
        .allowlist_function("rcutils_.*")
        .allowlist_var("RCL_.*")
        .allowlist_var("RMW_.*")
        .allowlist_var("RCUTILS_.*")
        .allowlist_var("g_rcutils_.*")
        .layout_tests(false)
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
