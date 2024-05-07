use astd::ffi::cstr;

extern "C" fn error_callback(error: i32, description: *const i8) {
  println!("GLFW error: {}: {}", error, unsafe {
    std::ffi::CStr::from_ptr(description).to_str().unwrap()
  });
}

fn main() {
  let mut logger: log::Logger = log::Logger::new("log");
  logger.sink(std::io::stderr());

  let glfw = glfw::Instance::new().expect("GLFW should be initialized");
  unsafe {
    let _ = glfw::c::glfwSetErrorCallback(Some(error_callback));
  }
  unsafe {
    glfw::c::glfwWindowHint(glfw::c::GLFW_CONTEXT_VERSION_MAJOR as _, 4);
    glfw::c::glfwWindowHint(glfw::c::GLFW_CONTEXT_VERSION_MINOR as _, 1);
    glfw::c::glfwWindowHint(glfw::c::GLFW_OPENGL_FORWARD_COMPAT as _, 1);
    glfw::c::glfwWindowHint(
      glfw::c::GLFW_OPENGL_PROFILE as _,
      glfw::c::GLFW_OPENGL_CORE_PROFILE as _,
    );
  }
  let raw_window = unsafe {
    glfw::c::glfwCreateWindow(
      640,
      480,
      cstr!("My first window").as_ptr(),
      std::ptr::null_mut(),
      std::ptr::null_mut(),
    )
  };
  let window = glfw::Window::from_c(raw_window);
  window.show();
  window.make_context_current();

  let _glew = glew::Instance::new().expect("GLEW should be initialized");
  let (major, minor) = gl::info::supported_version();
  if let Some(err) = gl::error::Error::last_error() {
    log::error!(logger, "OpenGL error: {}", err);
  }
  log::debug!(logger, "OpenGL supported version: {}.{}", major, minor);
  log::debug!(logger, "OpenGL version: {}", gl::info::version());
  log::debug!(logger, "OpenGL renderer: {}", gl::info::renderer());
  log::debug!(logger, "OpenGL vendor: {}", gl::info::vendor());
  log::debug!(
    logger,
    "OpenGL shading language version: {}",
    gl::info::shading_language_version()
  );
  log::debug!(logger, "OpenGL extensions:");
  for ext in gl::info::extensions() {
    log::debug!(logger, "- {}", ext);
  }

  let _device = openal::Device::open_default();
  if openal::has_extension(cstr!("ALC_ENUMERATION_EXT")) {
    log::debug!(logger, "OpenAL has enumeration extension!")
  } else {
    log::debug!(logger, "OpenAL does NOT have enumeration extension!")
  }

  // let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
  // let indices: [u32; 3] = [0, 1, 2];

  // unsafe {
  //   // let vao = glew::gl::vertex::generate_vertex_array();
  //   // glew::gl::vertex::bind_vertex_array(vao);

  //   // let vbo = glew::gl::vertex::generate_buffer();
  //   // glew::gl::vertex::bind_buffer(glew::c::GL_ARRAY_BUFFER, vbo);
  //   // glew::gl::vertex::buffer_data(
  //   //   glew::c::GL_ARRAY_BUFFER,
  //   //   (vertices.len() * std::mem::size_of::<f32>()) as _,
  //   //   vertices.as_ptr() as _,
  //   //   glew::c::GL_STATIC_DRAW,
  //   // )
  //   // glew::c::glBufferData(
  //   //   glew::c::GL_ARRAY_BUFFER,
  //   //   (vertices.len() * std::mem::size_of::<f32>()) as glew::c::GLsizeiptr,
  //   //   vertices.as_ptr() as *const glew::c::GLvoid,
  //   //   glew::c::GL_STATIC_DRAW,
  //   // );
  // }

  while !window.should_close() {
    gl::clear_color(1.0, 0.0, 0.0, 1.0);
    gl::clear(gl::ClearBits::COLOR);
    window.swap_buffers();
    glfw.poll_events();
  }
}
