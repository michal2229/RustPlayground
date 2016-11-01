/// Author: Michal Bokiniec
///
/// Simple toy project to learn basics of Rust + SDL2.
/// It presents a window, in which particles interact with each other
/// with gravity and charge forces. Every particle intracts with all others, so 
/// the complexity is a cube of particle number (smooth up to ~512 p.). 
/// It spawns many threads (as many particles there is), 
/// in each thread a force for a particle is computed. 
/// Resulting forces from threads are collected to vector.
/// This vector is used to compute accelerations, velocities, 
/// and positions of particles (singlethreaded).
///
/// cargo build && cargo run


extern crate sdl2;
extern crate rand;


use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
//use std::ops::Rem;
//use std::num;
//use std::sync::mpsc;
use std::thread;
//use std::time;
use std::sync::Arc;
//use std::sync::Mutex;
//use sdl2::video::GLProfile;
use rand::Rng;
use sdl2::event::Event;
//use std::cmp;


#[derive(Copy, Clone)]
struct Node {
    m:  f32, // mass
    c:  f32, // charge per mass unit
    px: f32, py: f32, // position
    vx: f32, vy: f32, // velocity
    ax: f32, ay: f32, // acceleration
    fx: f32, fy: f32  // force
}


fn emit_node(v: &mut Vec<Node>, x: f32, y:f32, vx:f32, vy:f32, m: f32, c: f32) {
    let node = Node {m: m, c: c, px: x, py: y, vx: vx, vy: vy, ax: 0.0, ay: 0.0, fx: 0.0, fy: 0.0, };
    v.push(node);
}


fn init_nodes_vec(v: &mut Vec<Node>, n: u32) {
    let sqrn2 = (n as f32/2.0).sqrt() as f32;
    //let thresholdn = n/2;
    let centery = 240.0;
    let centerx = 320.0;
    let radius =  200.0;
    
    // init random number generator
    let mut rng = rand::thread_rng();
    let sp = 1.0;
    
    for i in 0..n/2 {
        let x: f32 = ((i as f32 % sqrn2) + rng.gen::<f32>())*sp;
        let y: f32 = ((i as f32 / sqrn2) + rng.gen::<f32>())*sp;
        
        let node = Node {m: 10.0, c: 5.0, px: centerx - x, py: centery - y - radius , vx: -9.0 + rng.gen::<f32>()/4.0, vy: 3.0, ax: 0.0, ay: 0.0, fx: 0.0, fy: 0.0, };
        v.push(node);
    }
    
    for i in 0..n/2 {
        let x: f32 = ((i as f32 % sqrn2) + rng.gen::<f32>())*sp;
        let y: f32 = ((i as f32 / sqrn2) + rng.gen::<f32>())*sp;

        let node = Node {m: 10.0, c: -5.0, px: centerx + x, py: centery + y + radius, vx: 9.0 - rng.gen::<f32>()/4.0, vy: -3.0, ax: 0.0, ay: 0.0, fx: 0.0, fy: 0.0, };
        v.push(node);
    }
}


// computing forces, velocities, positions
fn update_nodes_vec(v: &mut Vec<Node>, dt: f32) {
    let vec_a = Arc::new(v.to_vec());
    //let vec_a = v.to_vec();
    let mut threadsv = Vec::with_capacity(v.len());
        
    for i in 0..v.len() {
        let n_c = (&v[i]).clone();
        let vec_ac = vec_a.clone();
        
        let child = thread::spawn(move || {            
            //println!("T{}...", i);
            
            // delay
            //if i == 1 {thread::sleep(time::Duration::from_millis(100))};
        
            let mut fv = (0.0, 0.0); 
        
            //for j in 0..vec_ac.len() {  // 13% core::iter::range iterator 
            //for m in vec_ac.iter().by_ref() {  // 40% core slice iter
            for m in vec_ac.iter() {  // FIXME: 40% core slice iter - main bottleneck 
            
                //if i == j { continue; }
                //assert!(i != j);
            
                let n = &n_c;       // from node
                if n.px == m.px && n.py == m.py { continue; }
                //let m = &vec_ac[j]; // to node //6% alloc..arc..Arc; 16% collections..vec..Vec core ops index
                
                let dthr = n.m;                
            
                let dnm  = (m.px - n.px, m.py - n.py);                  // distance vector
                //let d    = ((dnm.0).powi(2) + (dnm.1).powi(2) ).sqrt(); // distance scalar // 14% powi
                let mut d    = (dnm.0*dnm.0 + dnm.1*dnm.1).sqrt(); // distance scalar
                if d < dthr {d = dthr;}
                let dirv = (dnm.0/d, dnm.1/d);                          // direction vector
                
                //let fg   = 10.0*n.m*m.m/(d.powi(2));     // gravity force scalar  // 7% powi

                let fg = 10.0*n.m*m.m/(d*d);     // gravity force scalar
                let fgnm = (fg*dirv.0, fg*dirv.1); // gravity force vector
                
                //let fc   = -10.0*n.c*m.c/(d.powi(2));    // coulomb force scalar  // 7% powi
                let fc   = -10.0*n.c*m.c/(d*d);    // coulomb force scalar
                let fcnm = (fc*dirv.0, fc*dirv.1); // coulomb force vector
                
                fv.0 += fgnm.0 + fcnm.0;  // result force vector - x
                fv.1 += fgnm.1 + fcnm.1;  // result force vector - y
            }
            
            //println!("T{} done", i);
        
            (fv.0, fv.1) // force returned
        });
        
        // push thread to vector
        &threadsv.push(child);
    }
    
    let th_ret: Vec<(f32, f32)> = threadsv.into_iter().map(|t| t.join().unwrap()).collect();
    //println!(" >: {:?}", th_ret);
    
    for i in 0..v.len() {
        let mut n = &mut v[i];
        let  fv   = &th_ret[i];
        n.fx = fv.0;
        n.fy = fv.1;
        
        let av = (fv.0/n.m, fv.1/n.m);
        n.ax = av.0;
        n.ay = av.1;
        
        let kv = 1.0 - 0.001*dt;  // drag
        //let kv = 1.0;  // drag
        
        let mut vv = (n.vx + av.0*dt, n.vy + av.1*dt);
        vv.0 *= kv;
        vv.1 *= kv;
        n.vx = vv.0;
        n.vy = vv.1;
        
        let pv = (n.px + vv.0*dt, n.py + vv.1*dt);
        n.px = pv.0;
        n.py = pv.1;
    }
    
}


