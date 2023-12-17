use std::env;
use std::path::PathBuf;

fn main() {
  build::rerun_if_env_changed!("CC");
  build::rerun_if_env_changed!("CXX");
  build::rerun_if_changed!("../../3rd-party/openal-soft");
  build::rerun_if_changed!("../../.gitmodules");

  compile_openal();
  generate_openal_bindings();
}

// Compiles openal by calling out to CMake.
fn compile_openal() {
  let mut cfg = cmake::Config::new("../../3rd-party/openal-soft");

  if cfg!(target_os = "windows") {
    cfg
      .define("LSOFT_BUILD_ROUTER", "ON")
      .define("LSOFT_REQUIRE_WINMM", "ON")
      .define("LSOFT_REQUIRE_DSOUND", "ON")
      .define("LSOFT_REQUIRE_WASAPI", "ON")
      .define("LSOFT_EMBED_HRTF_DATA", "YES");
  } else if cfg!(target_os = "linux") {
    cfg
      .define("ALSOFT_REQUIRE_ALSA", "ON")
      .define("ALSOFT_REQUIRE_OSS", "ON")
      .define("ALSOFT_REQUIRE_PORTAUDIO", "ON")
      .define("ALSOFT_REQUIRE_PULSEAUDIO", "ON")
      .define("ALSOFT_REQUIRE_JACK", "ON")
      .define("ALSOFT_EMBED_HRTF_DATA", "YES");
  } else if cfg!(target_os = "macos") {
    cfg
      .define("ALSOFT_REQUIRE_COREAUDIO", "ON")
      .define("DALSOFT_EMBED_HRTF_DATA", "ON");
  } else if cfg!(target_os = "ios") {
    cfg
      .define("CMAKE_OSX_ARCHITECTURES", "armv7;arm64")
      .define("ALSOFT_REQUIRE_COREAUDIO", "ON")
      .define("DALSOFT_EMBED_HRTF_DATA", "ON");
  }

  let dst = cfg.define("LIBTYPE", "STATIC").build();

  build::rustc_link_search!("native={}", dst.join("lib").display());
  build::rustc_link_lib!("static=openal");
  if cfg!(any(target_os = "macos", target_os = "ios")) {
    build::rustc_link_lib!("c++");
    build::rustc_link_lib!("framework=AudioToolbox");
    build::rustc_link_lib!("framework=CoreAudio");
    build::rustc_link_lib!("framework=CoreFoundation");
  } else if cfg!(target_os = "linux") {
    build::rustc_link_lib!("stdc++");
  }
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
