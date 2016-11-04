#[macro_use]
extern crate glium;
extern crate rand;

use glium::Surface;
use glium::glutin;

mod support;

fn main() {
    let dt: f32 = 0.01;

    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    // building the vertex and index buffers
    let vertex_buffer = support::load_wavefront(&display, include_bytes!("models/icosphere.obj"));
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // list of teapots with position and direction
    let mut teapots = (0 .. 1000)
        .map(|i| {
            let dir = {if i < 500 {-1.0} else {1.0}};
        
        
            let pos: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
            let pos = (pos.0/10.0, pos.1/10.0 + dir*0.5, pos.2 * 1.5 - 0.75);
            
            let vel: (f32, f32, f32) = (0.0, rand::random(), 0.0);
            let vel = (dir*0.1, (vel.1 * 1.5 - 0.75)/10.0, 0.0);
            
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

    let program = glium::Program::from_source(&display,
        "
            #version 140

            in vec3 position;
            in vec3 normal;
            in vec3 world_position;
            out vec3 v_position;
            out vec3 v_normal;
            out vec3 v_color;

            void main() {
                v_position = position;
                v_normal = normal;
                v_color = vec3(float(gl_InstanceID) / 1000.0, 0.5, 1.0 - float(gl_InstanceID) / 1000.0);
                gl_Position = vec4(position * 0.01 + world_position, 1.0);
            }
        ",
        "
            #version 140

            in vec3 v_normal;
            in vec3 v_color;
            out vec4 f_color;

            const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

            void main() {
                float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                vec3 color = (0.3 + 0.7 * lum) * v_color;
                f_color = vec4(color, 1.0);
            }
        ",
        None)
        .unwrap();

    let camera = support::camera::CameraState::new();
    
    // the main loop
    support::start_loop(|| {
        // updating the teapots
        {
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
                    
                    let dthr = 0.1;                
            
                    let dnm  = (ox.0 - tx.0, ox.1 - tx.1, ox.2 - tx.2);                  // distance vector
                    let mut d    = (dnm.0*dnm.0 + dnm.1*dnm.1 + dnm.2*dnm.2).sqrt(); // distance scalar
                    if d < dthr {d = dthr;}
                    let dirv = (dnm.0/d, dnm.1/d, dnm.2/d);                          // direction vector
                    

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
                (src.1).0 += (src.2).0*dt;
                (src.1).1 += (src.2).1*dt;
                (src.1).2 += (src.2).2*dt;
            
                // x = x + v*t
                (src.0).0 += (src.1).0*dt;
                (src.0).1 += (src.1).1*dt;
                (src.0).2 += (src.1).2*dt;

                dest.world_position = src.0;
            }
        }

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
                    &indices, &program, &uniform! { matrix: camera.get_perspective() },
                    &params).unwrap();
        target.finish().unwrap();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                _ => ()
            }
        }

        support::Action::Continue
    });
}

