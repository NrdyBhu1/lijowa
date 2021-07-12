use macroquad::prelude::*;
mod face;
mod player;
use crate::player::*;

/// window config
/// TODO: Add More features
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Lijowa".to_owned(),
        // fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

/// main entry point
#[macroquad::main(window_conf)]
async fn main() {
    // variables
    let smile: Texture2D = load_texture("./assets/smile.png").await.unwrap();
    let annoyed: Texture2D = load_texture("./assets/annoyed.png").await.unwrap();
    let angry: Texture2D = load_texture("./assets/anrgy.png").await.unwrap();
    let confused: Texture2D = load_texture("./assets/confused.png").await.unwrap();
    let mut player: Player = Player::new().await.unwrap();
    let faces: [Texture2D; 4] = [smile, annoyed, angry, confused];

    // main loop
    loop {
        player.update(&vec![]).unwrap();
        clear_background(BLACK);
        player.render(&faces).unwrap();
        next_frame().await
    }
}

