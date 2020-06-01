use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::texture_manager::TextureManager;
use crate::vec2::Vec2;

/// Internal struct used to represent one frame which can be used
/// in a sprite's animation data
#[derive(Clone, Deserialize)]
struct SpriteFrame {
    x: i32,
    y: i32,
    width: u32,
    height: u32
}

impl From<&SpriteFrame> for Rect {
    fn from(frame: &SpriteFrame) -> Self {
        Rect::new(frame.x, frame.y, frame.width, frame.height)
    }
}

/// Internal struct used to store the sprite's animation data
#[derive(Clone, Deserialize)]
struct SpriteInfo {
    texture_filename: PathBuf,
    frames: Vec<SpriteFrame>,
    // TODO: strings probably aren't the best solution here
    animations: HashMap<String, Vec<usize>>,
    default_animation: String
}

impl SpriteInfo {
    /// Load a SpriteInfo struct from the given file
    fn from_file(path: impl AsRef<Path>) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    }

    fn get_animation(&self, anim: &str) -> &Vec<usize> {
        self.animations.get(anim).unwrap()
    }

    /// Given an animation name and a frame counter value, return the frame
    /// that this sprite should display
    fn frame_from_counter(&self, anim: &str, frame_counter: usize) -> usize {
        self.animations.get(anim).unwrap()[frame_counter]
    }
}

/// A graphic which has a position and may have animations
#[derive(Clone)]
pub struct Sprite {
    pos: Vec2<i32>,
    current_anim: String,
    frame_counter: usize,
    timer: f64,
    sprite_info: SpriteInfo
}

impl Sprite {
    pub fn new(sprite_info_path: &Path) -> Self {
        let sprite_info = SpriteInfo::from_file(sprite_info_path);
        let current_anim = sprite_info.default_animation.clone();
        Self {
            pos: Vec2::default(),
            frame_counter: 0,
            current_anim,
            timer: 0.0,
            sprite_info
        }
    }

    /// Get a reference to this sprite's position
    pub fn pos(&self) -> &Vec2<i32> {
        &self.pos
    }

    pub fn mut_pos(&mut self) -> &mut Vec2<i32> {
        &mut self.pos
    }

    /// Convenience function for
    /// self.[mut_pos()](#method.mut_pos).[set(x, y)](../vec2/struct.Vec2.html#method.translate)
    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.pos.set(x, y);
    }

    pub fn set_animation(&mut self, anim: String) {
        self.frame_counter = 0;
        self.current_anim = anim;
    }

    pub fn update(&mut self, delta_time: f64) {
        self.timer += delta_time;
        let current_anim_len = self.sprite_info.get_animation(&self.current_anim).len();
        while self.timer > 1.0 / 25.0 {
            self.frame_counter = (self.frame_counter + 1) % current_anim_len;
            self.timer -= 1.0 / 25.0;
        }
    }

    pub fn draw<T>(&self, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager<T>) {
        self.draw_with_offset((0, 0).into(), canvas, texture_manager);
    }

    pub fn draw_with_offset<T>(
        &self, offset: Vec2<i32>, canvas: &mut WindowCanvas, texture_manager: &mut TextureManager<T>
    ) {
        let current_frame_num = self.sprite_info.frame_from_counter(&self.current_anim, self.frame_counter);
        let current_frame = &self.sprite_info.frames[current_frame_num];
        let src_rect = Rect::from(current_frame);
        let dest_rect = Rect::new(self.pos.x + offset.x, self.pos.y + offset.y, current_frame.width, current_frame.height);
        let texture = texture_manager.get_texture::<T>(&self.sprite_info.texture_filename);
        canvas.copy(&texture, src_rect, dest_rect).unwrap();
    }
}
