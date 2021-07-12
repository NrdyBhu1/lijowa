use macroquad::prelude::*;
use crate::face::*;

/// Player struct
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Player {
    face: Face,
    rect: Rect,
    vel: Vec2,
    speed: f32,
}

/// implmentation for player struct
#[allow(dead_code)]
impl Player {
    /// Creates a new player
    pub async fn new() -> Result<Player, &'static str> {
        Ok(Self {
            face: Face::Confused,
            rect: Rect::new(1.0, 1.0, 64.0, 64.0),
            vel: Vec2::new(0.0, 0.0),
            speed: 20.0
        })
    }

    /// renders the player
    pub fn render(self, faces: &[Texture2D; 4]) -> Result<(), &'static str> {
        draw_texture(faces[self.face.get_face_index()], self.rect.x, self.rect.y, WHITE);
        Ok(())
    }

    /// update function for player
    pub fn update(&mut self, _tiles: &Vec<Rect>) -> Result<(), &'static str> {
        // pos actual man
        self.rect.x += self.vel.x * self.speed;
        self.rect.y += self.vel.y * self.speed;

        // face man
        if is_key_pressed(KeyCode::Key1) {
            self.face.smile();
        } else if is_key_pressed(KeyCode::Key2) {
            self.face.annoy();
        } else if is_key_pressed(KeyCode::Key3) {
            self.face.angry();
        } else if is_key_pressed(KeyCode::Key4) {
            self.face.confuse();
        }

        // pos man
        if is_key_down(KeyCode::W) {
            self.vel.y = -1.0;
        }
        if is_key_down(KeyCode::S) {
            self.vel.y = 1.0;
        }
        if is_key_down(KeyCode::D) {
            self.vel.x = 1.0;
        }
        if is_key_down(KeyCode::A) {
            self.vel.x = -1.0;
        }

        if is_key_released(KeyCode::W) || is_key_released(KeyCode::S) {
            self.vel.y = 0.0;
        }
        if is_key_released(KeyCode::A) || is_key_released(KeyCode::D) {
            self.vel.x = 0.0;
        }
            
        Ok(())
    }
}
