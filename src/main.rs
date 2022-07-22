use bevy::prelude::*;

fn main() {
    App::new().add_system(test_system).run();
}

fn test_system() {
    println!("Testing (: >_<");
}
