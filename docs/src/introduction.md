# Introduction

**Alloy** is an experimental game engine written in Rust which aims to recreate
logic previously authored in my [C++ engine of the same name][cpp_alloy].

In particular, this project exists to explore working with Graphics, Audio,
and all of the fun linear algebra required for rendering.

## Project Goals

The goals of this project are:

* **Abstracting the underlying rendering system**: The user should not have to
  care if they are using OpenGL, Vulkan, or DirectX; but this should also be
  selectable on-the-fly. Shaders should be pre-compiled where possible, and
  internalized as high-level `Material` objects.

* **Abstracting the underlying audio system**: Same as above; the user should
  not have to care if they are using OpenAL or FMOD; but this should also be
  selectable on-the-fly.

* **Support for both 2 and 3 rendering**: The ultimate goal is actually to do
  2.5-D renderings, where the world is "3D", but appears as 2D pixel-art.
  However, the engine should adapt to both.

* **Model scene graphs**: Models should be able to be composed of multiple
  bones, which can be rendered via scene-graphs and handle bone logic.

[cpp_alloy]: https://github.com/bitwizeshift/alloy
