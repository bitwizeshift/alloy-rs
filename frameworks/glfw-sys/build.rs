//! Build script to compile GLFW from source, and generate a valid bindgen-C API
//! from GLFW.

fn main() {
  compile_glfw();
  generate_glfw_bindings();
}

// Compiles GLFW by calling out to CMake.
fn compile_glfw() {
  let mut cfg = cmake::Config::new("../../3rd-party/glfw");

  cfg
    .define("GLFW_BUILD_EXAMPLES", "OFF")
    .define("GLFW_BUILD_TESTS", "OFF")
    .define("GLFW_BUILD_DOCS", "OFF")
    .define("CMAKE_INSTALL_LIBDIR", "lib");

  let dst = if cfg!(feature = "wayland") {
    cfg.define("GLFW_BUILD_WAYLAND", "ON").build()
  } else {
    cfg.define("GLFW_BUILD_WAYLAND", "OFF").build()
  };

  println!(
    "cargo:rustc-link-search=native={}",
    dst.join("lib").display()
  );
  println!("cargo:rustc-link-lib=static=glfw3");

  if cfg!(any(target_os = "macos", target_os = "ios")) {
    build::rustc_link_lib!("framework=Cocoa");
    build::rustc_link_lib!("framework=IOKit");
    build::rustc_link_lib!("framework=CoreFoundation");
    build::rustc_link_lib!("framework=OpenGL");
  }
}

// Generates bindgen bindings for GLFW.
fn generate_glfw_bindings() {
  use std::env;
  use std::path::PathBuf;

  build::rerun_if_changed!("../../3rd-party/glfw/include/GLFW/glfw3.h");
  build::rerun_if_changed!("../../3rd-party/glfw/include/GLFW/glfw3native.h");

  let mut builder = bindgen::Builder::default()
    .header("../../3rd-party/glfw/include/GLFW/glfw3.h")
    .header("../../3rd-party/glfw/include/GLFW/glfw3native.h")
    .clang_arg("-I../../3rd-party/glfw/include");

  builder = target_bindgen_args(builder);

  let bindings = builder
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn target_bindgen_args(builder: bindgen::Builder) -> bindgen::Builder {
  let sysroot = build::apple::target_xcode_sysroot();

  builder
    .clang_args(&["-isysroot", &sysroot])
    .clang_args(&["-framework", "Cocoa"])
    .clang_args(&["-framework", "IOKit"])
    .clang_args(&["-framework", "CoreFoundation"])
    .clang_args(&["-framework", "OpenGL"])
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn target_bindgen_args(builder: bindgen::Builder) -> bindgen::Builder {
  builder
}
