## where am I?

This is my repository containing code, which I emitted during learning Rust language. 

I like graphics and high performance computing, so topics 
like OpenGL, Vulkan, OpenCL etc. will probably constitute majority of this repo.

Previously I coded mostly in Python3 (NumPy and PyOpenCL for performance, micropython for embedded) and C, so using Rust is quite a change for me. 
I think I am starting to understand the spirit of this language... It also helps me write better C/C++ code, because *rustc* compiler tells You what You are doing wrong, how to do it right etc., and You can cast this experience to other languages. 

### examples

#### 2D particle simulation (multithreaded) using GPU accelerated SDL2 with WSAD+- navigation:

![Image1](https://raw.githubusercontent.com/michal2229/rust-playground/master/rust_sdl2_test/results/animated1.gif)

#### OpenGL 3D particle simulation using glium with WSADQE navigation; object is loaded from file, rendering is defined by vertex and fragment shaders, uses instancing, uses compute shaders to compute forces:

![screen3](https://raw.githubusercontent.com/michal2229/Rust-playground/master/rust_glium_opengl_test/results/screen3.png)
