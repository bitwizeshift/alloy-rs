use std::env;
use std::path::PathBuf;

fn main() {
  link_vulkan();
  generate_vulkan_bindings();
}

fn link_vulkan() {
  if let Some(search_path) = option_env!("VULKAN_SDK") {
    build::rustc_link_search!("native={}/lib", search_path);
  }
  build::rustc_link_lib!("dylib=vulkan");
}

fn generate_vulkan_bindings() {
  build::rerun_if_changed!("../../3rd-party/vulkan-headers/include/vulkan/vulkan.h");

  let builder =
    bindgen::Builder::default().header("../../3rd-party/vulkan-headers/include/vulkan/vulkan.h");

  let bindings = target_bindgen_args(builder)
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .clang_args(&["-I", "../../3rd-party/vulkan-headers/include"])
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
