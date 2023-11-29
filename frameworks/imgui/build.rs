//! Build script to compile GLFW from source, and generate a valid bindgen-C API
//! from imgui.
use std::env;

fn main() {
  compile_imgui();
  generate_imgui_bindings();
}

// Compiles GLFW by calling out to CMake.
fn compile_imgui() {
  let mut build = cc::Build::new();

  build
    .cpp(true)
    .include("../../3rd-party/imgui/")
    .include("../../3rd-party/imgui/backends")
    .include("../../3rd-party/glfw/include")
    .file("../../3rd-party/imgui/imgui.cpp")
    .file("../../3rd-party/imgui/imgui_widgets.cpp")
    .file("../../3rd-party/imgui/imgui_tables.cpp")
    .file("../../3rd-party/imgui/imgui_draw.cpp")
    .file("../../3rd-party/imgui/imgui_demo.cpp")
    .file("../../3rd-party/imgui/backends/imgui_impl_glfw.cpp");

  if cfg!(feature = "vulkan") {
    build.file("../../3rd-party/imgui/backends/imgui_impl_vulkan.cpp");
  }
  if cfg!(feature = "opengl") {
    build.file("../../3rd-party/imgui/backends/imgui_impl_opengl3.cpp");
  }

  build.compile("imgui-static");

  println!(
    "cargo:rustc-link-search=native={}",
    env::var("OUT_DIR").unwrap()
  );
  println!("cargo:rustc-link-lib=static=imgui-static");
}

fn generate_imgui_bindings() {
  use std::path::PathBuf;

  // build::rerun_if_changed!("../../3rd-party/imgui/imstb_truetype.h");
  // build::rerun_if_changed!("../../3rd-party/imgui/imstb_textedit.h");
  // build::rerun_if_changed!("../../3rd-party/imgui/imstb_rectpack.h");
  // build::rerun_if_changed!("../../3rd-party/imgui/imgui_internal.h");
  build::rerun_if_changed!("../../3rd-party/imgui/imgui.h");

  let mut builder = bindgen::Builder::default()
    // .header("../../3rd-party/imgui/imstb_truetype.h")
    // .header("../../3rd-party/imgui/imstb_textedit.h")
    // .header("../../3rd-party/imgui/imstb_rectpack.h")
    // .header("../../3rd-party/imgui/imgui_internal.h")
    .header("../../3rd-party/imgui/imgui.h")
    .clang_args(&["-I", "../../3rd-party/imgui"])
    .clang_args(&["-I", "../../3rd-party/imgui/backends"])
    .enable_cxx_namespaces()
    .generate_cstr(true);

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
