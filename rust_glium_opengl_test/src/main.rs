#[macro_use]
extern crate glium;
extern crate rand;

use glium::Surface;
use glium::glutin;

mod support;

fn main() {
    const GLSL_COMPUTE: bool = true;
    const NUM_VALUES: usize = 16384;
    const NUM_GROUPS: usize = 128;
    const DT: f32 = 0.005;

    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_dimensions(1920, 1080)
        .with_gl_profile(glutin::GlProfile::Core)
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 3)))
        //.with_multisampling(4 as u16) // not works with compute shader
        .build_glium()
        .unwrap();

    
    // OPENGL GEOMETRY INIT

    // building the vertex and index buffers
    let vertex_buffer = support::load_wavefront(&display, include_bytes!("models/icosphere.obj"));
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // list of teapots with position and direction
    let mut teapots = (0 .. NUM_VALUES)
        .map(|i| {
            let dir = {if i < NUM_VALUES/2 as usize {-1.0} else {1.0}};
            
            let pos: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
            let pos = ( pos.0*0.1 + dir*0.5, 
                        pos.1*0.1 + dir*0.2, 
                        pos.2*0.1 + dir*0.4);
            
            let vel: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
            let vel = ( (vel.0 * 1.5 - 0.75)*4.0 + dir*0.1, 
                        (vel.1 * 1.5 - 0.75)*4.0 - dir*0.3, 
                        (vel.2 * 1.5 - 0.75)*4.0 );
            
            let acc: (f32, f32, f32) = (0.0, 0.0, 0.0);
            
            (pos, vel, acc)
        })
        .collect::<Vec<_>>();

    // building the vertex buffer with the attributes per instance
    let mut per_instance = {
        #[derive(Copy, Clone)]
        struct Attr {
            world_position: (f32, f32, f32),
        }

        implement_vertex!(Attr, world_position);

        let data = teapots.iter().map(|_| {
            Attr {
                world_position: (0.0, 0.0, 0.0),
            }
        }).collect::<Vec<_>>();

        glium::vertex::VertexBuffer::dynamic(&display, &data).unwrap()
    };
    
    // END OF OPENGL GEOMETRY INIT

    
    // VERTEX, FRAGMENT SHADER INIT

    let program_vs_fs = glium::Program::from_source(&display, // vertex, fragment
        "
            #version 140
            
            #define N 16384.0
            #define SCALE 0.001 

            uniform mat4 persp_matrix;
            uniform mat4 view_matrix;

            in vec3 position;
            in vec3 normal;
            in vec3 world_position;
            
            out vec3 v_position;
            out vec3 v_normal;
            out vec3 v_color;

            void main() {
                float dir;
                if (float(gl_InstanceID) < N/2.0) {dir = -1.0;} else {dir = 1.0;}
            
                v_position = position;
                v_normal = normal;            
                v_color = vec3(0.5*(1.0 - dir), 0.25, 0.5*(1.0 + dir));
                gl_Position = persp_matrix * view_matrix * vec4(position * SCALE + world_position, 1.0);
            }
        ",
        "
            #version 140

            in  vec3 v_normal;
            in  vec3 v_color;
            
            out vec4 f_color;

            const vec3 LIGHT = vec3(1.0, 1.0, 1.0);

            void main() {
                float ambient = 0.1;
                float diffuse = 1.9;
            
                float lum     = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                vec3  color   = (ambient + diffuse*lum)*v_color;
                f_color       = vec4(color, 1.0);
            }
        ",
        None)
        .unwrap();
        
    // END OF VERTEX, FRAGMENT SHADER INIT
    
    
    // COMPUTE SHADER INIT
    
    let program_cs = glium::program::ComputeShader::from_source(&display, r#"\
            #version 430
            
            #define N      16384
            #define LSIZEX 128
            
//            in uvec3 gl_NumWorkGroups;        // Check how many work groups there are. Provided for convenience.
//            in uvec3 gl_WorkGroupID;          // Check which work group the thread belongs to.
//            in uvec3 gl_LocalInvocationID;    // Within the work group, get a unique identifier for the thread.
//            in uvec3 gl_GlobalInvocationID;   // Globally unique value across the entire compute dispatch. 
//                                              // Short-hand for gl_WorkGroupID * gl_WorkGroupSize + gl_LocalInvocationID;
//            in uint  gl_LocalInvocationIndex; // 1D version of gl_LocalInvocationID. Provided for convenience.
            
            
            layout(local_size_x = LSIZEX, local_size_y = 1, local_size_z = 1) in;
            
//            layout(std430) buffer MyBlock {
//                float dt;
//            
//                float values_in_x[N];
//                float values_in_y[N];
//                float values_in_z[N];
//                
//                float values_mid[N]; 
//                
//                float values_out_x[N];
//                float values_out_y[N];
//                float values_out_z[N];
//            };
            
            layout(binding = 0) buffer BufferF32in  { vec4  values_in [N];  };
            layout(binding = 1) buffer BufferF32mid { float values_mid[N]; };
            layout(binding = 2) buffer BufferF32out { vec4  values_out[N]; };
            
            //shared vec3 shared_data[N];
            
            void main() {            
                uint ix        = gl_GlobalInvocationID.x; //gl_WorkGroupID.x * gl_WorkGroupSize.x + gl_LocalInvocationID.x;
                //uint maxix     = N-1; //gl_NumWorkGroups.x*gl_WorkGroupID.x;
                
                vec3 val_in    = values_in [ix].xyz;
                float val_mid  = values_mid[ix];
                vec3 val_out   = vec3(0.0, 0.0, 0.0);
                
                vec3  tx;     // this object pos
                float tm;     // this object mass
                vec3  ox;     // other object pos
                float om;     // other object mass   
                float d_thr;  // threshold distance (d is always >= d_thr)              
                vec3  dnm;    // distance vector
                float d;      // distance scalar
                vec3  dirv;   // direction vector
                float fg;     // gravity force scalar
                vec3  fgnm;   // temp gravity force vector
                
                d_thr = 0.01;
                tx    = val_in;
                tm    = val_mid;
                
                for (int i=0; i<N; i++) {
                    // it gets slow here because of non-optimal memory access
                    ox = values_in[i].xyz; // other object pos
                    om = values_mid[i];    // other object mass

                    d    = max(distance(tx, ox), d_thr); // distance scalar
                    dnm  = ox - tx;                      // distance vector
                    dirv = normalize(dnm);               // direction vector
                    
                    fg   = 0.0002*tm*om/(d*d);   // gravity force scalar
                    fgnm = fg*dirv;              // gravity force vector
                    
                    if (ix == i) 
                        fgnm *= 0.0; 
                    else 
                        fgnm *= 1.0; 
                        
                    barrier();  // avoiding dynamic branching
                    
                    val_out += fgnm;  // adding temp force to out force
                }
                                
                values_out[ix] = vec4(val_out.xyz, 0); // assigning computer force to output buffer
            }
        "#).unwrap();

//    struct Data {
//        dt: f32,
//    
//        values_in_x:  [f32;NUM_VALUES], 
//        values_in_y:  [f32;NUM_VALUES], 
//        values_in_z:  [f32;NUM_VALUES], 
//        
//        values_mid:   [f32;NUM_VALUES], 
//        
//        values_out_x: [f32;NUM_VALUES],
//        values_out_y: [f32;NUM_VALUES],
//        values_out_z: [f32],
//    }

//    implement_buffer_content!(Data);
//    implement_uniform_block!(Data, 
//                                dt,
//                                values_in_x, 
//                                values_in_y, 
//                                values_in_z, 
//                                values_mid, 
//                                values_out_x, 
//                                values_out_y, 
//                                values_out_z);
//    
//    let bytes_to_allocate: usize = (NUM_VALUES * (3 + 1 + 3)) * 4; // 3d pos, 1d masses, 3d forces 
//    let mut buffer: glium::uniforms::UniformBuffer<Data> =
//              glium::uniforms::UniformBuffer::empty_unsized(&display, bytes_to_allocate).unwrap();
              
    pub struct BufferF32in { values_in: [[f32;4]] }
    implement_buffer_content!(BufferF32in);
    implement_uniform_block!(BufferF32in, values_in);
    let mut buf_in: glium::uniforms::UniformBuffer<BufferF32in> = glium::uniforms::UniformBuffer::empty_unsized(&display, (NUM_VALUES * 4) * 4).unwrap();           
              
    pub struct BufferF32mid { values_mid: [f32] }
    implement_buffer_content!(BufferF32mid);
    implement_uniform_block!(BufferF32mid, values_mid);
    let mut buf_mid: glium::uniforms::UniformBuffer<BufferF32mid> = glium::uniforms::UniformBuffer::empty_unsized(&display, (NUM_VALUES * 4) * 1).unwrap();           
              
    pub struct BufferF32out { values_out: [[f32;4]] }
    implement_buffer_content!(BufferF32out);
    implement_uniform_block!(BufferF32out, values_out);
    let mut buf_out: glium::uniforms::UniformBuffer<BufferF32out> = glium::uniforms::UniformBuffer::empty_unsized(&display, (NUM_VALUES * 4) * 4).unwrap();







    // END OF COMPUTE SHADER INIT
    

    let mut camera = support::camera::CameraState::new();
    
    // the main loop
    support::start_loop(|| {
    
        if GLSL_COMPUTE { // update using shader
            { // filling buffer with points vector, masses vector
                //let mut mapcsbuf = buffer.map();
                let mut mapcsbufin = buf_in.map();
                let mut mapcsbufmid = buf_mid.map();
                
                for i in 0..NUM_VALUES {
                    let pos = teapots[i].0;
                
                    mapcsbufin.values_in[i][0] = pos.0;
                    mapcsbufin.values_in[i][1] = pos.1;
                    mapcsbufin.values_in[i][2] = pos.2;
                    
                    mapcsbufmid.values_mid[i] = 1.0; 
                }
            }

            program_cs.execute(uniform! { BufferF32in: &buf_in, BufferF32mid: &buf_mid, BufferF32out: &buf_out }, //, MyBlock: &buffer
                                (NUM_VALUES/NUM_GROUPS) as u32, 1,  1);

            { // reading forces vector from buffer, updating accels, velocities, positions
                //let mapcsbuf = buffer.map();
                
                let mapcsbufin  = buf_in.map();
                let mapcsbufmid = buf_mid.map();
                let mapcsbufout = buf_out.map();
                let mut mapping = per_instance.map();
                
                for i in 0..NUM_VALUES {
                    //let pos3d   = (mapcsbuf.values_in_x[i], mapcsbuf.values_in_y[i], mapcsbuf.values_in_z[i]);
                    let pos3d = (mapcsbufin.values_in[i][0], mapcsbufin.values_in[i][1], mapcsbufin.values_in[i][2]);
                    let mass    =  mapcsbufmid.values_mid[i];
                    let force3d = (mapcsbufout.values_out[i][0], mapcsbufout.values_out[i][1], mapcsbufout.values_out[i][2]);
                  
                    let mut teapot = &mut teapots[i];
                    let mut pos_to_write = &mut mapping[i];
                    
                    // a = f/m
                    (teapot.2).0 = force3d.0/mass;
                    (teapot.2).1 = force3d.1/mass;
                    (teapot.2).2 = force3d.2/mass;

                    // v = v + a*t
                    (teapot.1).0 += (teapot.2).0*DT;
                    (teapot.1).1 += (teapot.2).1*DT;
                    (teapot.1).2 += (teapot.2).2*DT;
                
                    // x = x + v*t
                    (teapot.0).0 += (teapot.1).0*DT;
                    (teapot.0).1 += (teapot.1).1*DT;
                    (teapot.0).2 += (teapot.1).2*DT;
                    
                    //println!("{:?} -> {:?} -> {:?} -> {:?}", pos3d, mass, force3d, teapot.2);
                    //println!("{:?} -> {:?}", i,  mapcsbuf.values_mid[i]);

                    pos_to_write.world_position = teapot.0;
                }
            } 
            // end of update using shader
        } else { 
            // updating using CPU (singlethreaded)
            let mut mapping = per_instance.map();
            let tpcopy = teapots.to_vec();
            
            for (src, dest) in teapots.iter_mut().zip(mapping.iter_mut()) {
                let mut fv: (f32, f32, f32) = (0.0, 0.0, 0.0); // force vector
                
                let tm = 1.0; // this mass
                let tx = src.0; // this position
                
                for other_src in &tpcopy { // m iteruje
                    let om = 1.0;       // other mass
                    let ox = other_src.0; // other position
                    
                    if ox.0 == tx.0 && ox.1 == tx.1 && ox.2 == tx.2 { continue; }
                    
                    let d_thr = 0.01;                
            
                    let dnm   = (ox.0 - tx.0, ox.1 - tx.1, ox.2 - tx.2);                  // distance vector
                    let mut d = (dnm.0*dnm.0 + dnm.1*dnm.1 + dnm.2*dnm.2).sqrt(); // distance scalar
                    if d < d_thr {d = d_thr;}
                    let dirv  = (dnm.0/d, dnm.1/d, dnm.2/d);                          // direction vector

                    let fg = 0.0001*tm*om/(d*d);     // gravity force scalar
                    let fgnm = (fg*dirv.0, fg*dirv.1, fg*dirv.2); // gravity force vector
                    
                    fv.0 += fgnm.0;
                    fv.1 += fgnm.1;
                    fv.2 += fgnm.2;
                }
                
                // a = f/m
                (src.2).0 = fv.0/tm;
                (src.2).1 = fv.1/tm;
                (src.2).2 = fv.2/tm;

                // v = v + a*t
                (src.1).0 += (src.2).0*DT;
                (src.1).1 += (src.2).1*DT;
                (src.1).2 += (src.2).2*DT;
            
                // x = x + v*t
                (src.0).0 += (src.1).0*DT;
                (src.0).1 += (src.1).1*DT;
                (src.0).2 += (src.1).2*DT;

                dest.world_position = src.0;
            }
            // end of updating using CPU
        } 
        
        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
        };

        // drawing a frame
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw((&vertex_buffer, per_instance.per_instance().unwrap()),
                    &indices, &program_vs_fs, &uniforms, &params).unwrap();
        target.finish().unwrap();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                ev => camera.process_input(&ev),
            }
        }

        support::Action::Continue
    });
}

