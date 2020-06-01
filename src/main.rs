pub mod panel;
pub mod playfield;
pub mod sprite;
pub mod texture_manager;
pub mod vec2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::time::{Duration, Instant};

use crate::playfield::Playfield;
use crate::sprite::Sprite;
use crate::texture_manager::TextureManager;

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 224;
const WINDOW_SCALE: f32 = 4.0;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = (SCREEN_WIDTH as f32 * WINDOW_SCALE) as u32;
    let window_height = (SCREEN_HEIGHT as f32 * WINDOW_SCALE) as u32;
    let window = video_subsystem.window("OpenPon", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();
    canvas.set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_time = Instant::now();
    let mut delta_time = Duration::from_secs_f32(1.0 / 60.0);

    let mut texture_manager = TextureManager::new(&texture_creator);
    let mut field_sprite = Sprite::new(Path::new("res/sprites/onePlayerVsField.json"));
    field_sprite.set_pos(4, 20);
    let mut wall_sprite = field_sprite.clone();
    wall_sprite.set_animation("wallP1".to_string());
    wall_sprite.set_pos(8, 214);
    let mut playfield = Playfield::new(8, 214);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.clear();
        playfield.update(delta_time.as_secs_f64());
        playfield.draw(&mut canvas, &mut texture_manager);
        field_sprite.draw(&mut canvas, &mut texture_manager);
        wall_sprite.draw(&mut canvas, &mut texture_manager);
        canvas.present();
        delta_time = last_time.elapsed();
        last_time = Instant::now();
    }
}
