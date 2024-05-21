use std::path::PathBuf;

fn main() {
  build::rerun_if_env_changed!("CC");
  build::rerun_if_env_changed!("CXX");
  build::rerun_if_env_changed!("CARGO_FEATURE_VULKAN");
  build::rerun_if_env_changed!("CARGO_FEATURE_OPENGL");

  build::rerun_if_changed!("../../3rd-party/cimgui");
  build::rerun_if_changed!("../../.gitmodules");
  build::rerun_if_changed!("imgui_glfw.cpp");
  build::rerun_if_changed!("imgui_opengl3.cpp");
  build::rerun_if_changed!("imgui_vulkan.cpp");
  build::rerun_if_changed!("CMakeLists.txt");

  compile_imgui();
  generate_imgui_bindings();
}

// Compiles imgui by calling out to CMake.
fn compile_imgui() {
  let mut cfg = cmake::Config::new("src");

  if cfg!(feature = "vulkan") {
    cfg.define("VULKAN_BACKEND", "ON");
  }
  if cfg!(feature = "opengl") {
    cfg.define("OPENGL_BACKEND", "ON");
  }

  let dst = cfg.build();

  build::rustc_link_search!("native={}", dst.join("lib").display());
  build::rustc_link_lib!("static=imgui");
  if cfg!(any(target_os = "macos", target_os = "ios")) {
    build::rustc_link_lib!("c++");
  } else if cfg!(target_os = "linux") {
    build::rustc_link_lib!("stdc++");
  }
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
  let out_path = PathBuf::from(build::out_dir!());
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
