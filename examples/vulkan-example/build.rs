pub fn main() {
  if let Ok(sdk) = std::env::var("VULKAN_SDK") {
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}/lib", sdk);
  }
  println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/lib");
}
