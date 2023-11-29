use std::env;
use std::path::PathBuf;

fn main() {
  link_openal();
  generate_openal_bindings();
}

fn link_openal() {
  if cfg!(any(target_os = "macos", target_os = "ios")) {
    build::rustc_link_lib!("framework=OpenAL");
  } else {
    build::rustc_link_lib!("openal")
  }
}

fn generate_openal_bindings() {
  build::rerun_if_changed!("wrapper.h");

  let builder = bindgen::Builder::default().header("wrapper.h");

  let bindings = target_bindgen_args(builder)
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
