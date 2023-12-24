fn main() {
  let context = glfw::Context::new().expect("GLFW should be initialized");

  let raw_window = unsafe {
    glfw::c::glfwCreateWindow(
      640,
      480,
      foundation::cstr!("My first window").as_ptr(),
      std::ptr::null_mut(),
      std::ptr::null_mut(),
    )
  };
  let window = glfw::Window::from_c(raw_window);
  window.show();

  let mut logger: log::Logger = log::Logger::new("log");
  logger.sink(std::io::stderr());

  log::debug!(logger, "Debug message. Here's a number: {}", 42);
  log::info!(logger, "This is fine");
  log::warning!(logger, "Something happened!");
  log::error!(logger, "Testing error message. Here's a float: {}", 6.2);
  log::critical!(logger, "Testing critical message. Here's a dbg: {:?}", 6.2);

  if context.vulkan_supported() {
    log::debug!(logger, "Vulkan is supported!");
  } else {
    log::error!(logger, "Vulkan is not supported!");
    std::process::exit(1);
  }

  let count = vulkan::count_extension_properties();
  log::debug!(logger, "Vulkan extensions: {}", count);

  let _device = openal::Device::open_default();
  if openal::has_extension(foundation::cstr!("ALC_ENUMERATION_EXT")) {
    log::debug!(logger, "OpenAL has enumeration extension!")
  } else {
    log::debug!(logger, "OpenAL does NOT have enumeration extension!")
  }

  toast::ToastBuilder::question()
    .title_cstr(foundation::cstr!("Hello world"))
    .message_cstr(foundation::cstr!("something went wrong?"))
    .show();

  let _imgui = imgui::Context::new();
  unsafe { imgui::c::backend::glfw::imgui_glfw_init_for_vulkan(window.ptr_mut(), true) };

  while !window.should_close() {
    context.poll_events();
    window.swap_buffers();
  }
}