fn main() {
    let screen_shape: Vec<u32> = vec![640, 480];  
    let tex_res: u32 = 1;  
    
    let n = 2048;
    let mut vecnodes: Vec<Node> = Vec::new();

    let mut run = true;
    
    let mut nframes: u64 = 0;
    
    
    let mut rng = rand::thread_rng();
   
    let sdl_ctx = sdl2::init().unwrap();
    let sdl_ctx_vid = sdl_ctx.video().unwrap();
    let gl_attr = sdl_ctx_vid.gl_attr();

    // window object
    let win = sdl_ctx_vid.window(&"Rust on SDL2", screen_shape[0], screen_shape[1])
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // renderer object
    let mut rnd = win.renderer().build().unwrap();
    
    // Enable anti-aliasing
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    // orange texture
    let mut texturerg = rnd.create_texture_streaming(PixelFormatEnum::RGB24, tex_res, tex_res).unwrap();
    texturerg.with_lock(None, |buffer: &mut [u8], p: usize| {
        for y in 0..tex_res {
            for x in 0..tex_res {
                let t: usize = (y*p as u32 + x*3) as usize;
                buffer[t + 0] = 255;
                buffer[t + 1] = 128;
                buffer[t + 2] = 50;
            }
        }
    }).unwrap();
    
    // blue texture
    let mut texturegb = rnd.create_texture_streaming(PixelFormatEnum::RGB24, tex_res, tex_res).unwrap();
    texturegb.with_lock(None, |buffer: &mut [u8], p: usize| {
        for y in 0..tex_res {
            for x in 0..tex_res {
                let t: usize = (y*p as u32 + x*3) as usize;
                buffer[t + 0] = 50;
                buffer[t + 1] = 128;
                buffer[t + 2] = 255;
            }
        }
    }).unwrap();
    
    // generate nodes
    //init_nodes_vec(&mut vecnodes, n as u32 /2);
      
    // main loop
    while run {
        rnd.clear(); // clearing window
        
        // emiting new particles
        let vnum = vecnodes.len();
        if vnum < n {
            let em0 = ( (screen_shape[0]/2 - 200) as f32 + rng.gen::<f32>(), (screen_shape[1]/2) as f32 + rng.gen::<f32>() );
            let em1 = ( (screen_shape[0]/2 + 200) as f32 + rng.gen::<f32>(), (screen_shape[1]/2) as f32 + rng.gen::<f32>() );
        
            if nframes % 1 == 0 {
                emit_node(&mut vecnodes, em0.0, em0.1,  
                    10.0, 10.0, 
                    10.0, -5.0);
                emit_node(&mut vecnodes, em1.0, em1.1,  
                    -10.0, -10.0, 
                    10.0, 5.0);
            }
        }
        
        // drawing particles
        for n in &vecnodes {
            let r = (1.0+(n.m - 10.0)/10.0) as u32;
            let x = n.px as i32;
            let y = n.py as i32;
            let tex = {
                if n.c >= 0.0 { &texturerg } 
                else { &texturegb }
            };
            
            match rnd.copy(tex, None, Some(Rect::new(x, y, r, r) ) ) {
                Result::Ok(val) => val, 
                Result::Err(err) => panic!("rnd.copy() not ok!: {:?}", err),
            }
        }

        rnd.present(); // rendering
    
        // handling events
        for event in sdl_ctx.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { run = false },
//                Event::KeyDown { keycode: Some(Keycode::W), .. } => { vecpos2d[1] -= 10 },
//                Event::KeyDown { keycode: Some(Keycode::S), .. } => { vecpos2d[1] += 10 },
//                Event::KeyDown { keycode: Some(Keycode::A), .. } => { vecpos2d[0] -= 10 },
//                Event::KeyDown { keycode: Some(Keycode::D), .. } => { vecpos2d[0] += 10 },
                _ => {}
            }
        }
        
        // updating nodes forces, accel, vel, positions
        update_nodes_vec(&mut vecnodes, 0.025);
        
        // updating frame counter
        nframes += 1;
    }
}


