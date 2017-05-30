## My  Rustlang playground

This is my repository containing code, which I emitted during learning Rust language. 

I like graphics and high performance computing, so topics 
like OpenGL, Vulkan, OpenCL etc. will probably constitute majority of this repo.

Previously I coded mostly in C/C++, Java and Python3 (NumPy and PyOpenCL for performance, micropython for embedded), so using Rust is quite a change for me. It helps me write better code in general - *rustc* compiler describes You what You are doing wrong, how to do it right etc., and You can apply this experience, the philosophy of this language, to other low level languages (like C/C++). 

As of 3D graphics, I was interested in OpenGL mostly, but Vulkan is very promising. I was able to build and run some (not all, some of them had non supported features) [Sascha Willems's Vulkan examples](https://github.com/SaschaWillems/Vulkan) on my Intel GPU, but it seems Intel Vulkan drivers are not yet complete enough (even using up-to-date oibaf/padoka PPAs). Another problem is the Nvidia Optimus support: NV binary drivers support Vulkan quite well, but not when one have also integrated Intel GPU in Optimus laptop. I think I should get some hardware with AMD Polaris/Vega GPU and forget about all these problems...

### examples

#### 2D particle simulation (multithreaded) using GPU accelerated SDL2 with WSAD+- navigation:

![Image1](https://raw.githubusercontent.com/michal2229/rust-playground/master/rust_sdl2_test/results/animated1.gif)

#### OpenGL 3D particle simulation using glium with WSADQE navigation; object is loaded from file, rendering is defined by vertex and fragment shaders, uses instancing, uses compute shaders to compute forces, computes max 16000 particles (in real time on fast GPU):

![screen4](https://raw.githubusercontent.com/michal2229/Rust-playground/master/rust_glium_opengl_test/results/screen4.png)

![screen6](https://raw.githubusercontent.com/michal2229/Rust-playground/master/rust_glium_opengl_test/results/screen6.png)
