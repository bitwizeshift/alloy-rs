use std::path::{Path, PathBuf};

fn main() {
  build::rerun_if_changed!("../../3rd-party/vulkan-headers");
  build::rerun_if_changed!("../../.gitmodules");

  link_vulkan();
  generate_vulkan_bindings();
}

fn link_vulkan() {
  if let Some(search_path) = option_env!("VULKAN_SDK") {
    build::rustc_link_search!("native={}/lib", search_path);

    if cfg!(any(target_os = "macos", target_os = "ios")) {
      let out = build::out_dir!();
      let _ = std::fs::create_dir(format!("{}/lib", out));
      build::rustc_link_search!("native={}/lib", build::out_dir!());
      copy_file(search_path, &out, "libvulkan.dylib");
      copy_file(search_path, &out, "libvulkan.1.dylib");
    }
  }
  build::rustc_link_lib!("dylib=vulkan");
}

fn copy_file<P0, P1, P2>(search_path: P0, out: P1, file: P2)
where
  P0: AsRef<Path> + std::fmt::Display,
  P1: AsRef<Path> + std::fmt::Display,
  P2: AsRef<Path> + std::fmt::Display,
{
  std::fs::copy(
    format!("{}/lib/{}", search_path, file),
    format!("{}/lib/{}", out, file),
  )
  .unwrap_or_else(|_| panic!("Unable to copy {} to {}", file, out));
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
  let out_path = PathBuf::from(build::out_dir!());
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
