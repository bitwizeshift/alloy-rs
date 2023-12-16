//! Build script to compile boxer from source, and generate a valid bindgen-C API
//! from boxer.

fn main() {
  compile_boxer();
  generate_boxer_bindings();
}

// Compiles boxer by calling out to CMake.
fn compile_boxer() {
  let mut cfg = cmake::Config::new("src");

  cfg.define("CMAKE_INSTALL_LIBDIR", "lib");

  if cfg!(target_os = "macos") {
    cfg.define("BUILD_SHARED_LIBS", "0");
    build::rustc_link_lib!("static=Boxer");
  } else {
    cfg.define("BUILD_SHARED_LIBS", "1");
    build::rustc_link_lib!("dylib=Boxer");
  }
  let dst = cfg.build();
  build::rustc_link_search!("native={}", dst.join("lib").display());
}

// Generates bindgen bindings for GLFW.
fn generate_boxer_bindings() {
  use std::env;
  use std::path::PathBuf;

  build::rerun_if_changed!("src/boxer_linux.cpp");
  build::rerun_if_changed!("src/boxer_mac.mm");
  build::rerun_if_changed!("src/boxer_win.cpp");
  build::rerun_if_changed!("src/boxer.h");

  let mut builder = bindgen::Builder::default()
    .header("src/boxer.h")
    .clang_arg("-Isrc");

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

  builder.clang_args(&["-isysroot", &sysroot])
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn target_bindgen_args(builder: bindgen::Builder) -> bindgen::Builder {
  builder
}
