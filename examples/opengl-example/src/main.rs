use alloy::math::mat::Matrix4;
use astd::ffi::cstr;

use toast as _;

extern "C" fn error_callback(error: i32, description: *const i8) {
  println!("GLFW error: {}: {}", error, unsafe {
    std::ffi::CStr::from_ptr(description).to_str().unwrap()
  });
}

const VERTEX_SHADER: &str = include_str!("../assets/shaders/triangle/shader.vert");
const FRAGMENT_SHADER: &str = include_str!("../assets/shaders/triangle/shader.frag");

fn log_opengl_preamble(logger: &mut log::Logger) {
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
}

fn log_openal_preamble(logger: &mut log::Logger) {
  if openal::has_extension(cstr!("ALC_ENUMERATION_EXT")) {
    log::debug!(logger, "OpenAL has enumeration extension!")
  } else {
    log::debug!(logger, "OpenAL does NOT have enumeration extension!")
  }
}

fn main() {
  let mut logger: log::Logger = log::Logger::new("log");
  logger.sink(std::io::stderr());

  log::debug!(logger, "vertex: {}", VERTEX_SHADER);
  log::debug!(logger, "fragment: {}", FRAGMENT_SHADER);

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
  // tar::read_tar(&mut std::fs::File::open("out.tar").unwrap()).unwrap();
  let window = glfw::Window::from_c(raw_window);
  window.show();
  window.make_context_current();

  let _model =
    alloy::model::obj::Model::from_file("examples/opengl-example/assets/teapot.obj").unwrap();

  let _glew = glew::Instance::new().expect("GLEW should be initialized");
  let _device = openal::Device::open_default();

  log_opengl_preamble(&mut logger);
  log_openal_preamble(&mut logger);
  let _context = imgui::Context::new();
  imgui::io::set_ini_filename_cstr(cstr!("alloy.ini"));
  imgui::io::set_io_config(
    imgui::io::ConfigFlags::default()
      .allow_gamepad_navigation(true)
      .allow_keyboard_navigation(true)
      .allow_mouse_navigation(true)
      .allow_docking(true),
  );
  unsafe {
    imgui_sys::backend::opengl::imgui_opengl3_init(cstr!("#version 410").as_ptr());
    imgui_sys::backend::glfw::imgui_glfw_init_for_opengl(window.ptr_mut(), true);
  }

  const VERTICES: [f32; 9] = [
    -1.0, -1.0, 0.0, //
    1.0, -1.0, 0.0, //
    0.0, 1.0, 0.0, //
  ];
  const INDICES: [u32; 3] = [
    0, 1, 2, // first Triangle
  ];

  // These are output variables from the OpenGL C-API -- and they need to be
  // initialized so that Rust doesn't complain about their use before they are
  // initialized.
  #[allow(unused_assignments)]
  let mut m_vao: gl::c::GLuint = 0;
  #[allow(unused_assignments)]
  let mut m_vbo: gl::c::GLuint = 0;
  #[allow(unused_assignments)]
  let mut m_ebo: gl::c::GLuint = 0;

  let mut vertex_shader = gl::shader::Shader::new(gl::shader::ShaderKind::Vertex);
  vertex_shader.shader_source(VERTEX_SHADER);
  vertex_shader.compile().unwrap();

  let mut fragment_shader = gl::shader::Shader::new(gl::shader::ShaderKind::Fragment);
  fragment_shader.shader_source(FRAGMENT_SHADER);
  fragment_shader.compile().unwrap();

  let mut shader_program = gl::shader::Program::new();
  shader_program.attach_shader(&vertex_shader);
  shader_program.attach_shader(&fragment_shader);
  shader_program.link().unwrap();

  unsafe {
    gl::c::glGenVertexArrays(1, &mut m_vao as _);

    let mut buffers: [gl::c::GLuint; 2] = [0, 0];

    gl::c::glGenBuffers(2, buffers.as_mut_ptr());
    m_vbo = buffers[0];
    m_ebo = buffers[1];

    gl::c::glBindVertexArray(m_vao);
    gl::c::glBindBuffer(gl::c::GL_ARRAY_BUFFER, m_vbo);
    gl::c::glBufferData(
      gl::c::GL_ARRAY_BUFFER,
      (VERTICES.len() * 4) as _,
      VERTICES.as_ptr() as _,
      gl::c::GL_STATIC_DRAW,
    );
    gl::c::glVertexAttribPointer(0, 3, glew::c::GL_FLOAT, false, 0, std::ptr::null());
    gl::c::glEnableVertexAttribArray(0);

    gl::c::glBindBuffer(gl::c::GL_ELEMENT_ARRAY_BUFFER, m_ebo);
    gl::c::glBufferData(
      gl::c::GL_ELEMENT_ARRAY_BUFFER,
      (INDICES.len() * 4) as _,
      INDICES.as_ptr() as _,
      gl::c::GL_STATIC_DRAW,
    );
  }

  log::debug!(logger, "m_vao: {}", m_vao);
  log::debug!(logger, "m_vbo: {}", m_vbo);
  log::debug!(logger, "m_ebo: {}", m_ebo);

  use alloy::model::clip;
  let projection = alloy::model::projection::Projection::orthographic(clip::ClipSpace {
    depth: clip::Depth::new(-2.0, 2.0),
    horizontal: clip::Horizontal::uniform(2.0),
    vertical: clip::Vertical::uniform(2.0),
  });

  let projection_matrix = projection.to_matrix4();
  let view_matrix = Matrix4::identity();

  let mut transform = alloy::model::transform::Transform::default();
  while !window.should_close() {
    gl::clear(gl::ClearBits::none().color().depth());
    gl::clear_color(0.0, 0.0, 0.0, 1.0);

    shader_program.bind();
    let transform_loc = shader_program.uniform_location("transform");
    let projection_loc = shader_program.uniform_location("projection");
    let view_loc = shader_program.uniform_location("view");
    unsafe {
      shader_program.set_uniform_matrix4_unchecked(
        transform_loc,
        1,
        transform.transform().as_ptr(),
      );
      shader_program.set_uniform_matrix4_unchecked(projection_loc, 1, projection_matrix.as_ptr());
      shader_program.set_uniform_matrix4_unchecked(view_loc, 1, view_matrix.as_ptr());
    }
    transform.rotate_pitch(alloy::math::Degree::new(1.0));

    unsafe {
      gl::c::glBindBuffer(gl::c::GL_ARRAY_BUFFER, m_vbo);
      gl::c::glDrawArrays(glew::c::GL_TRIANGLES, 0, (VERTICES.len() / 3) as _);
    }

    // let fps = 1.0 / delta.as_secs_f32();
    unsafe {
      imgui_sys::backend::opengl::imgui_opengl3_new_frame();
      imgui_sys::backend::glfw::imgui_glfw_new_frame();
      imgui::new_frame();

      imgui_sys::igShowDemoWindow(&mut true as *mut _);
      imgui_sys::igBegin(cstr!("Hello, world!").as_ptr(), std::ptr::null_mut(), 0);
      imgui::text!("This is some useful text.");
      // imgui::text!("FPS {:6.2}", fps);
      imgui_sys::igEnd();
      imgui_sys::igRender();
      imgui_sys::igUpdatePlatformWindows();
      imgui_sys::backend::opengl::imgui_opengl3_render_draw_data(imgui_sys::igGetDrawData());
    }

    window.swap_buffers();
    glfw.poll_events();
  }
}
