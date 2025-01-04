use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Update, hello_vatideck)
        .run();
}

fn hello_vatideck() {
    println!("init Vatideck");
}