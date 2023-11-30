#include "backends/imgui_impl_glfw.h"

struct GLFWwindow;
struct GLFWmonitor;

extern "C"
auto imgui_glfw_init_for_opengl(GLFWwindow* window, bool install_callbacks) -> bool
{
  return ImGui_ImplGlfw_InitForOpenGL(window, install_callbacks);
}

extern "C"
auto imgui_glfw_init_for_vulkan(GLFWwindow* window, bool install_callbacks) -> bool
{
  return ImGui_ImplGlfw_InitForVulkan(window, install_callbacks);
}

extern "C"
auto imgui_glfw_init_for_other(GLFWwindow* window, bool install_callbacks) -> bool
{
  return ImGui_ImplGlfw_InitForOther(window, install_callbacks);
}

extern "C"
auto imgui_glfw_shutdown() -> void
{
  ImGui_ImplGlfw_Shutdown();
}

extern "C"
auto imgui_glfw_new_frame() -> void
{
  ImGui_ImplGlfw_NewFrame();
}

extern "C"
auto imgui_set_callbacks_chain_for_all_windows(bool chain_for_all_windows) -> void {
  ImGui_ImplGlfw_SetCallbacksChainForAllWindows(chain_for_all_windows);
}

extern "C"
auto imgui_window_focus_callback(GLFWwindow* window, int focused) -> void {
  ImGui_ImplGlfw_WindowFocusCallback(window, focused);
}

extern "C"
auto imgui_cursor_enter_callback(GLFWwindow* window, int entered) -> void {
  ImGui_ImplGlfw_CursorEnterCallback(window, entered);
}

extern "C"
auto imgui_cursor_pos_callback(GLFWwindow* window, double x, double y) -> void {
  ImGui_ImplGlfw_CursorPosCallback(window, x, y);
}

extern "C"
auto imgui_mouse_button_callback(GLFWwindow* window, int button, int action, int mods) -> void {
  ImGui_ImplGlfw_MouseButtonCallback(window, button, action, mods);
}

extern "C"
auto imgui_scroll_callback(GLFWwindow* window, double xoffset, double yoffset) -> void {
  ImGui_ImplGlfw_ScrollCallback(window, xoffset, yoffset);
}

extern "C"
auto imgui_key_callback(GLFWwindow* window, int key, int scancode, int action, int mods) -> void {
  ImGui_ImplGlfw_KeyCallback(window, key, scancode, action, mods);
}

extern "C"
auto imgui_char_callback(GLFWwindow* window, unsigned int c) -> void {
  ImGui_ImplGlfw_CharCallback(window, c);
}

extern "C"
auto imgui_monitor_callback(GLFWmonitor* monitor, int event) -> void {
  ImGui_ImplGlfw_MonitorCallback(monitor, event);
}
