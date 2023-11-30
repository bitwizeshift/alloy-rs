use std::env;
use std::path::PathBuf;

fn main() {
  compile_imgui();
  generate_imgui_bindings();
}

// Compiles imgui by calling out to CMake.
fn compile_imgui() {
  let mut cfg = cmake::Config::new("src");

  let dst = cfg.build();

  build::rustc_link_search!("native={}", dst.join("lib").display());
  build::rustc_link_lib!("static=imgui");
  build::rustc_link_lib!("c++")
}

// Generates bindgen bindings for imgui.
fn generate_imgui_bindings() {
  build::rerun_if_changed!("../../3rd-party/cimgui/cimgui.h");

  let mut builder = bindgen::Builder::default()
    .header("../../3rd-party/cimgui/cimgui.h")
    .clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS=1")
    .clang_arg("-DIMGUI_DISABLE_OBSOLETE_FUNCTIONS=1");

  builder = target_bindgen_args(builder);

  let bindings = builder
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("imgui.rs"))
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
