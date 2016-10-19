use std::thread;
use std::sync::{Arc, Mutex};

fn thfun(i: i32, av: Arc<Vec<i32>>) {
    println!("    some child play {}, {}", i, av[i as usize]);
}

fn main() {
    println!("main start");
    
    let N: i32 = 10;
    
    //let constvec: Vec<i32> = vec![100, 101, 102, 103, 104, 105, 106, 107, 108, 109];
    let constvec: Vec<i32> = (100..100+N).collect();
    let arc_constvec = Arc::new(constvec);
    
    let mut mutvec: Vec<i32> = vec![-1; N as usize];

    // vector for threads
    let mut children = vec![];

    // spawning threads
    for x in 0..N {
        let acc = arc_constvec.clone();
        
        // new thread
        let child = thread::spawn(move || {
            // child thread work
            thfun(x, acc);
        });
        
        // push thread to vector
        children.push(child);
    }
    
    println!("some main play");
    
    // wait for threads to exit
    for child in children {
        let _ = child.join();
    }
    
    println!("some main play after child exit");
}
