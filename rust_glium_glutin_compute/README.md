## Compute Shader test

Initial code is based on [glium example](https://github.com/tomaka/glium/blob/master/examples/gpgpu.rs).

### getting rid of BackendCreationError(NotSupported)

There is an [error](https://raw.githubusercontent.com/michal2229/Rust-playground/master/rust_glium_glutin_compute/results/runlogerror.log) related to [issue 1310](https://github.com/tomaka/glium/issues/1310) because of linux graphics stack with Nvidia binary driver (and probably also optimus support on my laptop) - basically it can not be ran in headless mode. 

I had to build a window in order to make it run properly on Nvidia GPU. Intel GPU's drivers are on OpenGL 3.3 only (there are plans to support 4.3 in the future), so no compute shaders support on this GPU. 

With a window build fix it seems to run properly on Nvidia binary driver (with optirun). 

![screen0](https://raw.githubusercontent.com/michal2229/Rust-playground/master/rust_glium_glutin_compute/results/screen0.png)

### fixing memory layout

In the example one could notice, that not all numbers were calculated, but every fourh. 
It was because of used memory layout: *std140*, which [rounds up size of every element to the size of a vec4](https://www.opengl.org/wiki/Interface_Block_(GLSL))​ (16B).

![screen1](https://raw.githubusercontent.com/michal2229/Rust-playground/master/rust_glium_glutin_compute/results/screen1.png)

After changing layout from *std140* to *std430* things behave a lot better.

![screen2](https://raw.githubusercontent.com/michal2229/Rust-playground/master/rust_glium_glutin_compute/results/screen2.png)


