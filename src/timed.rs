use macroquad::prelude::*;
use crate::components::*;


enum GameState {
    Starting,
    Playing,
    PostGame,
}
pub async fn timed_game_loop() {

    let screen_width = screen_width();
    let screen_height = screen_height();

    let texture: Texture2D = load_texture("assets/target.png").await.unwrap(); // Load texture once

    let mut game_state = GameState::Starting;
    let mut score: i32 = 0;
    let mut count: i32 = 30;
    let mut click_count: i32 = 0;
    let mut click_time_total: f32 = 0.0;
    let mut total_time: f32 = 0.0;

    let mut target = {
        let (_, _, _, tgt) = restart_game(&texture).await;
        tgt
    };

    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Starting => {
                draw_centered_text("Left click to start", screen_height / 2.0, 50, WHITE, screen_width);
                if is_mouse_button_pressed(MouseButton::Left) {
                    game_state = GameState::Playing;
                }
            }

            GameState::Playing => {
                draw_centered_text(&format!("Remaining: {}", count), screen_height - 20.0, 30, WHITE, screen_width);
                draw_centered_text(&format!("Time: {:.2}", total_time), screen_height - 40.0, 30, WHITE, screen_width);

                target.time_alive += get_frame_time();
                total_time += get_frame_time();


                if is_mouse_button_pressed(MouseButton::Left) {
                    click_count += 1;
                    let (mouse_x, mouse_y) = mouse_position();

                    if target.is_clicked(mouse_x, mouse_y) {
                        count -= 1;
                        score += 1;
                        click_time_total += target.time_alive;
                        target.randomize_position(screen_width, screen_height);
                    }
                }
                target.draw();

                if count == 0 || is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::PostGame;
                }
            }

            GameState::PostGame => {
                let average_time = if click_count > 0 {
                    click_time_total / click_count as f32
                } else {
                    0.0
                };
                draw_centered_text(&format!("Accuracy: {:.2}%", if click_count > 0 {
                    calc_accuracy(score, click_count)
                } else {
                    0.0
                }), screen_height / 2.0 - 20.0, 30, WHITE, screen_width);
                draw_centered_text(&format!("Average time until target clicked: {:.3}", average_time), screen_height / 2.0 + 10.0, 30, WHITE, screen_width);
                draw_centered_text(&format!("Total time: {:.1}", total_time), screen_height / 2.0 + 40.0, 30, WHITE, screen_width);

                let restart_button_clicked = draw_button(
                    screen_width / 2.0 - 200.0,
                    screen_height - 100.0,
                    400.0,
                    50.0,
                    "Restart",
                );
                if restart_button_clicked {
                    let (new_score, _new_miss, new_click_count, new_target) = restart_game(&texture).await;
                    count = 30;
                    score = new_score;
                    click_count = new_click_count;
                    target = new_target;
                    total_time = 0.0;
                    game_state = GameState::Starting;
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }

            }
        }
        next_frame().await;
    }
}