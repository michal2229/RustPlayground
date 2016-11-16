# As of this moment Intel Ivy Bridge GPUs support max 3.3 version of OpenGL.
# But... they also support many features from latter 
# versions (only not all, to claim support for given OpenGL version). 
# OpenGL version reported by drivers can be overrided, 
# and if an application does not use 
# unimplemented features, chances are that it can run OK.
# Compute shaders are a feature partially 
# supported by Ivy Bridge driver. Most notably, there is no 
# FP64 support.

export MESA_GL_VERSION_OVERRIDE=4.5

export MESA_GLSL_VERSION_OVERRIDE=450

cargo build --release

cargo run   --release
