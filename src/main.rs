use macroquad::prelude::*;
use crate::title_screen::start;

mod title_screen;
mod components;
mod infinite;
mod timed;

fn conf() -> Conf
{
    Conf {
        window_title: String::from("Target Challenge Infinite"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // You can now use functions or structs from the modules
    start().await;
}