use std::process;
use std::{thread, time};
use world::{World, WorldConfig};

fn main() {
    let config = WorldConfig::from_file("test.wrld").unwrap_or_else(|e| {
        println!("Unable to read world seed: {}", e);
        process::exit(1);
    });

    let mut world = World::new(config);

    for _ in 0..29 {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("{}", world);
        thread::sleep(time::Duration::from_millis(500));
        world.tick();
    }

    world.to_file("test.wrld").unwrap();
}
