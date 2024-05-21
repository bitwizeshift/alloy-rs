#include "backends/imgui_impl_opengl3.h"

extern "C"
auto imgui_opengl3_init(const char* glsl_version) -> bool
{
  return ImGui_ImplOpenGL3_Init(glsl_version);
}

extern "C"
auto imgui_opengl3_shutdown() -> void
{
  ImGui_ImplOpenGL3_Shutdown();
}

extern "C"
auto imgui_opengl3_new_frame() -> void
{
  ImGui_ImplOpenGL3_NewFrame();
}

extern "C"
auto imgui_opengl3_render_draw_data(ImDrawData* draw_data) -> void
{
  ImGui_ImplOpenGL3_RenderDrawData(draw_data);
}

extern "C"
auto imgui_opengl3_create_fonts_texture() -> bool
{
  return ImGui_ImplOpenGL3_CreateFontsTexture();
}

extern "C"
auto imgui_opengl3_destroy_fonts_texture() -> void
{
  ImGui_ImplOpenGL3_DestroyFontsTexture();
}

extern "C"
auto imgui_opengl3_create_device_objects() -> bool
{
  return ImGui_ImplOpenGL3_CreateDeviceObjects();
}

extern "C"
auto imgui_opengl3_destroy_device_objects() -> void
{
  ImGui_ImplOpenGL3_DestroyDeviceObjects();
}
