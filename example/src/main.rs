fn main() {
  let context = glfw::Context::new().expect("GLFW should be initialized");

  let raw_window = unsafe {
    glfw::c::glfwCreateWindow(
      640,
      480,
      "My first window\0".as_bytes().as_ptr() as *const i8,
      std::ptr::null_mut(),
      std::ptr::null_mut(),
    )
  };
  let window = glfw::Window::from_c(raw_window);
  window.show();

  if context.vulkan_supported() {
    println!("Vulkan is supported!")
  } else {
    println!("Vulkan is not supported!")
  }

  let count = vulkan::count_extension_properties();
  println!("Vulkan extensions: {}", count);

  use core::ffi::CStr;
  let _device = openal::Device::open_default();
  if openal::has_extension(unsafe {
    CStr::from_ptr("ALC_ENUMERATION_EXT\0".as_bytes().as_ptr() as *mut i8)
  }) {
    println!("OpenAL has enumeration extension!")
  } else {
    println!("OpenAL does NOT have enumeration extension!")
  }

  while !window.should_close() {
    context.poll_events();
    window.swap_buffers();
  }
}
