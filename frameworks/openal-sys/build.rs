use std::env;
use std::path::PathBuf;

fn main() {
  compile_openal();
  generate_openal_bindings();
}

// Compiles openal by calling out to CMake.
fn compile_openal() {
  let mut cfg = cmake::Config::new("../../3rd-party/openal-soft");

  let dst = cfg.build();

  build::rustc_link_search!("native={}", dst.join("lib").display());
  build::rustc_link_lib!("dylib=openal");
}

// Generates bindgen bindings for openal.
fn generate_openal_bindings() {
  build::rerun_if_changed!("../../3rd-party/openal-soft/include/AL/al.h");
  build::rerun_if_changed!("../../3rd-party/openal-soft/include/AL/alc.h");
  build::rerun_if_changed!("../../3rd-party/openal-soft/include/AL/alext.h");

  let mut builder = bindgen::Builder::default()
    .header("../../3rd-party/openal-soft/include/AL/al.h")
    .header("../../3rd-party/openal-soft/include/AL/alc.h")
    .header("../../3rd-party/openal-soft/include/AL/alext.h")
    .clang_arg("-I../../3rd-party/openal-soft/include");

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
