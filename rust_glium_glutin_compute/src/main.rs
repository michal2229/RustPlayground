#[macro_use]
extern crate glium;
extern crate rand;
use glium::glutin;

fn main() {
    use glium::DisplayBuild;

    // building new display
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 3)))
        .build_glium()
        .unwrap();

    // NOTE from https://www.opengl.org/wiki/Interface_Block_(GLSL) about std140 layout: 
    // "The array stride (the bytes between array elements) is 
    // always rounded up to the size of a vec4​ (ie: 16-bytes). 
    // So arrays will only match their C/C++ definitions if the type 
    // is a multiple of 16 bytes" 
    // std140 layout was used in this example and memory usage was 4x higher than needed, 
    // I changed it to std430 which do not waste memory (rounding up to 4 bytes = float is 4 bytes)
    let program = glium::program::ComputeShader::from_source(&display, r#"\
            #version 430
            layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
            layout(std430) buffer MyBlock {
                float power;
                float values[];
            };
            void main() {
                float val = values[gl_GlobalInvocationID.x];
                values[gl_GlobalInvocationID.x] = pow(val, power);
            }
        "#).unwrap();

    struct Data {
        power: f32,
        //_padding: [f32; 3] // not needed anymore, since std430 layout is used instead of std140
        values: [f32],
    }

    implement_buffer_content!(Data);
    implement_uniform_block!(Data, power, values);

    const NUM_VALUES: usize = 32;

    let mut buffer: glium::uniforms::UniformBuffer<Data> =
              glium::uniforms::UniformBuffer::empty_unsized(&display, 4 + 4 * NUM_VALUES).unwrap();

    {
        let mut mapping = buffer.map();
        mapping.power = 1.01;
        for (i, val) in mapping.values.iter_mut().enumerate() {
            //*val = rand::random();
            *val = i as f32;
        }
    }

    program.execute(uniform! { MyBlock: &*buffer }, NUM_VALUES as u32, 1, 1); // Nx1x1 space

    {
        let mapping = buffer.map();
        println!("Power is: {:?}", mapping.power);
        
        //for val in mapping.values.iter().take(25) { // I want to see them all!
        for val in mapping.values.iter() {
            println!("{:?}", *val);
        }
    }
}
