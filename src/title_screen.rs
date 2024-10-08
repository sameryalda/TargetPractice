use macroquad::prelude::*;
use crate::components::draw_button;
use crate::infinite::infinite_game_loop;
use crate::timed::timed_game_loop;

pub async fn start() {
    title_screen().await;
}

async fn title_screen() {
    loop {
        clear_background(BLACK);

        let screen_width = screen_width();
        let screen_height = screen_height();

        // Draw title


        // Draw buttons
        let infinite_button_clicked = draw_button(
            screen_width / 2.0 - 200.0,
            screen_height / 2.0 - 120.0,
            400.0,
            50.0,
            "Infinite Target Practice",
        );

        let timed_button_clicked = draw_button(
            screen_width / 2.0 - 200.0,
            screen_height / 2.0 - 50.0,
            400.0,
            50.0,
            "Timed Target Practice",
        );

        let exit_button_clicked = draw_button(
            screen_width / 2.0 - 200.0,

            screen_height / 2.0 + 20.0,
            400.0,
            50.0,
            "Exit",
        );

        // Button actions
        if infinite_button_clicked {
            infinite_game_loop().await;
        }
        else if timed_button_clicked {
            timed_game_loop().await;
        }

        if exit_button_clicked {
            break;
        }

        next_frame().await;
    }
}


