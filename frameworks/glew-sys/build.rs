//! Build script to compile GLFW from source, and generate a valid bindgen-C API
//! from GLFW.

use std::{
  env,
  path::{Path, PathBuf},
};

fn main() {
  compile_glew();
  generate_glew_bindings();

  if cfg!(any(target_os = "macos", target_os = "ios")) {
    build::rustc_link_lib!("framework=OpenGL");
  } else {
    build::rustc_link_lib!("native=opengl")
  }
}

// Compiles GLFW by calling out to CMake.
fn compile_glew() {
  let mut gen_ext_cmd = std::process::Command::new("make");
  let _ = gen_ext_cmd
    .arg("extensions")
    .current_dir("../../3rd-party/glew");
  if let Err(e) = gen_ext_cmd.status() {
    panic!("error generating extension: {}", e)
  }

  let dst = cmake::Config::new("../../3rd-party/glew/build/cmake").build();

  build::rustc_link_search!("native={}", dst.join("lib").display());
  match std::env::var("PROFILE").unwrap().as_str() {
    "debug" => build::rustc_link_lib!("static=GLEWd"),
    "release" => build::rustc_link_lib!("static=GLEW"),
    _ => {}
  }

  if cfg!(any(target_os = "macos", target_os = "ios")) {
    build::rustc_link_lib!("framework=OpenGL");
  } else {
    build::rustc_link_lib!("dylib=GL");
  }
}

// Generates bindgen bindings for GLFW.
fn generate_glew_bindings() {
  let mut builder =
    bindgen::Builder::default().header(format!("{}/include/GL/glew.h", build::out_dir!()));

  builder = target_bindgen_args(builder);

  let bindings = builder
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(build::out_dir!());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn target_bindgen_args(builder: bindgen::Builder) -> bindgen::Builder {
  let sysroot = build::apple::target_xcode_sysroot();

  builder
    .clang_args(&["-isysroot", &sysroot])
    .clang_args(&["-framework", "OpenGL"])
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn target_bindgen_args(builder: bindgen::Builder) -> bindgen::Builder {
  builder
}
