use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, greet).run();
}

fn greet() {
    println!("hello");
}
