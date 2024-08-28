use macroquad::prelude::*;

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

struct Target {
    x: f32,
    y: f32,
    radius: f32,
    time_alive: f32,
    texture: Option<Texture2D>,
}

enum GameState {
    Starting,
    Playing,
    PostGame,
}

impl Target {
    fn draw(&self) {
        if let Some(texture) = &self.texture {
            // Scale the texture to match the target's radius
            draw_texture_ex(
                &texture,
                self.x - self.radius,
                self.y - self.radius,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(self.radius * 2.0, self.radius * 2.0)),
                    ..Default::default()
                },
            );
        } else {
            // Fallback to drawing a circle if the texture is not available
            draw_circle(self.x, self.y, self.radius, RED);
        }
    }

    fn is_clicked(&self, mouse_x: f32, mouse_y: f32) -> bool {
        let dx = self.x - mouse_x;
        let dy = self.y - mouse_y;
        (dx * dx + dy * dy) < self.radius * self.radius
    }

    fn randomize_position(&mut self, screen_width: f32, screen_height: f32) {
        self.radius = rand::gen_range(30.0, 35.0);
        self.x = rand::gen_range(self.radius, screen_width - self.radius);
        self.y = rand::gen_range(self.radius, screen_height - self.radius);
        self.time_alive = 0.0;
    }
}

fn draw_centered_text(text: &str, y: f32, font_size: u16, color: Color, screen_width: f32) {
    let text_dimensions = measure_text(text, None, font_size, 1.0);
    let x = (screen_width - text_dimensions.width) / 2.0;
    draw_text(text, x, y, font_size as f32, color);
}

async fn restart_game() -> (i32, i32, i32, Target) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    let texture = match load_texture("assets/target.png").await {
        Ok(tex) => {
            println!("Texture loaded successfully");
            Some(tex)
        }
        Err(e) => {
            println!("Failed to load texture: {:?}", e);
            None
        }
    };

    (
        0, // score
        3, // miss
        0, // click_count
        Target {
            x: screen_width / 2.0,
            y: screen_height / 2.0,
            radius: 30.0,
            time_alive: 0.0,
            texture,
        }
    )
}

#[macroquad::main(conf)]
async fn main() {
    let screen_width = screen_width();
    let screen_height = screen_height();

    let mut game_state = GameState::Starting;
    let mut score: i32 = 0;
    let mut miss: i32 = 3;
    let mut click_count: i32 = 0;
    let mut click_time_total: f32 = 0.0;

    let mut target = {
        let (_, _, _, tgt) = restart_game().await;
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
                draw_centered_text(&format!("Lives: {}", miss), screen_height - 20.0, 30, WHITE, screen_width);

                target.time_alive += get_frame_time(); // Increment the timer

                // If the target has been alive for more than 0.75 seconds, it expires
                if target.time_alive > 0.75 {
                    miss -= 1;
                    target.randomize_position(screen_width, screen_height);
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    click_count += 1;
                    let (mouse_x, mouse_y) = mouse_position();

                    if target.is_clicked(mouse_x, mouse_y) {
                        score += 1;
                        click_time_total += target.time_alive;
                        target.randomize_position(screen_width, screen_height);
                    }
                }
                target.draw();

                if miss == 0 || is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::PostGame;
                }
            }

            GameState::PostGame => {
                let average_time = if click_count > 0 {
                    click_time_total / click_count as f32
                } else {
                    0.0
                };
                draw_centered_text(&format!("Score: {}", score), screen_height / 2.0 - 50.0, 30, WHITE, screen_width);
                draw_centered_text(&format!("Accuracy: {:.2}%", if click_count > 0 {
                    (score as f32 / click_count as f32) * 100.0
                } else {
                    0.0
                }), screen_height / 2.0 - 20.0, 30, WHITE, screen_width);
                draw_centered_text(&format!("Average time until target clicked: {:.3}", average_time), screen_height / 2.0 + 10.0, 30, WHITE, screen_width);
                draw_centered_text("Left click to play again", screen_height - 20.0, 30, WHITE, screen_width);

                if is_mouse_button_pressed(MouseButton::Left){
                    let (new_score, new_miss, new_click_count, new_target) = restart_game().await;
                    score = new_score;
                    miss = new_miss;
                    click_count = new_click_count;
                    target = new_target;
                    game_state = GameState::Playing;
                } else if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }
        next_frame().await;
    }
}
