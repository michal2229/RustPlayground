# Rust + Glium test

## about this repository

Playing with OpenGL in Rustlang.

An object is loaded from *\*.obj* file and displayed, while its position is being changed each frame. 

The code is based on [Glium examples](https://github.com/tomaka/glium/tree/master/examples). 

## building and running

Run in terminal:

```bash
cargo build --release
cargo run --release
```

## about [Glium](https://github.com/tomaka/glium)

Elegant and safe OpenGL wrapper.

Glium is an intermediate layer between OpenGL and your application. 
You still need to manually handle the graphics pipeline, 
but without having to use OpenGL's old and error-prone API.

Its objectives:

* Be safe to use. Many aspects of OpenGL that can trigger a crash if misused are automatically handled by glium.
* Provide an API that enforces good practices such as RAII or stateless function calls.
* Be compatible with all OpenGL versions that support shaders, providing a unified API when things diverge.
* Avoid all OpenGL errors beforehand.
* Produce optimized OpenGL function calls, and allow the user to easily use modern OpenGL techniques.