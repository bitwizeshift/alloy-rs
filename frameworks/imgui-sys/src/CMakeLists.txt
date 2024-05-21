cmake_minimum_required(VERSION 3.24)
project(imgui LANGUAGES CXX C)

option(VULKAN_BACKEND "Sets the vulkan backend" OFF)
option(OPENGL_BACKEND "Sets the OpenGL backend" OFF)
option(DX12_BACKEND "Sets the DX12 backend" OFF)

set(external_root "../../../3rd-party")
cmake_path(ABSOLUTE_PATH external_root)
set(cimgui_root "${external_root}/cimgui")

set(source_files
  "${cimgui_root}/cimgui.cpp"
  "${cimgui_root}/imgui/imgui.h"
  "${cimgui_root}/imgui/imgui.cpp"
  "${cimgui_root}/imgui/imgui_draw.cpp"
  "${cimgui_root}/imgui/imgui_demo.cpp"
  "${cimgui_root}/imgui/imgui_widgets.cpp"
  "${cimgui_root}/imgui/imgui_tables.cpp"
  "${cimgui_root}/imgui/backends/imgui_impl_glfw.h"
  "${cimgui_root}/imgui/backends/imgui_impl_glfw.cpp"
  "${CMAKE_CURRENT_LIST_DIR}/imgui_glfw.cpp"
)

if (VULKAN_BACKEND)
  list(APPEND source_files
    "${cimgui_root}/imgui/backends/imgui_impl_vulkan.cpp"
    "${cimgui_root}/imgui/backends/imgui_impl_vulkan.h"
    "${CMAKE_CURRENT_LIST_DIR}/imgui_vulkan.cpp"
  )
endif ()

if (OPENGL_BACKEND)
  list(APPEND source_files
    "${cimgui_root}/imgui/backends/imgui_impl_opengl3.cpp"
    "${cimgui_root}/imgui/backends/imgui_impl_opengl3.h"
    "${CMAKE_CURRENT_LIST_DIR}/imgui_opengl3.cpp"
  )
endif ()

if (DX12_BACKEND)
  message(FATAL_ERROR "DX12 does not yet have C bindings")
  list(APPEND source_files
    "${cimgui_root}/imgui/backends/imgui_impl_dx12.cpp"
    "${cimgui_root}/imgui/backends/imgui_impl_dx12.h"
  )
endif ()

add_library(imgui STATIC ${source_files})

target_compile_definitions(imgui PUBLIC IMGUI_DISABLE_OBSOLETE_FUNCTIONS=1)
# target_compile_definitions(imgui PUBLIC CIMGUI_DEFINE_ENUMS_AND_STRUCTS=0)
if (WIN32)
  target_compile_definitions(imgui PUBLIC IMGUI_IMPL_API=extern\t\"C\"\t__declspec\(dllexport\))
  target_link_libraries(imgui imm32)
else ()
  target_compile_definitions(imgui PUBLIC IMGUI_IMPL_API=extern\t\"C\"\t)
endif ()

target_include_directories(imgui PUBLIC "${CMAKE_CURRENT_LIST_DIR}")
target_include_directories(imgui PUBLIC "${cimgui_root}")
target_include_directories(imgui PUBLIC "${cimgui_root}/imgui")
target_include_directories(imgui PUBLIC "${external_root}/glfw/include")
if (VULKAN_BACKEND)
  find_package(Vulkan REQUIRED)
  target_link_libraries(imgui PUBLIC Vulkan::Vulkan)
  target_include_directories(imgui PUBLIC "${external_root}/vulkan-headers/include")
endif ()
# if (OPENGL_BACKEND)
#   find_package(OpenGL REQUIRED)
#   target_link_libraries(imgui PUBLIC OpenGL::OpenGL)
# endif ()

# set_target_properties(imgui PROPERTIES PREFIX "")

# install
include(GNUInstallDirs)
install(TARGETS imgui
  ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
