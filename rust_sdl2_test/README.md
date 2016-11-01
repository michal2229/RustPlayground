## description

Simple toy project to learn basics of Rust + SDL2.

It presents a window, in which particles interact with each other with gravity and charge forces. Every particle intracts with all others, so  the complexity is a cube of particle number (smooth up to ~512 p.). It spawns many threads (as many particles there is), in each thread a force for a particle is computed. Resulting forces from threads are collected to vector. This vector is used to compute accelerations, velocities, and positions of particles (singlethreaded).

## what is Rust? 

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

Featuring

* zero-cost abstractions
* move semantics
* guaranteed memory safety
* threads without data races
* trait-based generics
* pattern matching
* type inference
* minimal runtime
* efficient C bindings

*(info from rust-lang.org)*

## why rust?

Because

* it is almost as fast as C/C++
* it is far more thread and memory safe than C/C++
* it seems easier than C/C++
* it does not use garbage collector
* it makes You think and learn how to do things right
* it has nice compiler with useful hints
* easy library management (crates) with cargo

## what is SDL2?

Simple DirectMedia Layer is a cross-platform development library designed to provide low level access to audio, keyboard, mouse, joystick, and graphics hardware via OpenGL and Direct3D. It is used by video playback software, emulators, and popular games including Valve's award winning catalog and many Humble Bundle games.

SDL officially supports Windows, Mac OS X, Linux, iOS, and Android. Support for other platforms may be found in the source code.

SDL is written in C, works natively with C++, and there are bindings available for several other languages, including C# and Python.

SDL 2.0 is distributed under the zlib license. This license allows you to use SDL freely in any software.

*(info from libsdl.org)*

## why SDL2?

Because

* it has GPU acceleration
* 2D and 3D graphics
* OpenGL, D3D, Vulkan (which I am about to try out)
* cross platform

## how to run

Debug version:

```bash
cargo build && cargo run
```

...or for more optimized version:


```bash
cargo build --release && cargo run
```

## sample images

![Image1](https://raw.githubusercontent.com/michal2229/rust-playground/master/rust_sdl2_test/results/animated1.gif)

![Image2](https://raw.githubusercontent.com/michal2229/rust-playground/master/rust_sdl2_test/results/animation.gif)

![Image3](https://raw.githubusercontent.com/michal2229/rust-playground/master/rust_sdl2_test/results/screen%202016-10-20%2000-53-16.png)

![Image4](https://raw.githubusercontent.com/michal2229/rust-playground/master/rust_sdl2_test/results/screen%202016-10-20%2000-53-20.png)

![Image5](https://raw.githubusercontent.com/michal2229/rust-playground/master/rust_sdl2_test/results/screen%202016-10-20%2000-53-30.png)

![Image6](https://raw.githubusercontent.com/michal2229/rust-playground/master/rust_sdl2_test/results/screen%202016-10-20%2000-53-50.png)

![Image7](https://raw.githubusercontent.com/michal2229/rust-playground/master/rust_sdl2_test/results/screen%202016-10-20%2000-54-01.png)

