extern crate magnetic;
extern crate rand;

use std::thread;

mod messaging;
mod game_state;
mod identity;
mod resources;

enum GameMode {
    ServerMode, ClientMode, SingleMode,
}

fn main() {
    let mode = GameMode::ClientMode; //TODO
    println!("Hello, world!");
    match mode {
        GameMode::ClientMode => {
            thread::spawn(move || {
                //begin client network thread
            });
            //begin client engine
        },
        GameMode::ServerMode => {
            thread::spawn(move || {
                //begin server network thread
            });
            //begin server engine
        },
        GameMode::SingleMode => {
            thread::spawn(move || {
                //begin client engine
                //begin server engine
            });
        },
    }
}
