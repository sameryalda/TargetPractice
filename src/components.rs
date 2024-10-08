use macroquad::prelude::*;
//draw button on screen. returns true if clicked.
pub fn draw_button(x: f32, y: f32, width: f32, height: f32, label: &str) -> bool {
    // Draw the button
    draw_rectangle(x, y, width, height, GRAY);

    // Draw the button's label in the center
    let text_dimensions = measure_text(label, None, 30, 1.0);
    draw_text(
        label,
        x + (width - text_dimensions.width) / 2.0,
        y + (height + text_dimensions.height) / 2.0,
        30.0,
        WHITE,
    );

    // Check if the mouse is inside the button and if it's clicked
    let mouse_pos = mouse_position();
    let (mouse_x, mouse_y) = mouse_pos;

    let is_hovered = mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height;

    if is_hovered && is_mouse_button_released(MouseButton::Left) {
        return true;
    }

    false
}

//struct and implementation of a target
pub struct Target {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub time_alive: f32,
    pub texture: Option<Texture2D>,
}

impl Target {
    pub fn draw(&self) {
        if let Some(texture) = &self.texture {
            draw_texture_ex(
                texture,
                self.x - self.radius,
                self.y - self.radius,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(self.radius * 2.0, self.radius * 2.0)),
                    ..Default::default()
                },
            );
        } else {
            draw_circle(self.x, self.y, self.radius, RED);
        }
    }

    pub fn is_clicked(&self, mouse_x: f32, mouse_y: f32) -> bool {
        let dx = self.x - mouse_x;
        let dy = self.y - mouse_y;
        (dx * dx + dy * dy) < self.radius * self.radius
    }

    pub fn randomize_position(&mut self, screen_width: f32, screen_height: f32) {
        self.radius = rand::gen_range(30.0, 35.0);
        self.x = rand::gen_range(self.radius, screen_width - self.radius);
        self.y = rand::gen_range(self.radius, screen_height - self.radius);
        self.time_alive = 0.0;
    }
}

//draws text in center of screen
pub fn draw_centered_text(text: &str, y: f32, font_size: u16, color: Color, screen_width: f32) {
    let text_dimensions = measure_text(text, None, font_size, 1.0);
    let x = (screen_width - text_dimensions.width) / 2.0;
    draw_text(text, x, y, font_size as f32, color);
}

pub fn calc_accuracy(score: i32, click_count: i32) -> f32 {
       (score as f32 / click_count as f32) * 100.0
}

//restarts game
pub async fn restart_game(texture: &Texture2D) -> (i32, i32, i32, Target) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    (
        0, // score/count
        3, // miss
        0, // click_count
        Target {
            x: screen_width / 2.0,
            y: screen_height / 2.0,
            radius: 30.0,
            time_alive: 0.0,
            texture: Some(texture.clone()),
        }
    )
}